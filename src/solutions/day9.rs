use std::collections::HashMap;


fn get_dim(input: &String) -> (usize, usize, Vec<u32>) {
    let mut data: Vec<u32> = vec![];
    let mut nb_col: usize = 0;
    let mut nb_row: usize = 0;
    for li in input.lines(){
        nb_col = li.len();
        nb_row += 1;
        for c in li.chars() {
            data.push(c as u32 - 0x30);
        }
    }
    return (nb_col, nb_row, data);
}

fn get_lowest_indexes(nb_col: usize, nb_row: usize, data: Vec<u32>) -> (Vec<u32>, Vec<usize>) {
    let mut lowest_indexes: Vec<usize> = vec![];

    println!("line size : {:?} - Nb lines : {:?}", nb_col, nb_row);
    //println!("data : {:?}", data);
    println!("nb_col : {:?} - nb_row : {:?}", nb_col, nb_row);
    for (index, val) in data.iter().enumerate(){
        // corner cases
        if index == 0 {if val < &data[1] && val < &data[nb_col] {lowest_indexes.push(index);}}
        else if index == nb_col-1 {if val < &data[nb_col-2] && val < &data[2*nb_col-1] {lowest_indexes.push(index);}}
        else if index == (nb_row-1)*nb_col {if val < &data[index+1] && val < &data[index-nb_col] {lowest_indexes.push(index);}}
        else if index == (nb_row*nb_col)-1 {if val < &data[index-1] && val < &data[index-nb_col] {lowest_indexes.push(index);}}
        // letf/right sides
        else if index%nb_col == 0 {if val < &data[index+1] && val < &data[index-nb_col] && val < &data[index+nb_col] {lowest_indexes.push(index);}}
        else if index>1 && (index+1)%nb_col == 0 {if val < &data[index-1] && val < &data[index-nb_col] && val < &data[index+nb_col] {lowest_indexes.push(index);}}
        // Bottom/up sides
        else if index > 0 && index < nb_col-1 {if val < &data[index-1] && val < &data[index+1] && val < &data[index+nb_col] {lowest_indexes.push(index);}}
        else if index > (nb_row-1)*nb_col && index < (nb_row*nb_col)-1 {if val < &data[index-1] && val < &data[index+1] && val < &data[index-nb_col] {lowest_indexes.push(index);}}
        // Else
        else {if val < &data[index-1] && val < &data[index+1] && val < &data[index-nb_col] && val < &data[index+nb_col] {lowest_indexes.push(index);}}
    }
    return (data, lowest_indexes);
}

pub fn div_rem<T: std::ops::Div<Output=T> + std::ops::Rem<Output=T> + Copy>(x: T, y: T) -> (T, T) {
    let quot = x / y;
    let rem = x % y;
    (rem, quot)
}

pub fn div_rem_usize(x: usize, y: usize) -> (usize, usize) {
    div_rem(x,y)
}

pub fn solve(part: u8, input: &String) -> String {
    println!("Start day 9");
    // get_lowest_indexes returns data as a vector of u32 and indexes of lowest values as vector of usize
    let (nb_col, nb_row, data) = get_dim(input);
    let (data, indexes) = get_lowest_indexes(nb_col, nb_row, data);

    let result: u32 = indexes.iter().map(|x| data[*x]+1).sum::<u32>();
    println!("Result : {}", result);
    println!("indexes : {:?}", indexes);

    if part==1 { return String::from("Exit part 1"); }

    let to_idx = |x: usize, y: usize| y*nb_col + x;
    let to_coord = |x| div_rem_usize(x, nb_col);
    let distance = |a: usize, b: usize| {let (x1, y1) = to_coord(a); let (x2, y2) = to_coord(b); ((x2 - x1).pow(2) as f32 + (y2 - y1).pow(2) as f32).sqrt()};
    //println!("to_idx(3, 2) -> {}", to_idx(3, 2));
    //println!("to_coord(23) -> {:?}", to_coord(23));

    let mut basins: HashMap<usize, Vec<usize>> = HashMap::new();

    let idx: usize = 3;
    //for idx in 0..=indexes.len()-1 {
//
    //}
    let mut basin1: Vec<usize> = [indexes[idx]].to_vec();
    println!("basin1 : {:?}", basin1);

    let mut position: usize = indexes[idx];
    let mut start_position: usize = indexes[idx];
    //let mut count:usize = 0;
    let mut tried_basin_position: usize = 0;
    loop {
        let (x, y) = to_coord(position);
        let val: u32 = data[position];
        // go left if x != 0
        if x > 0 && (data[position-1] > val || distance(position-1, start_position) < distance(position, start_position)) && data[position-1] != 9 && !basin1.iter().any(|&i| i==position-1) {
            println!("Go left");
            println!("x: {} - y: {} - val: {}", x-1, y, val);
            basin1.push(position-1);
            position = position - 1;
            tried_basin_position=basin1.len();
        }
        // go right
        else if x < nb_col-1 && (data[position+1] > val || distance(position+1, start_position) < distance(position, start_position)) && data[position+1] != 9 && !basin1.iter().any(|&i| i==position+1) {
            println!("Go right");
            println!("x: {} - y: {} - val: {}", x+1, y, val);
            basin1.push(position+1);
            position = position + 1;
            tried_basin_position=basin1.len();
        }
        // go up
        else if y > 0 && (data[to_idx(x,y-1)] > val || distance(to_idx(x,y-1), start_position) < distance(position, start_position)) && data[to_idx(x,y-1)] != 9 && !basin1.iter().any(|&i| i==to_idx(x,y-1)) {
            println!("Go up");
            println!("x: {} - y: {} - val: {}", x, y-1, val);
            basin1.push(to_idx(x,y-1));
            position = to_idx(x,y-1);
            tried_basin_position=basin1.len();
        }
        // go down
        else if y < nb_row-1 && (data[to_idx(x,y+1)] > val || distance(to_idx(x,y+1), start_position) < distance(position, start_position)) && data[to_idx(x,y+1)] != 9 && !basin1.iter().any(|&i| i==to_idx(x,y+1)) {
            println!("Go down");
            println!("x: {} - y: {} - val: {}", x, y+1, val);
            basin1.push(to_idx(x,y+1));
            position = to_idx(x,y+1);
            tried_basin_position=basin1.len();
        }
        //else { position = start_position; }
        else {
            if tried_basin_position > 0 {
                tried_basin_position-=1;
                position=basin1[tried_basin_position];
                println!("Go previous position : {}", position);
                println!("tried_basin_position : {}", tried_basin_position);
            }
            else { break; }
            //position=basin1[basin1.len()-2];
        }
        //count+=1;
        //if count > 20 { break; }
    }
    println!("basin1 : {:?}", basin1);


    return String::from("Exit");
}