use std::fs::File;
use std::io::Read;

#[derive(Clone, Debug)]
struct GearNum {
    gear: (usize, usize),
    num: String,
}

impl GearNum {
    fn new(gear: (usize, usize), num: String) -> Self {
        Self { gear, num }
    }
}

fn main() {
    println!("{}",process(input_text()));
}

fn process(text: String) -> String {

    let gears = find_gears(text);

    let sorted_gears = sort_gears(gears);

    let mut old_gear: (usize, usize) = (0,0);
    let mut nums: Vec<u128> = Vec::new(); 
    let mut result: u128 = 0;
    for i in 0..sorted_gears.len() {
        if i == 0 {
            old_gear = sorted_gears[i].gear;
            let num = sorted_gears[i].num.parse().unwrap();
            nums.push(num);
            println!("Added to nums first: {}", num);
        }
        else if sorted_gears[i].gear != old_gear {
            if nums.len() > 1{
                let mut sum = 1;
                for num in &nums {
                    sum = sum * num; 
                }
                result = result + sum;
                println!("{:?}", nums);
                println!("Added Sum to Result: {}\n Result is now: {}\n----\n", sum, result);
            }
            old_gear = sorted_gears[i].gear;
            nums = Vec::new();
            let num = sorted_gears[i].num.parse().unwrap();
            nums.push(num);
            println!("Added to nums middle: {}", num);
        } else {
            let num = sorted_gears[i].num.parse().unwrap();
            nums.push(num);
            println!("Added to nums last: {}", num);
        }
    }
    if nums.len() > 1{
        let mut sum = 1;
        for num in &nums {
            sum = sum * num; 
        }
        result = result + sum;
        println!("{:?}", nums);
        println!("Added Sum to Result: {}\n Result is now: {}\n----\n", sum, result);
    }

    println!("{:?}", sorted_gears);

    result.to_string()
}


fn find_gears(text: String) -> Vec<GearNum> {
    let mut arr: Vec<Vec<char>> = Vec::new();

    for line in text.lines() {
        let mut narr: Vec<char> = Vec::new();
        for char in line.chars() {
            narr.push(char);
        }
        arr.push(narr);
    }

    let mut nums: Vec<GearNum> = Vec::new();
    for i in 0..arr.len() {
        let mut num_string = String::new();
        let mut is_good_num = false;
        let mut gear: (usize, usize) = (0, 0);
        for j in 0..arr[i].len() {
            if arr[i][j].is_numeric() {
                num_string = format!("{}{}", num_string, arr[i][j]);
                // Top
                if i > 0 {
                    if !arr[i-1][j].is_numeric() && arr[i-1][j] == '*' {
                        is_good_num = true;
                        gear = (i-1, j);
                    }
                    // Top Left
                    if j > 0 {
                        if !arr[i-1][j-1].is_numeric() && arr[i-1][j-1] == '*' {
                            is_good_num = true;
                            gear = (i-1, j-1);
                        }
                    }
                    // Top Right
                    if j + 1 < arr[i].len() {
                        if !arr[i-1][j+1].is_numeric() && arr[i-1][j+1] == '*' {
                            is_good_num = true;
                            gear = (i-1, j+1);
                        }
                    }
                }
                // Bottom
                if i + 1 < arr.len() {
                    if !arr[i+1][j].is_numeric() && arr[i+1][j] == '*' {
                        is_good_num = true;
                        gear = (i+1, j);
                    }
                    // Bottom Left
                    if j > 0 {
                        if !arr[i+1][j-1].is_numeric() && arr[i+1][j-1] == '*' {
                            is_good_num = true;
                            gear = (i+1, j-1);
                        }
                    }
                    // Bottom Right
                    if j + 1 < arr[i].len() {
                        if !arr[i+1][j+1].is_numeric() && arr[i+1][j+1] == '*' {
                            is_good_num = true;
                            gear = (i+1, j+1);
                        }
                    }
                }
                // Left
                if j > 0 {
                    if !arr[i][j-1].is_numeric() && arr[i][j-1] == '*' {
                        is_good_num = true;
                        gear = (i, j-1);
                    }
                }
                // Right
                if j + 1 < arr[i].len() {
                    if !arr[i][j+1].is_numeric() && arr[i][j+1] == '*' {
                        is_good_num = true;
                        gear = (i, j+1);
                    }
                }
            } else if !num_string.is_empty() {
                if is_good_num {
                    nums.push(GearNum::new(gear, num_string));
                }
                num_string = String::new();
                is_good_num = false;
            }
        }
        if !num_string.is_empty() && is_good_num == true {
            nums.push(GearNum::new(gear, num_string));
        }
    }
    nums
}

fn sort_gears(gears: Vec<GearNum>) -> Vec<GearNum> {
    // Sort gears
    let mut sorted_gears = gears;
    for i in 0..sorted_gears.len() {
        for j in 0..sorted_gears.len()-1 {
            if sorted_gears[j].gear > sorted_gears[j+1].gear {
                let merker = sorted_gears[j].clone();
                sorted_gears[j] = sorted_gears[j+1].clone();
                sorted_gears[j+1] = merker;

            }
        }
    }
    sorted_gears
}

fn calc_gears() -> i32 {
    0
}

fn input_text() -> String {
    let mut input_text = match File::open("input.txt") {
        Ok(file) => file,
        Err(_) => panic!("Error reading File")
    };
    let mut input_buf: String = String::new();
    input_text.read_to_string(&mut input_buf).unwrap();
    input_buf
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!("4361".to_string(), process(input.to_string()));
    }
}