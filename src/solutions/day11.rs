pub fn div_rem<T: std::ops::Div<Output=T> + std::ops::Rem<Output=T> + Copy>(x: T, y: T) -> (T, T) {
    let quot = x / y;
    let rem = x % y;
    (rem, quot)
}

pub fn div_rem_usize(x: usize, y: usize) -> (usize, usize) {
    div_rem(x,y)
}

fn print_grid(data: &Vec<u32>, nb_col: usize, nb_row: usize) -> () {
    for nbrow in 0..nb_row { println!("{:?}", &data[nbrow*nb_col..nbrow*nb_col+nb_col]); }
}

fn increment_data_at_position(data: &mut Vec<u32>, x: usize, y: usize, nb_col: usize, nb_row: usize) -> () {
    let to_idx = |x: usize, y: usize| y*nb_col + x;
    data[to_idx(x, y)] = data[to_idx(x, y)] + 1;
}


pub fn solve(part: u8, input: &String) -> String {
    let nb_col: usize = 10;
    let nb_row: usize = 10;
    let mut data: Vec<u32> = vec![];
    for li in input.lines(){
        for c in li.chars() {
            data.push(c as u32 - 0x30);
        }
    }

    let to_idx = |x: usize, y: usize| y*nb_col + x;
    let to_coord = |x| div_rem_usize(x, nb_col);
    println!("Before any step :\n");
    print_grid(&data, nb_col, nb_row);

    let mut nb_flashes: u32 = 0;
    for step in 1..=100 {
        let mut flashed_idx: Vec<usize> = vec![];
        for idx in 0..data.len() { data[idx] = data[idx] + 1; }

        while *data.iter().max().unwrap() >= 10 {
            let mut new_flashed: Vec<usize> = vec![];
            for (index, val) in data.clone().iter().enumerate() {
                if *val >= 10 {
                    data[index] = 0;
                    nb_flashes = nb_flashes + 1;
                    flashed_idx.push(index);
                    new_flashed.push(index);
                }
            }
            for index in new_flashed {
                // Increment adjacent octopuses energy
                let (x, y) = to_coord(index);

                if x == 0 && y == 0 { // up/left corner
                    if !flashed_idx.contains(&to_idx(x+1, y)) { /* println!("up/left corner"); */ increment_data_at_position(&mut data, x+1, y, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x+1, y+1)) { /* println!("up/left corner"); */ increment_data_at_position(&mut data, x+1, y+1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x, y+1)) { /* println!("up/left corner"); */ increment_data_at_position(&mut data, x, y+1, nb_col, nb_row); }
                }
                else if x == nb_col-1 && y == 0 { // up/right corner
                    if !flashed_idx.contains(&to_idx(x-1, y)) { /* println!("up/right corner"); */ increment_data_at_position(&mut data, x-1, y, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x-1, y+1)) { /* println!("up/right corner"); */ increment_data_at_position(&mut data, x-1, y+1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x, y+1)) { /* println!("up/right corner"); */ increment_data_at_position(&mut data, x, y+1, nb_col, nb_row); }
                }
                else if y == 0 { // up side
                    if !flashed_idx.contains(&to_idx(x-1, y)) { /* println!("up side"); */ increment_data_at_position(&mut data, x-1, y, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x-1, y+1)) { /* println!("up side"); */ increment_data_at_position(&mut data, x-1, y+1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x, y+1)) { /* println!("up side"); */ increment_data_at_position(&mut data, x, y+1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x+1, y+1)) { /* println!("up side"); */ increment_data_at_position(&mut data, x+1, y+1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x+1, y)) { /* println!("up side"); */ increment_data_at_position(&mut data, x+1, y, nb_col, nb_row); }
                }
                else if x == 0 && y == nb_row-1 { // down/left corner
                    if !flashed_idx.contains(&to_idx(x, y-1)) { /* println!("down/left corner"); */ increment_data_at_position(&mut data, x, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x+1, y-1)) { /* println!("down/left corner"); */ increment_data_at_position(&mut data, x+1, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x+1, y)) { /* println!("down/left corner"); */ increment_data_at_position(&mut data, x+1, y, nb_col, nb_row); }
                }
                else if x == 0 { // left side
                    if !flashed_idx.contains(&to_idx(x, y-1)) { /* println!("left side"); */ increment_data_at_position(&mut data, x, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x+1, y-1)) { /* println!("left side"); */ increment_data_at_position(&mut data, x+1, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x+1, y)) { /* println!("left side"); */ increment_data_at_position(&mut data, x+1, y, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x+1, y+1)) { /* println!("left side"); */ increment_data_at_position(&mut data, x+1, y+1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x, y+1)) { /* println!("left side"); */ increment_data_at_position(&mut data, x, y+1, nb_col, nb_row); }
                }
                else if x == nb_col-1 && y == nb_row-1 { // down/right corner
                    if !flashed_idx.contains(&to_idx(x, y-1)) { /* println!("down/right corner"); */ increment_data_at_position(&mut data, x, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x-1, y-1)) { /* println!("down/right corner"); */ increment_data_at_position(&mut data, x-1, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x-1, y)) { /* println!("down/right corner"); */ increment_data_at_position(&mut data, x-1, y, nb_col, nb_row); }
                }
                else if x == nb_col-1 { // right side
                    if !flashed_idx.contains(&to_idx(x, y-1)) { /* println!("right side"); */ increment_data_at_position(&mut data, x, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x-1, y-1)) { /* println!("right side"); */ increment_data_at_position(&mut data, x-1, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x-1, y)) { /* println!("right side"); */ increment_data_at_position(&mut data, x-1, y, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x-1, y+1)) { /* println!("right side"); */ increment_data_at_position(&mut data, x-1, y+1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x, y+1)) { /* println!("right side"); */ increment_data_at_position(&mut data, x, y+1, nb_col, nb_row); }
                }
                else if y == nb_row-1 { // down side
                    if !flashed_idx.contains(&to_idx(x-1, y)) { /* println!("down side"); */ increment_data_at_position(&mut data, x-1, y, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x-1, y-1)) { /* println!("down side"); */ increment_data_at_position(&mut data, x-1, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x, y-1)) { /* println!("down side"); */ increment_data_at_position(&mut data, x, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x+1, y-1)) { /* println!("down side"); */ increment_data_at_position(&mut data, x+1, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x+1, y)) { /* println!("down side"); */ increment_data_at_position(&mut data, x+1, y, nb_col, nb_row); }
                }
                else { // others
                    if !flashed_idx.contains(&to_idx(x-1, y)) { /* println!("Others"); */ increment_data_at_position(&mut data, x-1, y, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x-1, y-1)) { /* println!("Others"); */ increment_data_at_position(&mut data, x-1, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x, y-1)) { /* println!("Others"); */ increment_data_at_position(&mut data, x, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x+1, y-1)) { /* println!("Others"); */ increment_data_at_position(&mut data, x+1, y-1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x+1, y)) { /* println!("Others"); */ increment_data_at_position(&mut data, x+1, y, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x+1, y+1)) { /* println!("Others") ; */ increment_data_at_position(&mut data, x+1, y+1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x, y+1)) {/* println!("Others"); */ increment_data_at_position(&mut data, x, y+1, nb_col, nb_row); }
                    if !flashed_idx.contains(&to_idx(x-1, y+1)) { /* println!("Others"); */ increment_data_at_position(&mut data, x-1, y+1, nb_col, nb_row); }
                }
            }
        }
    }
    println!("\nAfter step {} :\n", 100);
    print_grid(&data, nb_col, nb_row);
    println!("\nNb flashes : {}", nb_flashes);
    return String::from("Exit");
}