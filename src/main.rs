use nalgebra::Vector2;

mod maze;
fn main() {
    let mut maze = maze::new().rows(10).cols(5).call();
    let start = Vector2::new(0, 0); // Top-left
    let end = Vector2::new(maze.nrows() - 1, maze.ncols() - 1); // Bottom-right
    maze::generate(&mut maze, start, start, end);
}
