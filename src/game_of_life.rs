use serde::{Deserialize, Serialize};

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

pub enum CellState{
    Alive = 1,
    Dead = 0,
    WasDeadNowAlive = 2,
    WasAliveNowDead = 3
}

#[derive(Deserialize, Serialize)]
pub struct CellsJson{
    pub cells:Vec<Vec<u8>>
}

#[wasm_bindgen]
pub struct UniverseImpl{
    pub cells_json:CellsJson,
    //we use these indexes
    //to keep track of each
    //past index for ticking
    //we change each
    pub last_col_index:usize,
    pub last_row_index:usize,
}

impl UniverseImpl{
   pub fn new(cells:String)->Self{
       Self{
        cells:serde_json::from_str(&cells).unwrap()
       }
   }
}
