fn get_lowest_indexes(input: &String) -> (Vec<u32>, Vec<usize>) {
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

pub fn solve(part: u8, input: &String) -> String {
    println!("Start day 9");
    // get_lowest_indexes returns data as a vector of u32 and indexes of lowest values as vector of usize
    let (data, indexes) = get_lowest_indexes(input);

    let result: u32 = indexes.iter().map(|x| data[*x]+1).sum::<u32>();
    println!("Result : {}", result);

    return String::from("Exit");
}