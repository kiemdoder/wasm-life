mod utils;

use wasm_bindgen::prelude::*;
use std::fmt::{Display, Formatter, Result as fmtResult};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Cell {
    Dead = 0,
    Alive = 1
}

#[wasm_bindgen]
pub struct Universe {
    width: u32,
    height: u32,
    cells: Vec<Cell>
}

#[wasm_bindgen]
impl Universe {
    pub fn new() -> Self {
        let width = 64;
        let height = 64;

        let cells = (0..width * height).map(|i| {
            if i % 2 == 0 || i % 7 == 0 {
                Cell::Alive
            } else {
                Cell::Dead
            }
        }).collect();

        Universe {width, height, cells}
    }

    pub fn render(&self) -> String {
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
            for col_delta in [self.width, 0, 1] {
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
}

impl Display for Universe {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        for line in self.cells.as_slice().chunks(self.width as usize) {
            for &cell in line {
                let symbol = if cell == Cell::Dead {'◻'} else {'◼'};
                write!(f, "{}", symbol);
            }
            write!(f, "\n");
        }

        Ok(())
    }
}