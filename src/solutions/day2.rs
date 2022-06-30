
pub fn solve(part: u8, input: &String) -> String {
    let mut hpos: i32 = 0;
    let mut depth: i32 = 0;
    let vecstr: Vec<&str> = input.lines().collect();
    //let vecint: Vec<i32> = vecstr.into_iter().map(|x| x.parse().unwrap()).collect();
    let vecstr_iter = vecstr.iter();
    if part == 1 {
        for command in vecstr_iter {
            let mut split = command.split(" ");
            let command_tuple = (split.next().unwrap(), split.next().unwrap());
            let action = command_tuple.0;
            let value: i32 = command_tuple.1.parse().unwrap();
            //println!("action : {} - value : {}", action, value);
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
        return String::from("part must be 1");
    }
    return String::from("part must be 1");
}
