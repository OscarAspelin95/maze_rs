use crate::utils::WilsonMaze;
use std::collections::HashSet;

pub fn backtrack(
    maze: &WilsonMaze,
    start: (usize, usize),
    end_cell: (usize, usize),
    path: &mut Vec<(usize, usize)>,
    solution: &mut Vec<(usize, usize)>,
    visited: &mut HashSet<(usize, usize)>,
    visited_to_return: &mut HashSet<(usize, usize)>,
) {
    if start == end_cell {
        path.push(end_cell);
        solution.extend(path.iter().cloned());
        visited_to_return.extend(visited.iter());
        return;
    }

    visited.insert(start);
    path.push(start);

    for (direction, (nrow, ncol)) in maze.neighbors(start.0 as i32, start.1 as i32) {
        if !visited.contains(&(nrow, ncol)) && maze.grid[start.0][start.1].contains(&direction) {
            // Do actual backtracking
            backtrack(
                maze,
                (nrow, ncol),
                end_cell,
                path,
                solution,
                visited,
                visited_to_return,
            );

            path.pop();
        }
    }
}

pub fn get_backtrack_solution(
    maze: &WilsonMaze,
) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>) {
    let mut solution: Vec<(usize, usize)> = vec![];
    let mut path: Vec<(usize, usize)> = vec![];
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    let mut visited_to_return: HashSet<(usize, usize)> = HashSet::new();

    backtrack(
        &maze,
        maze.start_cell,
        maze.end_cell,
        &mut path,
        &mut solution,
        &mut visited,
        &mut visited_to_return,
    );

    let solution_as_hashset: HashSet<(usize, usize)> = solution.into_iter().collect();

    return (solution_as_hashset, visited_to_return);
}
