use std::collections::HashMap;

fn check_line(line: &str, nb_corrupted: &mut i32) -> (i32, i64) {
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

    for el in line.chars(){
        if el == '(' || el == ')' {
            nb_open_1 = nb_open_1 + value.get(&el).unwrap();
            if nb_open_1 < 0 {
                *nb_corrupted += 1;
                return (*score.get(&el).unwrap(), 0);
            }
            if el == '(' { last_opened.push('('); }
            else if el == ')' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] != '(' {
                *nb_corrupted += 1;
                return (*score.get(&el).unwrap(), 0);
            }
            else if el == ')' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] == '(' {
                last_opened.pop();
            }
        }
        if el == '[' || el == ']' {
            nb_open_2 = nb_open_2 + value.get(&el).unwrap(); if nb_open_2 < 0 {
                *nb_corrupted += 1;
                return (*score.get(&el).unwrap(), 0);
            }
            if el == '[' { last_opened.push('['); }
            else if el == ']' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] != '[' {
                *nb_corrupted += 1;
                return (*score.get(&el).unwrap(), 0);
            }
            else if el == ']' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] == '[' {
                last_opened.pop();
            }
        }
        if el == '{' || el == '}' {
            nb_open_3 = nb_open_3 + value.get(&el).unwrap();
            if nb_open_3 < 0 {
                *nb_corrupted += 1;
                return (*score.get(&el).unwrap(), 0);
            }
            if el == '{' { last_opened.push('{'); }
            else if el == '}' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] != '{' {
                *nb_corrupted += 1;
                return (*score.get(&el).unwrap(), 0);
            }
            else if el == '}' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] == '{' {
                last_opened.pop();
            }
        }
        if el == '<' || el == '>' {
            nb_open_4 = nb_open_4 + value.get(&el).unwrap();
            if nb_open_4 < 0 {
                *nb_corrupted += 1;
                return (*score.get(&el).unwrap(), 0);
            }
            if el == '<' { last_opened.push('<'); }
            else if el == '>' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] != '<' {
                *nb_corrupted += 1;
                return (*score.get(&el).unwrap(), 0);
            }
            else if el == '>' && last_opened.len() > 0 &&  last_opened[last_opened.len()-1] == '<' {
                last_opened.pop();
            }
        }
    }
    let mut total_score_part2: i64 = 0;
    for car in last_opened.iter().rev() {
        if *car == '(' { total_score_part2 = 5 * total_score_part2 + 1; }
        if *car == '[' { total_score_part2 = 5 * total_score_part2 + 2; }
        if *car == '{' { total_score_part2 = 5 * total_score_part2 + 3; }
        if *car == '<' { total_score_part2 = 5 * total_score_part2 + 4; }
    }
    return (0, total_score_part2);
}

pub fn solve(part: u8, input: &String) -> String {
    let mut total_score: i32 = 0;
    let mut scores_part2: Vec<i64> = vec![];
    let mut nb_corrupted: i32 = 0;
    let mut nb_lines: i32 = 0;
    for li in input.lines() {
        nb_lines += 1;
        let (score, score_part_2) = check_line(li, &mut nb_corrupted);
        total_score += score;
        if score == 0 { scores_part2.push(score_part_2); }
    }
    println!("Nb total lines : {}", nb_lines);
    println!("Nb corrupted lines : {}", nb_corrupted);
    println!("Part 1 : Total score : {}", total_score);
    if part == 1 { return String::from("Exit"); }
    scores_part2.sort_by(|a, b| a.cmp(b));
    println!("Scores part 2 : {:?}", scores_part2);
    println!("Len scores_part2 : {}", scores_part2.len());
    println!("Part 2 : {}", scores_part2[scores_part2.len() / 2]);
    return String::from("Exit");
}