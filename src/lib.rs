extern crate web_sys;
extern crate js_sys;

mod utils;

use wasm_bindgen::prelude::*;
use std::fmt::{Display, Formatter, Result as fmtResult};

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

fn wrap_for_size(i: i32, size: u32) -> u32 {
    let s = size as i32;
    ((s + i) % s) as u32
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1,
}

impl Cell {
    pub fn toggle(&mut self) {
        *self = match self {
            Cell::Dead => Cell::Alive,
            Cell::Alive => Cell::Dead
        }
    }

    pub fn rand_cell_value() -> Cell {
        if js_sys::Math::random() < 0.5 {
            Cell::Dead
        } else {
            Cell::Alive
        }
    }
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>,
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let width = 200;
        let height = 100;

        log!("new universe {} x {}", width, height);

        let cells = (0..width * height).map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }).collect();

        Universe { width, height, cells }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
        self.cells = (0..width * self.height()).map(|_| Cell::Dead).collect();
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
        self.cells = (0..self.width * height).map(|_| Cell::Dead).collect();
    }

    pub fn cells(&self) -> *const Cell {
        self.cells.as_ptr()
    }

    pub fn text_render(&self) -> String {
        self.to_string()
    }

    pub fn tick(&mut self) {
        let mut next_state = self.cells.clone();

        for row in 0..self.height {
            for col in 0..self.width {
                let index = self.get_index(row, col);
                let cell = self.cells[index];
                let live_neighbors = self.live_neighbor_count(row, col);

                let next_cell_state = match (cell, live_neighbors) {
                    (Cell::Alive, living) if living < 2 => Cell::Dead,
                    (Cell::Alive, living) if living > 3 => Cell::Dead,
                    (Cell::Alive, 2) | (Cell::Alive, 3) => Cell::Alive,
                    (Cell::Dead, 3) => Cell::Alive,
                    (otherwise, _) => otherwise,
                };
                next_state[index] = next_cell_state;
            }
        }

        self.cells = next_state;
    }

    fn get_index(&self, row: u32, col: u32) -> usize {
        (row * self.width + col) as usize
    }

    fn live_neighbor_count(&self, row: u32, col: u32) -> u8 {
        let mut count: u8 = 0;
        for row_delta in [self.height - 1, 0, 1] {
            for col_delta in [self.width - 1, 0, 1] {
                if row_delta == 0 && col_delta == 0 {
                    continue;
                }

                let neighbor_row = (row + row_delta) % self.height;
                let neighbor_col = (col + col_delta) % self.width;
                let index = self.get_index(neighbor_row, neighbor_col);
                count += self.cells[index] as u8; //why cast to u8 here after repr(u8) for Cell?
            }
        }

        count
    }

    pub fn toggle_cell(&mut self, row: u32, col: u32) {
        let index = self.get_index(row, col);
        self.cells[index].toggle();
    }

    pub fn inject_random_cells(&mut self, row: u16, col: u16, size: i32) {
        for col_delta in -size..size {
            let col_with_offset = wrap_for_size(col as i32 + col_delta, self.width);
            for row_delta in -size..size {
                let row_with_offset = wrap_for_size(row as i32 + row_delta, self.height);
                let index = self.get_index(row_with_offset, col_with_offset);
                self.cells[index] = Cell::rand_cell_value();
            }
        }
    }
}

// Another Universe implementation without the wasm_bindgen annotation. This is because Rust-generated
// WebAssembly functions cannot return borrowed references
impl Universe {
    /// Get the dead and alive values of the entire universe.
    pub fn get_cells(&self) -> &[Cell] {
        &self.cells
    }

    /// Set cells to be alive in a universe by passing the row and column
    /// of each cell as an array.
    pub fn set_cells(&mut self, cells: &[(u32, u32)]) {
        for (row, col) in cells.iter().cloned() {
            let idx = self.get_index(row, col);
            self.cells[idx] = Cell::Alive;
        }
    }
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead { '???' } else { '???' };
                write!(f, "{}", symbol);
            }
            write!(f, "\n");
        }

        Ok(())
    }
}