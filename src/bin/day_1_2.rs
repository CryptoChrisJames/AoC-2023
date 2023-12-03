use std::{fs, collections::HashMap, };

fn compare(subject: &String) -> Option<char> {
    let mut number_strings = HashMap::new();

    number_strings.insert("zero", 0);
    number_strings.insert("one", 1);
    number_strings.insert("two", 2);
    number_strings.insert("three",3);
    number_strings.insert("four", 4);
    number_strings.insert("five", 5);
    number_strings.insert("six", 6);
    number_strings.insert("seven",7);
    number_strings.insert("eight", 8);
    number_strings.insert("nine", 9);

    for number in &number_strings {
        if subject.contains(*number.0) {
            return Some(char::from_u32(number_strings[number.0]).unwrap())
        }
    }

    return None;
}

fn main() {
    let location: &str = "src/bin/day_1_2_sample.txt";
    let calibration_document = fs::read_to_string(location).unwrap();
    let lines_of_doc: Vec<String> = calibration_document.lines()
                                                        .map(|line| line.to_string())
                                                        .collect();

    let filtered: Vec<String> = lines_of_doc.iter().map(|line| {
        let chars: Vec<char> = line.chars().collect();
        let mut current_sub_string: String = "".to_string();
        let mut num_string: String = "".to_string();
        for c in chars {
            if !c.to_string().parse::<usize>().is_err() {
                num_string.push(c);
                current_sub_string = "".to_string();
            } else {
                current_sub_string.push(c);
            }
            if current_sub_string.len() > 2 {
                let is_num = compare(&current_sub_string);
                match is_num {
                    Some(c) => {
                        num_string.push(c);
                        current_sub_string = "".to_string();
                    },
                    None => {}
                }
            }
        }
        num_string
    }).collect();

    for v in filtered {
        println!("{}", v);
    }

    // let numbers: Vec<usize> = filtered.iter().map(|num| {
    //     let nums: Vec<char> = num.to_string().chars().collect();
    //     if nums.len() > 2 {
    //         let mut two_digits: String = "".to_string();
    //         two_digits.push(nums[0]);
    //         two_digits.push(nums[nums.len()-1]);
    //         return two_digits.parse::<usize>().unwrap();
    //     }
    //     if nums.len() == 1  {
    //         let mut two_digits: String = "".to_string();
    //         two_digits.push(nums[0]);
    //         two_digits.push(nums[0]);
    //         return two_digits.parse::<usize>().unwrap();
    //     }
    //     *num
    // }).collect();

    // let sum = numbers.iter().fold(0, |acc, n| acc + n);
    // print!("{}", sum);
}