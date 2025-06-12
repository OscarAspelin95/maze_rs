use dioxus::prelude::*;

use crate::utils::{get_backtrack_solution, get_bfs_solution, Direction, Priority, WilsonMaze};
use std::collections::HashSet;
const NROWS_PLACEHOLDER: usize = 10;
const NCOLS_PLACEHOLDER: usize = 10;

/// General TODO for entire project:
/// * Smaller things:
///
/// * Refactor:
///     ** Separate scripts for solvers.
///     ** Move certain functions out of maze.rs
///
/// * Misc:
///     ** Add ability to set start point.
///     ** Check why row/col selection sometimes bugs out (dioxus related)?
///     ** Remove unused functions.
///     ** Make solver into an Enum.
///     ** Streamline css and get_class_name function for cells.
///     ** Other distance metric for BFS Prio?
///     ** Visited does not include start and/or end cell.
///
/// * Performance:
///     ** Visited should actually be (visited - solution).
///     ** Rework maze class with how we generate and store grid (and solution).
///
///
pub fn get_class_name(
    grid: &Vec<Vec<HashSet<Direction>>>,
    row: usize,
    col: usize,
    is_solution: bool,
    is_visit: bool,
) -> String {
    let cell = &grid[row][col];

    let last_row = grid.len() - 1;
    let last_col = grid[0].len() - 1;

    if cell.len() == 0 {
        return "maze-cell no".to_string();
    }

    let mut directions = "maze-cell ".to_string();

    if cell.contains(&Direction::Left) || (row == 0 && col == 0) {
        directions.push('l');
    }

    // We need to fix this as well for the end cell.
    if cell.contains(&Direction::Right) || (row == last_row && col == last_col) {
        directions.push('r');
    }

    if cell.contains(&Direction::Up) {
        directions.push('u');
    }

    if cell.contains(&Direction::Down) {
        directions.push('d');
    }

    if row == 0 && col == 0 {
        directions.push_str(" start");
    }

    if row == last_row && col == last_col {
        directions.push_str(" end");
    }

    if is_solution {
        directions.push_str(" solution");
    }

    if !is_solution && is_visit {
        directions.push_str(" visited");
    }

    return directions;
}

#[component]
pub fn Maze() -> Element {
    // We need signals and use effects for rows and columns
    let mut nrows: Signal<usize> = use_signal(|| NROWS_PLACEHOLDER);
    let mut ncols: Signal<usize> = use_signal(|| NCOLS_PLACEHOLDER);

    //
    let mut solution: Signal<HashSet<(usize, usize)>> = use_signal(|| HashSet::new());
    let mut visited: Signal<HashSet<(usize, usize)>> = use_signal(|| HashSet::new());
    let mut solver: Signal<String> = use_signal(|| "bfs".to_string());

    let mut maze: Signal<WilsonMaze> =
        use_signal(|| WilsonMaze::new(NROWS_PLACEHOLDER, NCOLS_PLACEHOLDER));

    // When changing cols or rows, we need to update the maze
    // and also set the solution to empty.
    use_effect(move || {
        let mut m = WilsonMaze::new(*nrows.read(), *ncols.read());
        m.generate();
        maze.set(m);
        solution.set(HashSet::new());
        visited.set(HashSet::new());
    });

    let m = &maze.read();

    let sol = &solution.read();
    let visit = &visited.read();

    rsx! {
        div { id: "container",
            h1 { id: "header", "Generate a maze!" }

            div { id: "row-col-input-container",
                div { id: "row-input-container",
                    label { r#for: "row-input", "Rows: " }
                    input {
                        id: "row-input",
                        r#type: "range",
                        value: "10",
                        min: "10",
                        max: "40",
                        step: "10",
                        list: "row-markers",
                        class: "slider",
                        onchange: move |evt| { nrows.set(evt.value().parse().unwrap()) },
                    }
                    datalist { id: "row-markers",
                        option { value: "10" }
                        option { value: "20" }
                        option { value: "30" }
                        option { value: "40" }
                    }
                    span { id: "row-input-span", "{nrows}" }
                }

                div { id: "col-input-container",
                    label { r#for: "col-input", "Columns: " }
                    input {
                        id: "col-input",
                        r#type: "range",
                        value: "10",
                        min: "10",
                        max: "40",
                        step: "10",
                        list: "col-markers",
                        class: "slider",
                        onchange: move |evt| { ncols.set(evt.value().parse().unwrap()) },
                    }
                    datalist { id: "col-markers",
                        option { value: "10" }
                        option { value: "20" }
                        option { value: "30" }
                        option { value: "40" }
                    }
                    span { id: "col-input-span", "{ncols}" }

                }


            }

            // We might streamline this with some kind of
            // enum of a vec of options.
            div { id: "solver-container",
                label { id: "solver-label", r#for: "solver", "Choose a solver:" }
                select {
                    id: "solver",
                    name: "solver",
                    onchange: move |evt| {
                        solver.set(evt.value());
                    },
                    option { value: "bfs", "BFS Default" }
                    option { value: "bfs-priority", "BFS Prio" }
                    option { value: "bfs-close", "BFS Close" }
                    option { value: "backtrack", "DFS Backtrack" }

                }
            }

            div { id: "btn-row",
                button {
                    id: "solve-btn",
                    onclick: move |_| {
                        let (maze_solution, maze_visited) = match &solver.read().as_str() {
                            &"bfs" => get_bfs_solution(&maze.read(), Priority::Disabled),
                            &"bfs-prio" => get_bfs_solution(&maze.read(), Priority::Prio),
                            &"bfs-close" => get_bfs_solution(&maze.read(), Priority::Close),
                            &"backtrack" => get_backtrack_solution(&maze.read()),
                            _ => panic!("Invalid solver method."),
                        };
                        solution.set(maze_solution);
                        visited.set(maze_visited);
                    },
                    "Solve"
                }

                button {
                    id: "reset-btn",
                    onclick: move |_| {
                        solution.set(HashSet::new());
                        visited.set(HashSet::new());
                    },
                    "Reset"
                }
            }

            div { id: "num-iterations",

                match visit.len() {
                    0 => format!("Maze is unsolved..."),
                    _ => {
                        format!(
                            "Solved in {} iterations ({}% of maze searched).",
                            visit.len(),
                            (100 as f32 * (visit.len() as f32 / (ncols * *nrows.read()) as f32))
                                as usize,
                        )
                    }
                }

            }

            div { id: "maze-container",

                for row in 0..maze.read().rows {
                    div { id: "maze-row",
                        for col in 0..maze.read().cols {
                            div {
                                class: get_class_name(
                                    &m.grid,
                                    row,
                                    col,
                                    sol.contains(&(row, col)),
                                    visit.contains(&(row, col)),
                                ),
                                // This is not ideal
                                if row == 0 && col == 0 {
                                    span { id: "start-cell", "S" }
                                }
                                if row == *nrows.read() - 1 && col == *ncols.read() - 1 {
                                    span { id: "end-cell", "E" }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
