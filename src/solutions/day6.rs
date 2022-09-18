
fn compute_nb_fishs(nb_days: i32, init_value: i32) -> i64 {
    let mut arr: Vec<i64> = vec![0; 9];
    arr[(init_value-1) as usize] = 1;
    for day in 1..nb_days {
        let nb_zeros_days: i64 = arr[0];
        let carr = arr.clone();
        for (i, x) in arr.clone().iter().enumerate(){
            arr[i] = carr[(i + 1) % arr.len()];
        }
        arr[6] = arr[6]+nb_zeros_days;
    }
    return arr.iter().sum::<i64>();
}


pub fn solve(part: u8, input: &String) -> String {
    let nb_days: i32 = if part == 1 {80}else if part == 2 {256}else{0};
    let initial_state: Vec<&str> = input.lines().collect();
    let mut v_fish: Vec<i32> = initial_state[0].split(",").map(|c| c.parse().unwrap()).collect::<Vec<i32>>();
    let result = v_fish.iter().map(|x| compute_nb_fishs(nb_days, *x)).sum::<i64>();
    println!("Result : {}", result);
    return String::from("Exit");
}