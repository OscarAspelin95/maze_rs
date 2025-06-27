use priority_queue::PriorityQueue;
use std::collections::{HashMap, HashSet};

use crate::utils::WilsonMaze;

pub enum Priority {
    // Normal BFS.
    Disabled,
    // Prioritize cells closer to start cell (very non ideal).
    Close,
    // Prioritize cells that are further away from start point and closer to end point.
    Prio,
    // Given each neighbor a random priority.
    Random,
}

#[inline]
pub fn abs_dist(x: usize, y: usize) -> usize {
    return std::cmp::max(x, y) - std::cmp::min(x, y);
}

/// We calculate a priority based on considering the distance
/// from start cell (lower priority) and end cell (higher priority).
#[inline]
pub fn weighted_priority(
    cell: (usize, usize),
    start_cell: (usize, usize),
    end_cell: (usize, usize),
    max_dist: usize,
) -> usize {
    let dx_end = abs_dist(cell.0, end_cell.0);
    let dy_end = abs_dist(cell.1, end_cell.1);

    let dx_start = abs_dist(cell.0, start_cell.0);
    let dy_start = abs_dist(cell.1, start_cell.1);

    return max_dist + (dx_start + dy_start) - (dx_end + dy_end);
}

/// Cells closer to the end cell will get lower priority (very non-ideal).
#[inline]
pub fn close_priority(cell: (usize, usize), end_cell: (usize, usize)) -> usize {
    let dx = abs_dist(end_cell.0, cell.0);
    let dy = abs_dist(end_cell.1, cell.1);

    return dx + dy;
}

pub fn bfs_solve(
    maze: &WilsonMaze,
    priority: Priority,
) -> (
    HashMap<(usize, usize), (usize, usize)>,
    HashSet<(usize, usize)>,
) {
    let end_cell = maze.end_cell;
    let start_cell = maze.start_cell;

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut path: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut queue: PriorityQueue<(usize, usize), usize> = PriorityQueue::new();
    queue.push(start_cell, 1);

    while queue.len() > 0 {
        // Do something with this later on.
        let (current, _) = queue.pop().unwrap();
        visited.insert(current);

        // We have reached the end cell.
        if current == end_cell {
            return (path, visited);
        }

        for (direction, (nrow, ncol)) in maze.neighbors(current.0 as i32, current.1 as i32) {
            // Otherwise, we need to check if
            if !visited.contains(&(nrow, ncol))
                && maze.grid[current.0][current.1].contains(&direction)
            {
                path.insert((nrow, ncol), current);

                let priority = match priority {
                    Priority::Prio => weighted_priority(
                        (nrow, ncol),
                        maze.start_cell,
                        maze.end_cell,
                        maze.max_dist(),
                    ),
                    Priority::Close => close_priority((nrow, ncol), maze.end_cell),
                    Priority::Disabled => 1,
                    Priority::Random => rand::random_range(1..10),
                };
                queue.push((nrow, ncol), priority);
            }
        }
    }

    panic!("No solution exists.");
}

pub fn get_bfs_solution(
    maze: &WilsonMaze,
    priority: Priority,
) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>) {
    let (bfs_path, visited): (
        HashMap<(usize, usize), (usize, usize)>,
        HashSet<(usize, usize)>,
    ) = bfs_solve(&maze, priority);

    let mut s = maze.end_cell;
    let start = maze.start_cell;

    let mut path: HashSet<(usize, usize)> = HashSet::new();
    path.insert(s);

    let mut num = 0;

    while num < bfs_path.len() {
        s = *bfs_path.get(&s).unwrap();

        if s == start {
            path.insert(s);
            return (path, visited);
        }

        path.insert(s);
        num += 1;
    }

    panic!("No solution exists");
}
