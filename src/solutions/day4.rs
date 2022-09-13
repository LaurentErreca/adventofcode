use std::collections::HashSet;


#[derive(Debug)]
#[derive(Clone)]
struct Board {
    board: Vec<Vec<i32>>
}

impl Default for Board {
    fn default() -> Board {
        Board{board: vec![]}
    }
}

impl Board {
    fn get_board_values(&self) -> HashSet<i32> {
        let mut values: Vec<i32> = vec![];
        for row in 0..self.board.len() {
            values.extend(&self.board[row]);
        }
        let hs_values: HashSet<i32> = values.into_iter().collect();
        return hs_values;
    }

    fn bingo_in_row(&self, numbers: &Vec<i32>) -> i32 {
        let local_numbers = numbers.clone();
        let set_numbers: HashSet<i32> = local_numbers.clone().into_iter().collect();
        for row in 0..self.board.len() {
            let tmp: Vec<i32> = self.board[row].clone();
            let set_board_row: HashSet<i32> = tmp.into_iter().collect(); //HashSet::from(self.board[row]);
            let result = set_board_row.difference(&set_numbers).collect::<Vec<&i32>>();
            if result.len() == 0 {
                //println!("Board row : {:?}", set_board_row);
                //println!("Numbers : {:?}", set_numbers);
                //println!("Bingo with board {:?} !!!", self.board);
                //return 1;
                let board_values = self.get_board_values();
                return local_numbers[local_numbers.len() - 1] * board_values.difference(&set_numbers).sum::<i32>();
                //return numbers[numbers.len() - 1] * board_values.difference(&set_numbers).collect::<Vec<&i32>>().iter().sum::<i32>();
            }
        }
        return -1;
    }

    fn bingo_in_column(&self, numbers: &Vec<i32>) -> i32 {
        let local_numbers = numbers.clone();
        let set_numbers: HashSet<i32> = local_numbers.clone().into_iter().collect();
        for col in 0..self.board[0].len() {
            let tmp: Vec<i32> = self.board.iter().map(|v| v[col]).collect();
            //println!("col : {:?}", tmp);
            let set_board_row: HashSet<i32> = tmp.into_iter().collect(); //HashSet::from(self.board[row]);
            let result = set_board_row.difference(&set_numbers).collect::<Vec<&i32>>();
            if result.len() == 0 {
                //println!("Bingo with board {:?} !!!", self.board);
                //return 1;
                let board_values = self.get_board_values();
                return local_numbers[local_numbers.len() - 1] * board_values.difference(&set_numbers).sum::<i32>();
                //return numbers[numbers.len() - 1] * board_values.difference(&set_numbers).collect::<Vec<&i32>>().iter().sum::<i32>();
            }
        }
        return -1;
    }

    fn bingo_in_diagonal(&self, numbers: &Vec<i32>) -> i32 {
        let local_numbers = numbers.clone();
        let set_numbers: HashSet<i32> = local_numbers.clone().into_iter().collect();
        let mut diagonal: Vec<i32> = vec![];
        for col in 0..self.board[0].len() {
            diagonal.push(self.board[col][col]);
        }
            let set_board_row: HashSet<i32> = diagonal.into_iter().collect(); //HashSet::from(self.board[row]);
            let result = set_board_row.difference(&set_numbers).collect::<Vec<&i32>>();
            if result.len() == 0 {
                //println!("Bingo with board {:?} !!!", self.board);
                //return 1;
                let board_values = self.get_board_values();
                return local_numbers[local_numbers.len() - 1] * board_values.difference(&set_numbers).sum::<i32>();
                //return numbers[numbers.len() - 1] * board_values.difference(&set_numbers).collect::<Vec<&i32>>().iter().sum::<i32>();
            }
        return -1;
    }

    fn bingo(&self, numbers: &Vec<i32>) -> i32 {
        if self.bingo_in_row(numbers) != -1 {
            //println!("Bingo in row");
            return self.bingo_in_row(numbers);
        }
        if self.bingo_in_column(numbers) != -1 {
            //println!("Bingo in column");
            return self.bingo_in_column(numbers);
        }
        if self.bingo_in_diagonal(numbers) != -1 {
            //println!("Bingo in diagonal");
            return self.bingo_in_diagonal(numbers);
        }
        return -1;
    }
}

fn get_boards(data: &Vec<&str>) -> Vec<Board> {
    let mut output: Vec<Board> = vec![];
    let mut myvec: Vec<i32>;
    let mut vectors: Vec<Vec<i32>> = vec![];
    for nb in 2..data.len(){
        if data[nb] != "" {
            //println!("Row : {:?}", data[nb]);
            //println!("Row : {:?}", data[nb].split_whitespace().collect::<Vec<&str>>());
            myvec = data[nb].split_whitespace().map(|c| c.parse().unwrap()).collect();
            vectors.push(myvec);
        }
        else{
            output.push(Board{board: vectors});
            vectors = vec![];
        }
    }
    // Add last board
    output.push(Board{board: vectors});
    return output;
}


pub fn solve(part: u8, input: &String) -> String {
    let vecstr: Vec<&str> = input.lines().collect();
    let rnd_numbers: Vec<i32> = vecstr[0].split(",").map(|c| c.parse().unwrap()).collect::<Vec<i32>>();
    //println!("Random numbers : {:?}", rnd_numbers);
    let mut boards = get_boards(&vecstr);
    //println!("Number of boards : {:?}", boards.len());
    let mut res: i32 = 0;
    if part == 1 {
        for nb in 0..rnd_numbers.len() {
            for board in &boards {
                //println!("{:?}", board);
                let tmp_vec: Vec<i32> = rnd_numbers[..nb].to_vec().clone();
                res = board.bingo(&tmp_vec);
                if res != -1 {
                    //println!("Nb iter : {:?}", nb);
                    let last_value: i32 = rnd_numbers[nb-1];
                    //println!("Last value {:?}", last_value);
                    println!("Result {:?}", res);
                    return String::from("Exit");
                }
            }
        }
    }
    let mut completed_boards: Vec<usize> = vec![];
    if part == 2 {
        let nb_boards: usize = boards.len();
        let mut nb_res: usize = 0;
        for nb in 0..rnd_numbers.len() {
            for (i, board) in boards.iter().enumerate() {
                if !completed_boards.contains(&i) {
                    let tmp_vec: Vec<i32> = rnd_numbers[..nb].to_vec().clone();
                    res = board.bingo(&tmp_vec);
                    if res != -1 {
                        completed_boards.push(i);
                        println!("Removed boards {:?}", i);
                        nb_res = nb_res + 1;
                        println!("Nb res : {:?}", nb_res);
                        if nb_res == nb_boards {
                            println!("Nb iter : {:?}", nb);
                            let last_value: i32 = rnd_numbers[nb-1];
                            println!("Last value {:?}", last_value);
                            println!("Result {:?}", res);
                            return String::from("Exit");
                        }
                    }
                }
            }
        }
    }
    return String::from("part must be 1 or 2");
}