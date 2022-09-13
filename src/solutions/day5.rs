use std::cmp;
use std::num;


fn get_coord(x: &str) -> Vec<Vec<i32>> {
    let mut splt = x.split(" -> ");
    let p1 = splt.next().unwrap();
    let mut spl_p1 = p1.split(',');
    let (x1, y1)= (spl_p1.next().unwrap().parse().unwrap(), spl_p1.next().unwrap().parse().unwrap());
    let p2 = splt.next().unwrap();
    let mut spl_p2 = p2.split(',');
    let (x2, y2) = (spl_p2.next().unwrap().parse().unwrap(), spl_p2.next().unwrap().parse().unwrap());
    let res: Vec<Vec<i32>> = vec![vec![x1, y1], vec![x2, y2]];
    return res;

}

fn get_sizes(coord: &Vec<Vec<Vec<i32>>>) -> (i32, i32) {
    let mut xmax: i32 = 0;
    let mut xmin: i32 = 10000;
    let mut ymax: i32 = 0;
    let mut ymin: i32 = 10000;
    for line in coord {
        for point in line {
            if point[0] > xmax {
                xmax = point[0];
            }
            if point[0] < xmin {
                xmin = point[0];
            }
            if point[1] > ymax {
                ymax = point[1];
            }
            if point[1] < ymin {
                ymin = point[1];
            }
        }
    }
    (xmax+1, ymax+1)
}

pub fn solve(part: u8, input: &String) -> String {
    if part == 1 {
        let vecstr: Vec<&str> = input.lines().collect();
        let out: Vec<i32> = vec![];
        let mut res: Vec<Vec<Vec<i32>>> = vec![];
        for i in vecstr
        .iter()
        .map(|x| get_coord(x))
        .filter(|x| x[0][0]==x[1][0] || x[0][1]==x[1][1]) {
            res.push(i);
        }
        let (xsize, ysize) = get_sizes(&res);

        let mut vector: Vec<i32> = vec![0; (xsize*ysize).try_into().unwrap()];
        println!("Dim : {} - {}", xsize, ysize);
        println!("VEctor length : {}", vector.len());
        println!("Vector size : {}", xsize*ysize);

        for line in res {
            if line[0][0] == line[1][0] {
                // vertical line
                let min_val = cmp::min(line[0][1], line[1][1]);
                let max_val = cmp::max(line[0][1], line[1][1]);
                for x in min_val..max_val+1 {
                    //println!("{} - {}", x, (x*xsize)+line[0][0]);
                    vector[((x*xsize)+line[0][0]) as usize] = vector[((x*xsize)+line[0][0]) as usize] + 1;
                }
            }
            if line[0][1] == line[1][1] {
                // horizontal line
                let min_val = cmp::min(line[0][0], line[1][0]);
                let max_val = cmp::max(line[0][0], line[1][0]);
                for x in min_val..max_val+1 {
                    vector[(x+(line[0][1]*xsize)) as usize] = vector[(x+(line[0][1]*xsize)) as usize] + 1;
                }
            }
        }
        let mut nb_gt_2: i32 = 0;
        for i in &vector {
            if i >= &2 {
                nb_gt_2 += 1;
            }
        }
        return nb_gt_2.to_string();
    }
    if part == 2 {
        let vecstr: Vec<&str> = input.lines().collect();
        let out: Vec<i32> = vec![];
        let mut res: Vec<Vec<Vec<i32>>> = vec![];
        for i in vecstr
        .iter()
        .map(|x| get_coord(x))
        .filter(|x| x[0][0]==x[1][0] || x[0][1]==x[1][1] || i32::abs(x[1][1]-x[0][1])==i32::abs(x[1][0]-x[0][0])) {
            res.push(i);
        }
        let (xsize, ysize) = get_sizes(&res);

        let mut vector: Vec<i32> = vec![0; (xsize*ysize).try_into().unwrap()];
        println!("Dim : {} - {}", xsize, ysize);
        println!("VEctor length : {}", vector.len());
        println!("Vector size : {}", xsize*ysize);

        for line in res {
            if line[0][0] == line[1][0] {
                // vertical line
                let min_val = cmp::min(line[0][1], line[1][1]);
                let max_val = cmp::max(line[0][1], line[1][1]);
                for x in min_val..max_val+1 {
                    vector[((x*xsize)+line[0][0]) as usize] = vector[((x*xsize)+line[0][0]) as usize] + 1;
                }
            }
            else if line[0][1] == line[1][1] {
                // horizontal line
                let min_val = cmp::min(line[0][0], line[1][0]);
                let max_val = cmp::max(line[0][0], line[1][0]);
                for x in min_val..max_val+1 {
                    vector[(x+(line[0][1]*xsize)) as usize] = vector[(x+(line[0][1]*xsize)) as usize] + 1;
                }
            }
            else {
                let a: i32 = xsize * line[0][1] + line[0][0];
                let b: i32 = xsize * line[1][1] + line[1][0];
                let sign_a: i32 = if line[1][1] < line[0][1] {-1} else {1};
                let sign_b: i32 = if line[1][0] < line[0][0] {-1} else {1};
                let mut i: usize = a as usize;
                vector[i as usize] = vector[i as usize] + 1;
                while i != b as usize {
                    i = i + (sign_a as usize) * (xsize as usize) as usize + 1*(sign_b as usize);
                    vector[i as usize] = vector[i as usize] + 1;
                }
            }
        }
        let mut nb_gt_2: i32 = 0;
        for i in &vector {
            if i >= &2 {
                nb_gt_2 += 1;
            }
        }
        return nb_gt_2.to_string();
    }
    else {
        return String::from("Exit");
    }
}
