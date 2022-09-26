use std::collections::HashMap;
use std::iter;

//   aaaa
//  b    c
//  b    c
//   dddd
//  e    f
//  e    f
//   gggg

fn find_a(a: &str, b: &str) -> char {

    for c in b.chars(){
        if !a.contains(c) {return c;}
    }
    for c in a.chars(){
        if !b.contains(c) {return c;}
    }
    return 'z';
}

fn find_three(five_letters: &str, one: &str) -> char {

    for c in one.chars(){
        if !five_letters.contains(c) {return c;}
    }
    return 'z';
}

fn find_e(eight: &str, one: &str, four: &str, seven: &str, g: char) -> char {
    for c in eight.chars() {
        if !one.contains(c) & !four.contains(c) & !seven.contains(c) & (g != c) {return c;}
    }
    return 'z';
}

fn find_b(one: &str, four: &str, five_letters: &Vec<&str>) -> char {
    for c in four.chars(){
        if five_letters[0].contains(c) & !five_letters[1].contains(c) & !five_letters[2].contains(c) & !one.contains(c){ return c; }
        if five_letters[1].contains(c) & !five_letters[0].contains(c) & !five_letters[2].contains(c) & !one.contains(c){ return c; }
        if five_letters[2].contains(c) & !five_letters[1].contains(c) & !five_letters[0].contains(c) & !one.contains(c){ return c; }
    }
    return 'z';
}

fn find_c(one: &str, f: char) -> char {
    for c in one.chars() {
        if c != f { return c; }
    }
    return 'z';
}

fn find_f(five_letters: &Vec<&str>, one: &str, four: &str, b: char) -> char {
    for fi in five_letters {
        if fi.contains(b) & four.contains(b) {
            for c in fi.chars(){
                if one.contains(c) { return c; }
            }
        }
    }
    return 'z';
}

fn find_d(eight: &str, a: char, g: char, e: char, b: char, f: char, c: char) -> char {
    for car in eight.chars(){
        if (car != a) & (car != g) & (car != e) & (car != b) & (car != f) & (car != c) { return car; }
    }
    return 'z';
}

fn same_letters(a: &str, b: &str) -> bool {
    if a.len() != b.len() {return false;}
    for l in a.chars(){
        if !b.contains(l) {return false;}
    }
    for l in b.chars(){
        if !a.contains(l) {return false;}
    }
    return true;
}

fn concat_new(vec: &[u32]) -> u32 {
    let t = vec.iter().fold("".to_string(), |acc, x| acc + &x.to_string());
    t.parse::<u32>().unwrap()
}

//fn decode_inputs(input: Vec<&str>) -> HashMap {
fn decode_inputs(input: Vec<&str>, output: Vec<&str>) -> i32 {
    let mut a: char = 'x';
    let mut b: char = 'x';
    let mut c: char = 'x';
    let mut d: char = 'x';
    let mut e: char = 'x';
    let mut f: char = 'x';
    let mut g: char = 'x';
    let mut zero: &str = "";
    let mut one: &str = "";
    let mut two: &str = "";
    let mut three: &str = "";
    let mut four: &str = "";
    let mut five: &str = "";
    let mut six: &str = "";
    let mut seven: &str = "";
    let mut eight: &str = "";
    let mut nine: &str = "";
    //println!("input = {:?}", &input);
    // find a
    for code in &input {
        //println!("code = {:?}", &code);
        //println!("len code : {:?}", code.len());
        if code.len() == 2 { one = code.clone(); /* println!("one : {:?}", one); */ }
        if code.len() == 4 { four = code.clone(); /* println!("four : {:?}", four); */ }
        if code.len() == 3 { seven = code.clone(); /* println!("seven : {:?}", seven); */ }
        if code.len() == 7 { eight = code.clone(); /* println!("eight : {:?}", eight); */ }
    }
    //println!("one: {} - seven : {}", one, seven);
    let a = find_a(one, seven);
    //println!("a = {:?}", &a);
    // find three
    for code in &input {
        if code.len() == 5 && find_three(code, one) == 'z' {
            three = code;
        }
    }
    //println!("three = {:?}", &three);
    // find g
    // keep 2, 3 and 5
    let mut li: Vec<&str> = vec![];
    for code in &input {
        if code.len() == 5 {li.push(code);}
    }
    //println!("Five letters : {:?}", li);
    let mut tmp: char = 'z';
    // Dans 3, le segment qui est pas dans 1, 4, ou 7 est le g
    for el in three.chars() {
        tmp = el;
        if !one.contains(&el.to_string()) & !four.contains(&el.to_string()) & !seven.contains(&el.to_string()) { g = tmp; }
    }
    //println!("g : {}", g);
    // finde e
    e = find_e(eight, one, four, seven, g);
    //println!("e : {}", e);

    //find b
    b = find_b(one, four, &li);
    //println!("b : {}", b);

    // find f
    f = find_f(&li, one, four, b);
    //println!("f : {}", f);

    // find c
    c = find_c(one, f);
    //println!("c : {}", c);

    // find d
    d = find_d(eight, a, g, e, b, f, c);
    //println!("d : {}", d);

    // zero
    let ref_zero = format!("{}{}{}{}{}{}", a, b, c, e, f, g);
    zero = &ref_zero;
    //zero = &format!("{}{}{}{}{}{}", a, b, c, e, f, g).to_string();
    // two
    let ref_two = format!("{}{}{}{}{}", a, c, d, e, g);
    two = &ref_two;
    // three
    let ref_three = format!("{}{}{}{}{}", a, c, d, f, g);
    three = &ref_three;
    // five
    let ref_five = format!("{}{}{}{}{}", a, b, d, f, g);
    five = &ref_five;
    // six
    let ref_six = format!("{}{}{}{}{}{}", a, b, d, e, f, g);
    six = &ref_six;
    // nine
    let ref_nine = format!("{}{}{}{}{}{}", a, b, c, d, f, g);
    nine = &ref_nine;
    //println!("{}-{}-{}-{}-{}-{}-{}-{}-{}-{}", zero, one, two, three, four, five, six, seven, eight, nine);

    //println!("output : {:?}", output);

    let mut res_out: Vec<u32> = vec![];
    for out in output {
        if same_letters(out, zero) {res_out.push(0);}
        if same_letters(out, one) {res_out.push(1);}
        if same_letters(out, two) {res_out.push(2);}
        if same_letters(out, three) {res_out.push(3);}
        if same_letters(out, four) {res_out.push(4);}
        if same_letters(out, five) {res_out.push(5);}
        if same_letters(out, six) {res_out.push(6);}
        if same_letters(out, seven) {res_out.push(7);}
        if same_letters(out, eight) {res_out.push(8);}
        if same_letters(out, nine) {res_out.push(9);}
    }
    //println!("res_out : {:?}", res_out);
    //println!("{:?} --> {:?}", res_out, concat_new(res_out.as_slice()));
    return concat_new(res_out.as_slice()) as i32;
}

pub fn solve(part: u8, input: &String) -> String {

    if part == 1 {
    
        let nb_digits: usize = input
            .lines()
            .map(|x| {
                let mut it = x.split(" | ");
                it.next();
                return it.next().unwrap().split(" ")
                }
            )
            .flatten()
            .map(|x| x.len())
            .filter(|x| { *x==2 || *x==3 || *x==4 || *x==7})
            .collect::<Vec<usize>>()
            .len();

        println!("Result : {:?}", nb_digits);
    }
    if part == 2 {
        let mut res: Vec<i32> = vec![];
        for li in input.lines(){
            let mut splt_line = li.split(" | ");
            let input_code = splt_line.next().unwrap();
            let output_code = splt_line.next().unwrap();
            res.push(decode_inputs(input_code.split(" ").collect::<Vec<&str>>(), output_code.split(" ").collect::<Vec<&str>>()));
        }
        println!("Result : {:?}", res.iter().sum::<i32>());
    }
    return String::from("Exit");
}