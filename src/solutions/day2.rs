
fn apply_forward(hpos: &i32, aim: &i32, value: &i32, depth: &i32) -> (i32, i32) {
    return (hpos + value, depth + (aim * value));
}

pub fn solve(part: u8, input: &String) -> String {
    let mut hpos: i32 = 0;
    let mut depth: i32 = 0;
    let mut aim: i32 = 0;
    let vecstr: Vec<&str> = input.lines().collect();
    let vecstr_iter = vecstr.iter();
    if part == 1 {
        for command in vecstr_iter {
            let mut split = command.split(" ");
            let command_tuple = (split.next().unwrap(), split.next().unwrap());
            let action = command_tuple.0;
            let value: i32 = command_tuple.1.parse().unwrap();
            match action {
                "up" => depth -= value,
                "down" => depth += value,
                "forward" => hpos += value,
                _ => println!("Wrong action"),
            }
        }
        println!("depth : {} - hpos : {}", depth, hpos);
        return (depth * hpos).to_string();
    }
    if part == 2 {
        for command in vecstr_iter {
            let mut split = command.split(" ");
            let command_tuple = (split.next().unwrap(), split.next().unwrap());
            let action = command_tuple.0;
            let value: i32 = command_tuple.1.parse().unwrap();
            match action {
                "up" => aim -= value,
                "down" => aim += value,
                "forward" => (hpos, depth) = apply_forward(&hpos, &aim, &value, &depth), //hpos += value,// depth = aim * value,
                _ => println!("Wrong action"),
            }
        }
        println!("depth : {} - hpos : {}", depth, hpos);
        return (depth * hpos).to_string();
    }
    return String::from("part must be 1");
}
