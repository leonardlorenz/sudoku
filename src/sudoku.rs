#![allow(dead_code)]

extern crate rand;

use rand::prelude::*;

/**
 * returns a 2 dimensional array filled with random numbers
 */
fn generate_grid() -> [[u8; 9]; 9] {
    let mut rng = rand::thread_rng();

    let mut grid: [[u8; 9]; 9] = [[0; 9]; 9];

    for x in grid.iter_mut() {
        for y in x.iter_mut() {
            *y = rng.gen_range(1, 9);
        }
    }

    grid
}

#[derive(Copy, Clone, Debug)]
pub struct Sudoku {
    grid: [[u8; 9]; 9],
    verbose: bool,
}

impl Sudoku {
    pub fn new(verbose: bool) -> Sudoku {
        Sudoku {
            grid: generate_grid(),
            verbose,
        }
    }

    pub fn new_from_grid(new_grid: [[u8; 9]; 9], verbose: bool) -> Sudoku {
        Sudoku {
            grid: new_grid,
            verbose,
        }
    }

    /**
     * Solves the Sudoku.
     */
    //pub fn solve(&mut self) -> bool {
    //    let iterations: u64 = 0;
    //    if Sudoku::solved(self) {
    //        println!("This Sudoku is already solved.");
    //        println!("Here's the solved grid:");
    //        self.to_string();
    //        return true;
    //    } else if Sudoku::solve_rec(self, iterations) {
    //        println!("Solving successful!");
    //        println!("Here's the solved grid:");
    //        self.to_string();
    //        return true;
    //    } else {
    //        println!("Couldn't be solved.")
    //    }
    //    false
    //}

    /**
     * A recursive helper method for solve()
     * TODO fix stack overflow
     */
    //fn solve_rec(sudoku: &mut Sudoku, mut iterations: u64) -> bool {
    //    let mut current_number = 1;
    //    for x in 0..9 {
    //        for y in 0..9 {
    //            if sudoku.grid[x][y] == 0 {
    //                sudoku.grid[x][y] = current_number;
    //            } else {
    //                iterations += 1;
    //                if !Sudoku::solved(sudoku) {
    //                    current_number += 1;
    //                    sudoku.grid[x][y] = current_number;
    //                    Sudoku::solve_rec(sudoku, iterations);
    //                } else {
    //                    return true;
    //                }
    //            }
    //        }
    //    }
    //    false
    //}

    /**
     * takes three from column from index i and stores them in a Block
     */
    pub fn take_block(&self, x: usize, y: usize) -> Block {
        let take_three_from_index = |col: [u8; 9], i: usize| [col[i], col[i + 1], col[i + 2]];

        // takes block of 3x3 from coordinates
        // x = width coord
        // y = height coord
        let new_block = [
            take_three_from_index(self.grid[y], x),
            take_three_from_index(self.grid[y + 1], x),
            take_three_from_index(self.grid[y + 2], x),
        ];
        Block { grid: new_block }
    }

    /**
     * Tests if a sudoku is solved or not.
     */
    pub fn solved(&self) -> bool {
        // we need to part the game field in 9 boxes.
        // those we test first
        // every row, column, and box needs to have every single number from
        // 1 - 9 in them without duplicates
        let blocks: [Block; 9] = [
            self.take_block(0, 0),
            self.take_block(3, 0),
            self.take_block(6, 0),
            self.take_block(0, 3),
            self.take_block(3, 3),
            self.take_block(6, 3),
            self.take_block(0, 6),
            self.take_block(3, 6),
            self.take_block(6, 6),
        ];

        // check blocks
        for block in blocks.iter() {
            if !block.solved() {
                if self.verbose {
                    println!("This block is not solved:");
                    block.to_string();
                }
                return false;
            }
        }

        // check rows
        for grid_row in self.grid.iter() {
            let row = ListOfNine {
                arr: *grid_row,
                is_column: false,
            };
            if row.is_valid() {
                if self.verbose {
                    println!("This row is not valid:");
                    row.to_string();
                }
                return false;
            }
        }

        // check columns
        for x in 0..9 {
            let mut col = ListOfNine {
                arr: [0 as u8; 9],
                is_column: true,
            };
            for y in self.grid.iter() {
                col.arr[x] = y[x];
            }
            if col.is_valid() {
                if self.verbose {
                    println!("This column is not valid:");
                    col.to_string();
                }
                return false;
            }
        }

        true
    }

    /**
     * prints the game grid in a fancy way
     */
    pub fn to_string(&self) {
        println!("+---+---+---+");
        let mut i: u8 = 1;
        for x in &self.grid {
            print!("|");
            for (k, y) in x.iter().enumerate() {
                if k % 3 == 0 && k != 0 {
                    print!("|");
                }
                if *y != 0 {
                    print!("{}", y.to_string());
                } else {
                    print!(" ");
                }
            }
            println!("|");
            if i % 3 == 0 && i != 9 {
                println!("|---+---+---|");
            }
            i += 1;
        }
        println!("+-----------+");
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ListOfNine {
    pub arr: [u8; 9],
    pub is_column: bool,
}

impl ListOfNine {
    /**
     * checks if a list of 9 elements contains all numbers 0..9 but none twice
     */
    pub fn is_valid(&self) -> bool {
        let mut sorted_row = self.arr;
        sorted_row.sort();

        for (x, i) in sorted_row.iter().enumerate() {
            if *i != x as u8 {
                return false;
            }
        }
        true
    }

    pub fn to_string(&self) {
        if self.is_column {
            println!("[{},", self.arr[0]);
            for (number, i) in self.arr.iter().enumerate() {
                if *i > 1 && *i < 8 {
                    println!(" {},", number);
                }
            }
            println!("{}]", self.arr[8]);
        } else {
            print!("[");
            for number in self.arr.iter() {
                print!("{}, ", number);
            }
            println!("]");
        }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Block {
    pub grid: [[u8; 3]; 3],
}

impl Block {
    pub fn solved(&self) -> bool {
        // flatten grid into a one dimensional array
        let mut list = ListOfNine {
            arr: [0 as u8; 9],
            is_column: false,
        };
        let mut i = 0;
        for x in self.grid.iter() {
            for y in x.iter() {
                list.arr[i] = *y;
                i += 1;
            }
        }

        return list.is_valid();
    }

    pub fn to_string(&self) {
        for x in self.grid.iter() {
            for y in x.iter() {
                print!("{}", y.to_string());
            }
            println!();
        }
    }
}
