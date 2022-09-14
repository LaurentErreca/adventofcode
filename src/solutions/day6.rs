
fn get_nb_fish(fishs: &Vec<i32>, nbdays: i32) -> i32 {
    let mut result = fishs.clone();
    for day in 1..nbdays {
        let v_fish_copy = result.clone();
        for (idx, cnt) in v_fish_copy.iter().enumerate() {
            if *cnt == 0 {
                result[idx] = 6;
                result.push(8);
            }
            else {
                result[idx] = result[idx] - 1;
            }
        }
    }
    return result.len().try_into().unwrap();
}


pub fn solve(part: u8, input: &String) -> String {
    let nb_days: i32 = if part == 1 {81}else if part == 2 {257}else{0};
    let initial_state: Vec<&str> = input.lines().collect();
    let mut v_fish: Vec<i32> = initial_state[0].split(",").map(|c| c.parse().unwrap()).collect::<Vec<i32>>();
    let result = get_nb_fish(&v_fish, nb_days);
    println!("Result : {}", result);
    return String::from("Exit");
}