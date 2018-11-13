//type Grid = [[Option<u8>; 3]; 3];

#[derive(Clone)]
struct Grid {
    grid: [[Option<u8>; 3]; 3],
    used_digits: [bool; 9],    
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

fn main() {    
    let payouts: [i32; 25] = [0, 0, 0, 0, 0, 0, 10_000, 36, 720, 360, 80, 252, 108, 72, 54, 180, 72, 180, 119, 36, 306, 1080, 144, 1800, 3600];
    let grid = Grid::new().set(Some(1), (0, 0));
    grid.pretty_print();

    println!("Score: {}", score(&grid, payouts));
    
    println!("Next score (0, 1): {}", calculate_average_increase(&grid, payouts, (0, 1)));
    println!("Next score (0, 2): {}", calculate_average_increase(&grid, payouts, (0, 2)));

}

// Numbers in a range.
// If the first number is 1, then the range can of payouts can be [1+2+3, 1+8+9] = [6, 18]
//
// Calculate the expected of a row
fn calculate_row_candidates(grid: &Grid, row: u32, payouts:[i32; 25]) -> i32 {
   let digits = [grid.grid[row as usize][0], grid.grid[row as usize][1], grid.grid[row as usize][2]];
   return calculate_expected_value(grid, payouts, digits);
}

fn calculate_column_candidates(grid: &Grid, column: u32, payouts:[i32; 25]) -> i32 {
    let digits = [grid.grid[0][column as usize], grid.grid[1][column as usize], grid.grid[2][column as usize]];
    return calculate_expected_value(grid, payouts, digits);
}

fn calculate_diagonal_candidates(grid: &Grid, diagonal: u32, payouts: [i32; 25]) -> i32 {
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
    
    println!("unused: {:?}", unused_digits);
    // A generator that generates all possible values.
    let possible_sums: Vec<i32> = calculate_candidate_sums(row_digits.clone(), unused_digits.clone());

    let max_size = possible_sums.iter().map(|x| payouts[*x as usize]).sum::<i32>();
    let size: i32 = possible_sums.len() as i32;
    //println!("Current Expected value: {}", max_size / size);
    return max_size / size;
}

fn calculate_candidate_sums(used: Vec<u8>, unused_digits: Vec<u8>) -> Vec<i32> {
    match used.len() {
        3 => vec!(used.into_iter().map(|x| x as i32).sum()),
        _ => unused_digits.iter().flat_map(|x| {
            let mut u = used.clone();
            let d = unused_digits.clone().into_iter();
            u.push(x.clone());            
            calculate_candidate_sums(u, d.filter(|a| x != a).collect())
        }).collect()
    }
}

fn calculate_average_increase(grid: &Grid, payouts: [i32; 25], point: (i32, i32)) -> i32 {
    let (y, x) = point;

    grid.used_digits.iter().enumerate().filter(|(_, b)| !**b).map(|(i, _)| {
        score(&grid.set(Some((i + 1) as u8), (y, x)), payouts)
    }).sum::<i32>()
}

fn score(grid: &Grid, payouts: [i32; 25]) -> i32 {
    [calculate_row_candidates(&grid, 0, payouts),
        calculate_row_candidates(&grid, 1, payouts),
        calculate_row_candidates(&grid, 2, payouts),        
        calculate_column_candidates(&grid, 0, payouts),
        calculate_column_candidates(&grid, 1, payouts),
        calculate_column_candidates(&grid, 2, payouts),       
        calculate_diagonal_candidates(&grid, 0, payouts),
        calculate_diagonal_candidates(&grid, 1, payouts)].iter().sum::<i32>()      
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test] fn test_sums() {
        let payouts: [i32; 25] = [0, 0, 0, 0, 0, 0, 10_000, 36, 720, 360, 80, 252, 108, 72, 54, 180, 72, 180, 119, 36, 306, 1080, 144, 1800, 3600];
        let grid = Grid::new().set(Some(1), (0, 0));
        assert_eq!(2964, score(&grid, payouts), "Testing grid");
    }
}