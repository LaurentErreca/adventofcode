
fn get_most_common(data: &Vec<Vec<char>>) -> Vec<char> {
    let mut output: Vec<char> = vec!['0'; data[0].len()];
    for i in 0..data[0].len() {
        let mut zeros: i32 = 0;
        let mut ones: i32 = 0;
        for chars in data {
            if chars[i] == '0' {
                zeros += 1;
            }
            else {
                ones += 1;
            }
        }
        if zeros > ones {
            output[i] = '0'
        }
        else {
            output[i] = '1'
        }
    }
    return output;
}

fn get_opposite_chars(chars: &Vec<char>) -> Vec<char> {
    let mut output: Vec<char> = vec!['0'; chars.len()];
    for (ind, i) in chars.iter().enumerate() {
        match i {
            '0' => output[ind] = '1',
            '1' => output[ind] = '0',
            _   => println!("Wrong char value"),
        }
    }
    return output;
}

fn convert_to_decimal(vector_of_int: Vec<u32>) -> u32 {
    vector_of_int
        .into_iter()
        .rev()
        .enumerate()
        .map(|(place, bit)| bit << place)
        .sum()
}

pub fn solve(part: u8, input: &String) -> String {
    let vecstr: Vec<&str> = input.lines().collect();
    let vecstr_iter = vecstr.iter();
    let mut chars: Vec<Vec<char>> = Vec::new();
    if part == 1 {
        for bits in vecstr_iter {
            chars.push(bits.chars().collect());
        }
        /* println!("Nb vec chars : {}", chars.len());
        println!("First vector of chars : {:?}", chars[0]);
        println!("First char of first vector of chars : {:?}", chars[0][0]);
        println!("Last char of first vector of chars : {:?}", chars[0][chars[0].len()-1]);
        println!("Most common : {:?}", get_most_common(&chars));
        println!("Opposite most common : {:?}", get_opposite_chars(&get_most_common(&chars))); */
        let most_common_int: Vec<u32> = get_most_common(&chars).iter().map(|x| x.to_digit(10).unwrap()).collect();
        let less_common_int: Vec<u32> = get_opposite_chars(&get_most_common(&chars)).iter().map(|x| x.to_digit(10).unwrap()).collect();
        //println!("Most common int : {:?}", most_common_int);
        //println!("Less common int : {:?}", less_common_int);
        let decimal_most_common: u32 = convert_to_decimal(most_common_int);
        let decimal_less_common: u32 = convert_to_decimal(less_common_int);
        //println!("Decimal most common : {:?}", decimal_most_common);
        //println!("Decimal less common : {:?}", decimal_less_common);
        return (decimal_less_common * decimal_most_common).to_string();
    }

    if part == 2 {
        println!("No part 2 !");
    }
    return String::from("part must be 1");
}
