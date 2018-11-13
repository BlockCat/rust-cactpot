type Grid = [[Option<u8>; 3]; 3];
fn main() {
    let mut grid: Grid = [[None; 3]; 3];    
    let payouts: [i32; 25] = [0, 0, 0, 0, 0, 0, 10_000, 36, 720, 360, 80, 252, 108, 72, 54, 180, 72, 180, 119, 36, 306, 1080, 144, 1800, 3600];

    grid[0][0] = Some(1);
    grid[0][1] = Some(2);   
    grid[1][1] = Some(3);
    

    print_grid(&grid);
    calculate_row_candidates(&grid, 0, payouts);
    calculate_row_candidates(&grid, 1, payouts);
    calculate_row_candidates(&grid, 2, payouts);
    calculate_column_candidates(&grid, 0, payouts);
    calculate_column_candidates(&grid, 1, payouts);
    calculate_column_candidates(&grid, 2, payouts);
}

// Numbers in a range.
// If the first number is 1, then the range can of payouts can be [1+2+3, 1+8+9] = [6, 18]
//
// Calculate the expected of a row
fn calculate_row_candidates(grid: &Grid, row: u32, payouts:[i32; 25]) -> i32 {
   let digits = [grid[row as usize][0], grid[row as usize][1], grid[row as usize][2]];
   return calculate_expected_value(grid, payouts, digits);
}

fn calculate_column_candidates(grid: &Grid, column: u32, payouts:[i32; 25]) -> i32 {
    let digits = [grid[0][column as usize], grid[1][column as usize], grid[0][column as usize]];
    return calculate_expected_value(grid, payouts, digits);
}

fn calculate_expected_value(grid: &Grid,payouts:[i32; 25], digits: [Option<u8>; 3]) -> i32 {
    let row_digits: Vec<u8> = digits.iter().filter(|x| x.is_some()).map(|x| x.unwrap()).collect();
    let row_sum: u8 = row_digits.iter().sum();
    let used_digits: Vec<u8> = grid.iter().flat_map(|t| t.iter()).filter(|x| x.is_some()).map(|x| x.unwrap()).collect();
    let unused_digits: Vec<u8> = vec![1,2,3,4,5,6,7,8,9].into_iter().filter(|x| !used_digits.contains(&x)).collect();    
   
    // A generator that generates all possible values.
    let possible_sums: Vec<i32> = calculate_candidate_sums(row_digits.clone(), unused_digits.clone());

    let max_size = possible_sums.iter().map(|x| payouts[*x as usize]).sum::<i32>();
    let size: i32 = possible_sums.len() as i32;
    println!("Current Expected value: {}", max_size / size);
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


fn print_grid(grid: &[[Option<u8>; 3]; 3]) {
    for i in 0..3 {
        let a = grid[i][0].map(|s| s.to_string()).unwrap_or(String::from(" "));
        let b = grid[i][1].map(|s| s.to_string()).unwrap_or(String::from(" "));
        let c = grid[i][2].map(|s| s.to_string()).unwrap_or(String::from(" "));

        println!("|{}|{}|{}|", a, b, c);
    }
}
