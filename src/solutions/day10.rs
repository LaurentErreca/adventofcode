use std::collections::HashMap;

fn check_line(line: &str) -> i32 {
    let mut nb_open_1: i32 = 0; // for character : (
    let mut nb_open_2: i32 = 0; // for character : [
    let mut nb_open_3: i32 = 0; // for character : {
    let mut nb_open_4: i32 = 0; // for character : <

    let score: HashMap<char, i32> = HashMap::from([
        (')', 3),
        (']', 57),
        ('}', 1197),
        ('>', 25137),
    ]);

    let value: HashMap<char, i32> = HashMap::from([
        (')', -1),
        (']', -1),
        ('}', -1),
        ('>', -1),
        ('(', 1),
        ('[', 1),
        ('{', 1),
        ('<', 1),
    ]);

    let mut last_opened: Vec<char> = vec![];

    //println!("Line : {}", line);
    for el in line.chars(){
        //println!("el : {}", el);
        if el == '(' || el == ')' {
            nb_open_1 = nb_open_1 + value.get(&el).unwrap();
            if nb_open_1 < 0 {
                //println!("Illegal ( in {}", line);
                return *score.get(&el).unwrap();
            }
            if el == '(' { last_opened.push('('); }
            else if el == ')' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] != '(' {
                //println!("Illegal ) in {}", line);
                return *score.get(&el).unwrap();
            }
            else if el == ')' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] == '(' {
                last_opened.pop();
            }
        }
        if el == '[' || el == ']' {
            nb_open_2 = nb_open_2 + value.get(&el).unwrap(); if nb_open_2 < 0 {
                //println!("Illegal ] in {}", line);
                return *score.get(&el).unwrap();
            }
            if el == '[' { last_opened.push('['); }
            else if el == ']' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] != '[' {
                //println!("Illegal ] in {}", line);
                return *score.get(&el).unwrap();
            }
            else if el == ']' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] == '[' {
                last_opened.pop();
            }
        }
        if el == '{' || el == '}' {
            nb_open_3 = nb_open_3 + value.get(&el).unwrap();
            if nb_open_3 < 0 {
                //println!("Illegal }} in {}", line);
                return *score.get(&el).unwrap();
            }
            if el == '{' { last_opened.push('{'); }
            else if el == '}' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] != '{' {
                //println!("Illegal }} in {}", line);
                return *score.get(&el).unwrap();
            }
            else if el == '}' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] == '{' {
                last_opened.pop();
            }
        }
        if el == '<' || el == '>' {
            nb_open_4 = nb_open_4 + value.get(&el).unwrap();
            if nb_open_4 < 0 {
                //println!("Illegal < in {}", line);
                return *score.get(&el).unwrap();
            }
            if el == '<' { last_opened.push('<'); }
            else if el == '>' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] != '<' {
                //println!("Illegal > in {}", line);
                return *score.get(&el).unwrap();
            }
            else if el == '>' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] == '<' {
                last_opened.pop();
            }
        }
        
        //println!("nb_open_1 : {}", nb_open_1);
        //println!("nb_open_2 : {}", nb_open_2);
        //println!("nb_open_3 : {}", nb_open_3);
        //println!("nb_open_4 : {}", nb_open_4);
    }
    return 0;
}

/*
  ): 3 points.
  ]: 57 points.
  }: 1197 points.
  >: 25137 points.
*/

pub fn solve(part: u8, input: &String) -> String {
    let mut total_score: i32 = 0;
    for li in input.lines() {
        total_score += check_line(li);
    }
    println!("Total score : {}", total_score);
    return String::from("Exit");
}