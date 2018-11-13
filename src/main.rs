use std::ops::{Add, Div, Sub};
use std::iter::Sum;

#[derive(Clone)]
struct Grid {
    grid: [[Option<u8>; 3]; 3],
    used_digits: [bool; 9],    
}

#[derive(Debug)]
struct GridResult {
    rows: [i32; 3],
    columns: [i32; 3],
    diagonals: [i32; 2]
}


impl GridResult {
    fn new() -> GridResult {
        GridResult {
            rows: [0; 3],
            columns: [0; 3],
            diagonals: [0; 2]
        }
    }

    fn sum(&self) -> i32 {
        let a = self.rows.iter().sum::<i32>();
        let b = self.columns.iter().sum::<i32>();
        let c = self.diagonals.iter().sum::<i32>();

        a + b + c
    }
}

impl Add for GridResult {
    type Output = GridResult;

    fn add(self, other: GridResult) -> GridResult {
        let orows = [self.rows[0] + other.rows[0], self.rows[1] + other.rows[1], self.rows[2] + other.rows[2]];
        let ocols = [self.columns[0] + other.columns[0], self.columns[1] + other.columns[1], self.columns[2] + other.columns[2]];
        let odiagonals = [self.diagonals[0] + other.diagonals[0], self.diagonals[1] + other.diagonals[1]];

        GridResult {
            rows: orows,
            columns: ocols,
            diagonals: odiagonals
        }
    }
}

impl<'a, 'b> Sub<&'a GridResult> for &'b GridResult {
    type Output = GridResult;

    fn sub(self, other: &'a GridResult) -> GridResult {
        let orows = [self.rows[0] - other.rows[0], self.rows[1] - other.rows[1], self.rows[2] - other.rows[2]];
        let ocols = [self.columns[0] - other.columns[0], self.columns[1] - other.columns[1], self.columns[2] - other.columns[2]];
        let odiagonals = [self.diagonals[0] - other.diagonals[0], self.diagonals[1] - other.diagonals[1]];

        GridResult {
            rows: orows,
            columns: ocols,
            diagonals: odiagonals
        }
    }
}

impl Div<i32> for GridResult {
    type Output = GridResult;

    fn div(self, other: i32) -> GridResult {
        let orows = [self.rows[0] / other, self.rows[1] / other, self.rows[2] / other];
        let ocols = [self.columns[0] / other, self.columns[1] / other, self.columns[2] / other];
        let odiagonals = [self.diagonals[0] / other, self.diagonals[1] / other];

        GridResult {
            rows: orows,
            columns: ocols,
            diagonals: odiagonals
        }
    }
}

impl Sum for GridResult {
    fn sum<I: Iterator<Item=GridResult>>(iter: I) -> GridResult {
        iter.fold(GridResult::new(), |a, b| a + b)
    }
}

impl Grid {
    fn new() -> Grid {
        Grid {
            grid: [[None; 3]; 3],
            used_digits: [false; 9]
       }
    }

    fn set(&self, number: Option<u8>, point: (i32, i32)) -> Grid {
        let mut c = self.clone();
        let (y, x) = point;

        match number {
            Some(e) => {
                // 
                if let Some(cn) = c.grid[y as usize][x as usize] {
                    // Replace number                    
                    c.used_digits[(cn - 1) as usize] = false;
                }                
                c.used_digits[(e - 1) as usize] = true;
                
            },
            _ => {
                if let Some(cn) = c.grid[y as usize][x as usize] {
                    // Replace number                    
                    c.used_digits[(cn - 1) as usize] = false;
                }
            }
        }
        c.grid[y as usize][x as usize] = number;

        c
    }

    fn pretty_print(&self) {
    for i in 0..3 {
            let a = self.grid[i][0].map(|s| s.to_string()).unwrap_or(String::from(" "));
            let b = self.grid[i][1].map(|s| s.to_string()).unwrap_or(String::from(" "));
            let c = self.grid[i][2].map(|s| s.to_string()).unwrap_or(String::from(" "));

            println!("|{}|{}|{}|", a, b, c);
        }
    }
}

// (0, 1): 6
// (1, 1): 2
// (2, 1): 4

fn main() {    
    let payouts: [i32; 25] = [0, 0, 0, 0, 0, 0, 10_000, 36, 720, 360, 80, 252, 108, 72, 54, 180, 72, 180, 119, 36, 306, 1080, 144, 1800, 3600];
    let grid = Grid::new().set(Some(9), (0, 0)).set(Some(4), (0, 1)).set(Some(5), (0, 2)).set(Some(3), (1, 1));
    /*let grid = Grid {
        grid: [[Some(6), Some(3), Some(2)], [Some(5), Some(8), Some(7)], [Some(1), Some(9) ,Some(4)]],
        used_digits: [true; 9] 
    };*/
    grid.pretty_print();

    let original_score = score(&grid, payouts);
    println!("Score: {:?}", original_score);
    
    for i in 0..3 {
        for j in 0..3 {
            if let None = grid.grid[i][j] {
                let dif = &calculate_average_increase(&grid, payouts, (i as i32, j as i32)) - &original_score;

                println!("R ({}, {}): {:?}", i, j, dif.sum());
                
            }
        }
    }
}

fn score(grid: &Grid, payouts: [i32; 25]) -> GridResult {
    GridResult {
        rows: [calculate_expected_row_value(&grid, 0, payouts), calculate_expected_row_value(&grid, 1, payouts), calculate_expected_row_value(&grid, 2, payouts)],
        columns: [calculate_expected_column_value(&grid, 0, payouts), calculate_expected_column_value(&grid, 1, payouts), calculate_expected_column_value(&grid, 2, payouts)],
        diagonals: [calculate_expected_diagonal_value(&grid, 0, payouts), calculate_expected_diagonal_value(&grid, 1, payouts)]
    }
}

fn calculate_average_increase(grid: &Grid, payouts: [i32; 25], point: (i32, i32)) -> GridResult {    
    //let unused_count = grid.used_digits.iter().enumerate().filter(|(_, b)| !**b).count() as i32;        
    grid.used_digits.iter().enumerate().filter(|(_, b)| !**b).map(|(i, _)| { score(&grid.set(Some((i + 1) as u8), point), payouts) }).sum::<GridResult>()
}


// Numbers in a range.
// If the first number is 1, then the range can of payouts can be [1+2+3, 1+8+9] = [6, 18]
//
// Calculate the expected of a row
fn calculate_expected_row_value(grid: &Grid, row: u32, payouts:[i32; 25]) -> i32 {   
   return calculate_expected_value(grid, payouts, [grid.grid[row as usize][0], grid.grid[row as usize][1], grid.grid[row as usize][2]]);
}

fn calculate_expected_column_value(grid: &Grid, column: u32, payouts:[i32; 25]) -> i32 {    
    return calculate_expected_value(grid, payouts, [grid.grid[0][column as usize], grid.grid[1][column as usize], grid.grid[2][column as usize]]);
}

fn calculate_expected_diagonal_value(grid: &Grid, diagonal: u32, payouts: [i32; 25]) -> i32 {
    match diagonal {
        0 => calculate_expected_value(grid, payouts, [grid.grid[0][0], grid.grid[1][1], grid.grid[2][2]]),
        1 => calculate_expected_value(grid, payouts, [grid.grid[0][2], grid.grid[1][1], grid.grid[2][0]]),
        _ => panic!("")
    }
}

fn calculate_expected_value(grid: &Grid,payouts:[i32; 25], digits: [Option<u8>; 3]) -> i32 {
    // Calculate the digits that are used in the current line
    let row_digits: Vec<u8> = digits.iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect();    

    // All digits that are used in the grid    
    let unused_digits: Vec<u8> = grid.used_digits.iter().enumerate().filter(|(_, b)| !**b).map(|(i, _)| (i+1) as u8).collect();

    
    // A generator that generates all possible values.
    let possible_sums: Vec<i32> = calculate_candidate_sums(row_digits.clone(), unused_digits.clone());

    let max_size = possible_sums.iter().map(|x| payouts[*x as usize]).sum::<i32>();
    let size: i32 = possible_sums.len() as i32;
    
    return max_size / size;
}

fn calculate_candidate_sums(used: Vec<u8>, unused_digits: Vec<u8>) -> Vec<i32> {
    match used.len() {
        3 => vec!(used.into_iter().map(|x| x as i32).sum()),
        _ => unused_digits.iter().flat_map(|x| {
            let mut u = used.clone();            
            u.push(x.clone());
            calculate_candidate_sums(u, unused_digits.clone().into_iter().filter(|a| x != a).collect())
        }).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_rows_one_value() {
        let payouts: [i32; 25] = [0, 0, 0, 0, 0, 0, 10_000, 36, 720, 360, 80, 252, 108, 72, 54, 180, 72, 180, 119, 36, 306, 1080, 144, 1800, 3600];
        let grid = Grid::new().set(Some(1), (0, 0));
        let score = score(&grid, payouts);
        assert_eq!(528, score.rows[0], "Testing grid - row 0");
        assert_eq!(276, score.rows[1], "Testing grid - row 1");
        assert_eq!(276, score.rows[2], "Testing grid - row 2");

        assert_eq!(528, score.columns[0], "Testing grid - col 0");
        assert_eq!(276, score.columns[1], "Testing grid - col 1");
        assert_eq!(276, score.columns[2], "Testing grid - col 2");

        assert_eq!(528, score.diagonals[0], "Testing grid - diagonal 0");
        assert_eq!(276, score.diagonals[1], "Testing grid - diagonal 1");
    }

        #[test] fn test_two_value() {
        let payouts: [i32; 25] = [0, 0, 0, 0, 0, 0, 10_000, 36, 720, 360, 80, 252, 108, 72, 54, 180, 72, 180, 119, 36, 306, 1080, 144, 1800, 3600];
        let grid = Grid::new().set(Some(1), (0, 0)).set(Some(3), (0, 2));
        let score = score(&grid, payouts);
        assert_eq!(1656, score.rows[0], "Testing grid - row 0");
        assert_eq!(363, score.rows[1], "Testing grid - row 1");
        assert_eq!(363, score.rows[2], "Testing grid - row 2");

        assert_eq!(153, score.columns[0], "Testing grid - col 0");
        assert_eq!(363, score.columns[1], "Testing grid - col 1");
        assert_eq!(130, score.columns[2], "Testing grid - col 2");

        assert_eq!(153, score.diagonals[0], "Testing grid - diagonal 0");
        assert_eq!(130, score.diagonals[1], "Testing grid - diagonal 1");
    }
}