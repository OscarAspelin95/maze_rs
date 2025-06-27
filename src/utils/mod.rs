mod utils;
pub use utils::{Direction, WilsonMaze};

mod bfs_solver;
pub use bfs_solver::{get_bfs_solution, Priority};

mod dfs_solver;
pub use dfs_solver::get_backtrack_solution;
