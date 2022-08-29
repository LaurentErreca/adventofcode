
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

fn filter_common(data: &Vec<Vec<char>>, position: &usize, rating: &String) -> Vec<Vec<char>> {
    let mut ones: u32 = 0;
    let mut zeros: u32 = 0;
    let mut common: char = '1';
    let mut output: Vec<Vec<char>> = Vec::with_capacity(0);
    for chars in data {
        if chars[*position] == '1' {
            ones += 1;
        }
        else {
            zeros += 1;
        }
    }
    if rating == "oxygen" {
        if ones >= zeros {
            common = '1';
        }
        else {
            common = '0';
        }
    }
    if rating == "co2" {
        if zeros <= ones {
            common = '0';
        }
        else {
            common = '1';
        }
    }
    for char in data {
        if char[*position] == common {
            output.push(char.to_vec());
        }
    }
    return output
}

pub fn solve(part: u8, input: &String) -> String {
    let vecstr: Vec<&str> = input.lines().collect();
    let vecstr_iter = vecstr.iter();
    let mut chars: Vec<Vec<char>> = Vec::new();
    if part == 1 {
        for bits in vecstr_iter {
            chars.push(bits.chars().collect());
        }

        let most_common_int: Vec<u32> = get_most_common(&chars).iter().map(|x| x.to_digit(10).unwrap()).collect();
        let less_common_int: Vec<u32> = get_opposite_chars(&get_most_common(&chars)).iter().map(|x| x.to_digit(10).unwrap()).collect();
        println!("Most common int : {:?}", most_common_int);
        println!("Less common int : {:?}", less_common_int);
        let decimal_most_common: u32 = convert_to_decimal(most_common_int);
        let decimal_less_common: u32 = convert_to_decimal(less_common_int);
        return (decimal_less_common * decimal_most_common).to_string();
    }

    let mut out_ox: Vec<Vec<char>> = Vec::new();
    let mut out_co: Vec<Vec<char>> = Vec::new();
    if part == 2 {
        for bits in vecstr_iter {
            out_ox.push(bits.chars().collect());
            out_co.push(bits.chars().collect());
        }
        for position in 0..out_ox[0].len() {
            if out_ox.len() == 1 {
                break;
            }
            out_ox = filter_common(&out_ox, &position, &String::from("oxygen"));
        }
        for position in 0..out_co[0].len() {
            if out_co.len() == 1 {
                break;
            }
            out_co = filter_common(&out_co, &position, &String::from("co2"));
        }

        let ox_rating: u32 = convert_to_decimal(out_ox[0].iter().map(|x| x.to_digit(10).unwrap()).collect());
        let co_rating: u32 = convert_to_decimal(out_co[0].iter().map(|x| x.to_digit(10).unwrap()).collect());
        println!("ox vector : {:?}", out_ox[0]);
        println!("co vector : {:?}", out_co[0]);
        return (ox_rating * co_rating).to_string();
    }
    return String::from("part must be 1 or 2");
}
