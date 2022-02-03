use std::convert::TryInto;
extern crate wasm_bindgen;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;

/*
Game of life is popular grid
problem that forces you to determine
all cells that need to die based on
a few sets of rules.

It is a popular leetcode problem and
tests your understanding. To find more
on the problem check :

https://leetcode.com/problems/game-of-life/


Note: I took the rust-wasm tutorial's
project template, but this is an original
implementation.
*/

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

#[derive(Deserialize, Serialize)]
pub struct CellsJson {
    pub cells: Vec<Vec<u8>>,
}

#[wasm_bindgen]
pub struct UniverseImpl {
    cells_json: CellsJson,
    //we use these indexes
    //to keep track of each
    //past index for ticking
    //we change each
    col_index: usize,
    row_index: usize,
    //we are storing all possible combinations of
    //neighbors as array pairs like for example
    //UP = [1,0]
    possible_neighbors: [[i8; 2]; 8],
    //js uses this to determine if ticking should stop
    pub complete: bool,
}

#[wasm_bindgen]
impl UniverseImpl {
    pub fn new(cells: String) -> Self {
        Self {
            cells_json: serde_json::from_str(&cells).unwrap(),
            col_index: 0,
            row_index: 0,
            possible_neighbors: [
                [1, 0],   //down
                [-1, 0],  //up
                [0, 1],   //right
                [0, -1],  //left
                [1, 1],   //down right
                [-1, -1], //top left
                [1, -1],  //bottom left
                [-1, 1],
            ], //top right
            complete: false,
        }
    }

    pub fn tick(&mut self) {
        self.col_index += 1;
        if !(0..self.cells_json.cells[0].len()).contains(&self.col_index) {
            //ensure there is another row
            if (0..self.cells_json.cells.len()).contains(&(self.row_index + 1)) {
                self.col_index = 0;
                self.row_index += 1;
            } else {
                self.complete = true;
                log("completed");
                return;
            }
            return;
        }

        let current_count = self.get_live_neighbor_count();
        if self.cells_json.cells[self.row_index][self.col_index] == 1 {
            //if it doesn't have 2 or 3 neighbors it dies
            if current_count != 2 || current_count != 3 {
                self.cells_json.cells[self.row_index][self.col_index] = 3;
            }
        } else {
            if current_count == 3 {
                self.cells_json.cells[self.row_index][self.col_index] = 2;
            }
        }
    }

    // 0 = Dead
    // 1 = Alive
    // 2 = WasDeadNowAlive
    // 3 = WasAliveNowDead
    fn get_live_neighbor_count(&self) -> usize {
        let mut live_neighbor_count = 0;
        for possible_neighbor in self.possible_neighbors {
            let temp_row = self.row_index as i8;
            let temp_col = self.col_index as i8;
            let current_possible_row: i8 = possible_neighbor[0] + temp_row;
            let current_possible_col: i8 = possible_neighbor[1] + temp_col;

            if current_possible_row >= 0 && current_possible_col >= 0 {
                let curr_row_as_usize: usize = current_possible_row.try_into().unwrap();
                let curr_col_as_usize: usize = current_possible_col.try_into().unwrap();

                // if this new neighbor is in range and its alive
                if (0..self.cells_json.cells.len()).contains(&curr_row_as_usize)
                    && (0..self.cells_json.cells[0].len()).contains(&curr_col_as_usize)
                {
                    if self.cells_json.cells[curr_row_as_usize][curr_row_as_usize] == 2
                        || self.cells_json.cells[curr_row_as_usize][curr_row_as_usize] == 1
                    {
                        live_neighbor_count += 1;
                    }
                }
            }
        }
        return live_neighbor_count;
    }

    pub fn render(&self) -> String {
        let mut starting_string: String = String::from("");
        log("rendering");
        for row in &self.cells_json.cells {
            
            for column in row {
                log(&column.to_string());
                if column == &(2 as u8) || column == &(1 as u8) {
                    starting_string.push_str("◼");
                } else {
                    starting_string.push_str("◻");
                }
            }
            starting_string.push_str("\n");
        }
        return starting_string;
    }
}
