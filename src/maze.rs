use bon::builder;
use nalgebra::{DMatrix, Vector2};
use rand::prelude::*;
use std::collections::{HashMap, VecDeque};
use std::{collections::HashSet, thread, time};
use strum::{AsRefStr, EnumIter, IntoEnumIterator};

const ROW_UP: Vector2<isize> = Vector2::new(-1, 0);
const ROW_DOWN: Vector2<isize> = Vector2::new(1, 0);
const COL_LEFT: Vector2<isize> = Vector2::new(0, -1);
const COL_RIGHT: Vector2<isize> = Vector2::new(0, 1);

const PATH_COLOR: &str = "\x1b[33m"; // Yellow foreground
const RESET_COLOR: &str = "\x1b[0m";

#[builder]
pub fn new(rows: usize, cols: usize) -> DMatrix<Cell> {
    DMatrix::from_element(rows, cols, Cell::default())
}

#[derive(Clone, Copy, AsRefStr, EnumIter)]
#[repr(u8)]
pub enum Direction {
    N = 1,
    S = 2,
    E = 4,
    W = 8,
}

impl Direction {
    pub fn offset(self) -> Vector2<isize> {
        match self {
            Direction::N => ROW_UP,
            Direction::S => ROW_DOWN,
            Direction::E => COL_RIGHT,
            Direction::W => COL_LEFT,
        }
    }
    pub fn opposite(self) -> Self {
        match self {
            Self::W => Self::E,
            Self::E => Self::W,
            Self::N => Self::S,
            Self::S => Self::N,
        }
    }
}

#[derive(Clone, Copy, AsRefStr, PartialEq, Debug)]
pub enum CellStatus {
    Visited,
    NotVisited,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub status: CellStatus,
    pub walls: u8,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            status: CellStatus::NotVisited,
            walls: 0,
        }
    }
}

fn carve_path(grid: &mut DMatrix<Cell>, current: Vector2<usize>, next: Vector2<usize>, dir: Direction) {
    grid[(current.x, current.y)].walls |= dir as u8;
    grid[(next.x, next.y)].walls |= dir.opposite() as u8;
}

fn in_boundaries(offset: Vector2<isize>, grid: &DMatrix<Cell>) -> bool {
    offset.x >= 0 && offset.y >= 0 && offset.x < grid.nrows() as isize && offset.y < grid.ncols() as isize
}

pub fn generate(
    grid: &mut DMatrix<Cell>,
    current: Vector2<usize>,
    start: Vector2<usize>,
    end: Vector2<usize>,
) {
    grid[(current.x, current.y)].status = CellStatus::Visited;

    let mut rng = rand::thread_rng();
    let mut directions = Direction::iter().collect::<Vec<_>>();
    directions.shuffle(&mut rng);

    for dir in directions {
        let next_isize = current.map(|v| v as isize) + dir.offset();

        if in_boundaries(next_isize, grid) {
            let next = next_isize.map(|v| v as usize);

            if grid[(next.x, next.y)].status == CellStatus::NotVisited {
                carve_path(grid, current, next, dir);

                // Find path from start to end after carving path
                let path = find_path(grid, start, end);

                // Clear screen and redraw with path visualization
                print!("\x1B[2J\x1B[1;1H");
                draw(grid, start, end, path.as_deref());

                // Small delay for animation effect
                thread::sleep(time::Duration::from_millis(50));

                generate(grid, next, start, end);
            }
        }
    }
}

pub fn find_path(
    grid: &DMatrix<Cell>,
    start: Vector2<usize>,
    end: Vector2<usize>,
) -> Option<Vec<Vector2<usize>>> {
    let mut queue = VecDeque::new();
    let mut came_from: HashMap<Vector2<usize>, Vector2<usize>> = HashMap::new();
    let mut visited = HashSet::new();

    queue.push_back(start);
    visited.insert(start);

    while let Some(current) = queue.pop_front() {
        if current == end {
            let mut path = Vec::new();
            let mut at = current;
            while at != start {
                path.push(at);
                at = came_from[&at];
            }
            path.push(start);
            path.reverse();
            return Some(path);
        }

        let cell = grid[(current.x, current.y)];

        for dir in Direction::iter() {
            if cell.walls & dir as u8 != 0 {
                let neighbor_isize = current.map(|v| v as isize) + dir.offset();
                if in_boundaries(neighbor_isize, grid) {
                    let neighbor = neighbor_isize.map(|v| v as usize);
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        came_from.insert(neighbor, current);
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }

    None // no path found
}

pub fn draw(
    grid: &DMatrix<Cell>,
    start: Vector2<usize>,
    end: Vector2<usize>,
    path: Option<&[Vector2<usize>]>,
) {
    let rows = grid.nrows();
    let cols = grid.ncols();

    let path_set: HashSet<Vector2<usize>> = path.map(|p| p.iter().cloned().collect()).unwrap_or_default();

    // Top border
    print!("+");
    for _ in 0..cols {
        print!("---+");
    }
    println!();

    for row in 0..rows {
        let mut top = String::from("|"); // left outer wall
        let mut bottom = String::from("+");

        for col in 0..cols {
            let pos = Vector2::new(row, col);
            let cell = grid[(row, col)];

            // The actual cell
            let content = if pos == start {
                " S ".to_string()
            } else if pos == end {
                " E ".to_string()
            } else if path_set.contains(&pos) {
                format!("{PATH_COLOR} * {RESET_COLOR}").to_string()
            } else if cell.status == CellStatus::Visited {
                " · ".to_string()
            } else {
                "   ".to_string()
            };
            top.push_str(content.as_str());

            // Right wall or space
            // For the last column, always print right wall '|'
            if cell.walls & Direction::E as u8 == 0 || col == cols - 1 {
                top.push('|');
            } else {
                top.push(' ');
            }

            // Bottom wall or spaces
            if cell.walls & Direction::S as u8 == 0 {
                bottom.push_str("---");
            } else {
                bottom.push_str("   ");
            }
            bottom.push('+');
        }

        println!("{top}");
        println!("{bottom}");
    }
}
