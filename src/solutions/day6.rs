

pub fn solve(part: u8, input: &String) -> String {
    let nb_days: i32 = if part == 1 {81}else if part == 2 {257}else{0};
    let initial_state: Vec<&str> = input.lines().collect();
    let mut v_fish: Vec<i32> = initial_state[0].split(",").map(|c| c.parse().unwrap()).collect::<Vec<i32>>();
    for day in 1..nb_days {
        let v_fish_copy = v_fish.clone();
        for (idx, cnt) in v_fish_copy.iter().enumerate() {
            if *cnt == 0 {
                v_fish[idx] = 6;
                v_fish.push(8);
            }
            else {
                v_fish[idx] = v_fish[idx] - 1;
            }
        }
    }
    println!("{}", v_fish.len());
    return String::from("Exit");
}