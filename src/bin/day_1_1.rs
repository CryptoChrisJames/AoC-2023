use std::fs;

fn main() {
    let location: &str = "src/bin/day_1_1_sample.txt";
    let calibration_document = fs::read_to_string(location).unwrap();
    let lines_of_doc: Vec<String> = calibration_document.lines()
                                                        .map(|line| line.to_string())
                                                        .collect();

    let filtered: Vec<usize> = lines_of_doc.iter().map(|line| {
        let chars: Vec<char> = line.chars().collect();
        let mut num_string: String = "".to_string();
        for c in chars {
            if !c.to_string().parse::<usize>().is_err() {
                num_string.push(c);
            }
        }
        num_string.parse::<usize>().unwrap()
    }).collect();

    let numbers: Vec<usize> = filtered.iter().map(|num| {
        let nums: Vec<char> = num.to_string().chars().collect();
        if nums.len() > 2 {
            let mut two_digits: String = "".to_string();
            two_digits.push(nums[0]);
            two_digits.push(nums[nums.len()-1]);
            return two_digits.parse::<usize>().unwrap();
        }
        if nums.len() == 1  {
            let mut two_digits: String = "".to_string();
            two_digits.push(nums[0]);
            two_digits.push(nums[0]);
            return two_digits.parse::<usize>().unwrap();
        }
        *num
    }).collect();

    let sum = numbers.iter().fold(0, |acc, n| acc + n);
    print!("{}", sum);
}