fn get_fuel_consumption(array: &Vec<i32>, pos: i32, part: u8) -> i32 {
    if part == 1 {
        return array.iter().map(|x| i32::abs(x-pos)).sum::<i32>();
    }
    else if part == 2 {
        return array.iter().map(|x| (1..i32::abs(x-pos)+1).sum::<i32>()).sum::<i32>();
    }
    else {
        return 0;
    }
}

fn get_best_position(array: &Vec<i32>, part: u8) -> i32 {
    let minp: i32 = *array.iter().min().unwrap();
    let maxp: i32 = *array.iter().max().unwrap();
    let diff = maxp-minp;
    let mut sol: Vec<i32> = vec![0; (diff+1) as usize];
    for i in minp..maxp+1 {
        sol[i as usize] = get_fuel_consumption(&array, i, part);
    }
    let min_value = *sol.iter().min().unwrap();
    return min_value;
}


pub fn solve(part: u8, input: &String) -> String {
    let positions: Vec<i32> = input.lines().collect::<Vec<&str>>()[0].split(",").map(|c| c.parse().unwrap()).collect::<Vec<i32>>();
    let result = get_best_position(&positions, part);
    println!("Result : {}", result);
    return String::from("Exit");
}