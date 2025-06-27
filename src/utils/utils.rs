use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use rand::prelude::*;

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
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
// For now, only the wilson maze generation algorithm is supported.
#[derive(Debug, Clone)]
pub struct WilsonMaze {
    pub rows: usize,
    pub cols: usize,
    pub start_cell: (usize, usize),
    pub end_cell: (usize, usize),
    pub visited: HashSet<(usize, usize)>,
    pub grid: Vec<Vec<HashSet<Direction>>>,
}

impl WilsonMaze {
    pub fn new(
        rows: usize,
        cols: usize,
        start_cell: (usize, usize),
        end_cell: (usize, usize),
    ) -> Self {
        return Self {
            rows: rows,
            cols: cols,
            start_cell: start_cell,
            end_cell: end_cell,
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

    pub fn max_dist(&self) -> usize {
        return (self.rows - 0) + (self.cols - 0);
    }

    pub fn in_bounds(&self, row: i32, col: i32) -> bool {
        return (row >= 0 && row < self.rows as i32) && (col >= 0 && col < self.cols as i32);
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
        let mut start = self.start_cell;

        self.visited.insert(start);

        // We can probably make this better by using
        // self.visited or similar.
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
