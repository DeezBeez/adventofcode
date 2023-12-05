use std::fs::File;
use std::io::Read;

fn main() {
    println!("{}",process(input_text()));
}

fn process(text: String) -> String {
    let mut arr: Vec<Vec<char>> = Vec::new();

    for line in text.lines() {
        let mut narr: Vec<char> = Vec::new();
        for char in line.chars() {
            narr.push(char);
        }
        arr.push(narr);
    }

    let mut nums: Vec<String> = Vec::new();
    for i in 0..arr.len() {
        let mut num_string = String::new();
        let mut is_good_num = false;
        for j in 0..arr[i].len() {
            if arr[i][j].is_numeric() {
                num_string = format!("{}{}", num_string, arr[i][j]);
                // Top
                if i > 0 {
                    if !arr[i-1][j].is_numeric() && arr[i-1][j] != '.' {
                        is_good_num = true
                    }
                    // Top Left
                    if j > 0 {
                        if !arr[i-1][j-1].is_numeric() && arr[i-1][j-1] != '.' {
                            is_good_num = true
                        }
                    }
                    // Top Right
                    if j + 1 < arr[i].len() {
                        if !arr[i-1][j+1].is_numeric() && arr[i-1][j+1] != '.' {
                            is_good_num = true
                        }
                    }
                }
                // Bottom
                if i + 1 < arr.len() {
                    if !arr[i+1][j].is_numeric() && arr[i+1][j] != '.' {
                        is_good_num = true
                    }
                    // Bottom Left
                    if j > 0 {
                        if !arr[i+1][j-1].is_numeric() && arr[i+1][j-1] != '.' {
                            is_good_num = true
                        }
                    }
                    // Bottom Right
                    if j + 1 < arr[i].len() {
                        if !arr[i+1][j+1].is_numeric() && arr[i+1][j+1] != '.' {
                            is_good_num = true
                        }
                    }
                }
                // Left
                if j > 0 {
                    if !arr[i][j-1].is_numeric() && arr[i][j-1] != '.' {
                        is_good_num = true
                    }
                }
                // Right
                if j + 1 < arr[i].len() {
                    if !arr[i][j+1].is_numeric() && arr[i][j+1] != '.' {
                        is_good_num = true
                    }
                }
            } else if !num_string.is_empty() {
                if is_good_num {
                    nums.push(num_string);
                }
                num_string = String::new();
                is_good_num = false;
            }
        }
        if !num_string.is_empty() && is_good_num == true {
            nums.push(num_string);
        }
    }

    let mut result = 0;
    for string in nums {
        let num: i32 = string.parse().unwrap();
        result = result + num;
    }

    result.to_string()
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