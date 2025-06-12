use core::panic;
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use priority_queue::PriorityQueue;

use rand::prelude::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum Priority {
    // Normal BFS.
    Disabled,
    // Prioritize cells closer to start cell (very non ideal).
    Close,
    // Prioritize cells that are further away from start point and closer to end point.
    Prio,
}

impl Direction {
    /// We can implement this with vec::with_capacity() later on.
    pub fn as_list() -> Vec<Direction> {
        return vec![
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];
    }
}

/// In the grid, we store each cell as its coordinate.
/// e.g., (0, 0) is the cell located as grid[0][0]. The
/// Value grid[0][0] shows what neigbors are available.
pub fn generate_grid(rows: usize, cols: usize) -> Vec<Vec<HashSet<Direction>>> {
    let mut grid: Vec<Vec<HashSet<Direction>>> = vec![];

    for _ in 0..rows {
        let mut grid_vec: Vec<HashSet<Direction>> = vec![];
        for _ in 0..cols {
            grid_vec.push(HashSet::new());
        }
        grid.push(grid_vec);
    }

    return grid;
}

pub fn direction_reverse(direction: &Direction) -> Direction {
    match direction {
        Direction::Up => Direction::Down,
        Direction::Down => Direction::Up,
        Direction::Left => Direction::Right,
        Direction::Right => Direction::Left,
    }
}
// We need a class for generating the maze.
#[derive(Debug, Clone)]
pub struct WilsonMaze {
    pub rows: usize,
    pub cols: usize,
    pub visited: HashSet<(usize, usize)>,
    pub grid: Vec<Vec<HashSet<Direction>>>,
}

impl WilsonMaze {
    pub fn new(rows: usize, cols: usize) -> Self {
        return Self {
            rows: rows,
            cols: cols,
            visited: HashSet::new(),
            grid: generate_grid(rows, cols),
        };
    }

    pub fn direction_lookup(&self, direction: &Direction) -> (i32, i32) {
        match direction {
            Direction::Up => return (-1, 0),
            Direction::Down => return (1, 0),
            Direction::Left => return (0, -1),
            Direction::Right => return (0, 1),
        }
    }

    pub fn in_bounds(&self, row: i32, col: i32) -> bool {
        return (row >= 0 && row < self.rows as i32) && (col >= 0 && col < self.cols as i32);
    }

    pub fn start_cell(&self) -> (usize, usize) {
        return (0, 0);
    }

    pub fn end_cell(&self) -> (usize, usize) {
        return (self.rows - 1, self.cols - 1);
    }

    pub fn neighbors(&self, row: i32, col: i32) -> Vec<(Direction, (usize, usize))> {
        if !self.in_bounds(row, col) {
            panic!(
                "Provided row and col: {}, {} are outside of bounds",
                row, col
            );
        }

        // We have at most 4 neighbors.
        let mut neigbors: Vec<(Direction, (usize, usize))> = Vec::with_capacity(4);

        for direction in Direction::as_list() {
            let (row_offset, col_offset) = self.direction_lookup(&direction);

            let neighbor_row = row_offset + row;
            let neighbor_col = col_offset + col;

            if self.in_bounds(neighbor_row, neighbor_col) {
                neigbors.push((direction, (neighbor_row as usize, neighbor_col as usize)));
            }
        }

        return neigbors;
    }

    pub fn random_walk(
        &mut self,
        start: (usize, usize),
    ) -> Vec<((usize, usize), Direction, (usize, usize))> {
        let mut path: HashMap<(usize, usize), (Direction, (usize, usize))> = HashMap::new();

        let mut cell = start;

        // Keep track of what has been visited in random walk.
        let mut visited_in_walk: HashSet<(usize, usize)> = HashSet::new();
        visited_in_walk.insert(cell);

        while !self.visited.contains(&cell) {
            let mut rng = rand::rng();
            let (direction, next_cell_tuple) = self
                .neighbors(cell.0 as i32, cell.1 as i32)
                .choose(&mut rng)
                .unwrap()
                .clone();

            let (next_row, next_col) = next_cell_tuple;
            let next_cell = (next_row, next_col);

            path.insert(cell, (direction, next_cell));

            cell = next_cell;

            visited_in_walk.insert(cell);

            let mut lp: HashSet<(usize, usize)> = HashSet::new();

            while path.contains_key(&cell) && !lp.contains(&cell) {
                lp.insert(cell);

                cell = path.get(&cell).unwrap().1;
            }
        }

        let mut final_path: Vec<((usize, usize), Direction, (usize, usize))> = vec![];
        cell = start;

        while !self.visited.contains(&cell) {
            let (direction, next_cell) = path.get(&cell).unwrap().clone();

            final_path.push((cell, direction, next_cell));

            cell = next_cell;
        }

        return final_path;
    }

    pub fn generate(&mut self) {
        let mut start = self.start_cell();

        self.visited.insert(start);

        let mut unvisited: HashSet<(usize, usize)> = HashSet::new();
        for row in 0..self.rows {
            for col in 0..self.cols {
                unvisited.insert((row, col));
            }
        }

        while unvisited.len() > 0 {
            let mut rng = rand::rng();
            start = *unvisited.iter().choose(&mut rng).unwrap();
            unvisited.remove(&start);

            let walk = self.random_walk(start);

            for (cell, direction, next_cell) in walk {
                let (cell_row, cell_col) = cell;
                let (next_cell_row, next_cell_col) = next_cell;
                self.grid[cell_row][cell_col].insert(direction);
                self.grid[next_cell_row][next_cell_col].insert(direction_reverse(&direction));

                self.visited.insert(cell);
                self.visited.insert(next_cell);
            }
        }
    }
}

pub fn get_bfs_solution(
    maze: &WilsonMaze,
    priority: Priority,
) -> (HashSet<(usize, usize)>, HashSet<(usize, usize)>) {
    let (bfs_path, visited): (
        HashMap<(usize, usize), (usize, usize)>,
        HashSet<(usize, usize)>,
    ) = bfs_solve(&maze, priority);

    let mut s = maze.end_cell();
    let start = maze.start_cell();

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

/// We calculate a priority based on considering the distance
/// from start cell (lower priority) and end cell (higher priority).
pub fn weighted_priority(
    cell: (usize, usize),
    start_cell: (usize, usize),
    end_cell: (usize, usize),
) -> usize {
    let max_dist = (end_cell.0 - start_cell.0) + (end_cell.1 - start_cell.1);

    let dx_end = std::cmp::max(cell.0, end_cell.0) - std::cmp::min(cell.0, end_cell.0);
    let dy_end = std::cmp::max(cell.1, end_cell.1) - std::cmp::min(cell.1, end_cell.1);

    let dx_start = std::cmp::max(cell.0, start_cell.0) - std::cmp::min(cell.0, start_cell.0);
    let dy_start = std::cmp::max(cell.1, start_cell.1) - std::cmp::min(cell.1, start_cell.1);

    return max_dist + (dx_start + dy_start) - (dx_end + dy_end);
}

/// Cells closer to the end cell will get lower priority (very non-ideal).
pub fn close_priority(cell: (usize, usize), end_cell: (usize, usize)) -> usize {
    return (end_cell.0 - cell.0) + (end_cell.1 - cell.1);
}

pub fn bfs_solve(
    maze: &WilsonMaze,
    priority: Priority,
) -> (
    HashMap<(usize, usize), (usize, usize)>,
    HashSet<(usize, usize)>,
) {
    let end_cell = maze.end_cell();
    let start_cell = maze.start_cell();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    let mut path: HashMap<(usize, usize), (usize, usize)> = HashMap::new();

    let mut queue: PriorityQueue<(usize, usize), usize> = PriorityQueue::new();
    queue.push(start_cell, 0);

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
                    Priority::Prio => {
                        weighted_priority((nrow, ncol), maze.start_cell(), maze.end_cell())
                    }
                    Priority::Close => close_priority((nrow, ncol), maze.end_cell()),
                    Priority::Disabled => 1,
                };
                queue.push((nrow, ncol), priority);
            }
        }
    }

    panic!("No solution exists.");
}

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
        maze.start_cell(),
        maze.end_cell(),
        &mut path,
        &mut solution,
        &mut visited,
        &mut visited_to_return,
    );

    let solution_as_hashset: HashSet<(usize, usize)> = solution.into_iter().collect();

    return (solution_as_hashset, visited_to_return);
}
