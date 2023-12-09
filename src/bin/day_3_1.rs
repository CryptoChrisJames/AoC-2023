use std::{fs, char};

fn get_lines(val: String) -> Vec<String> {
    return val.lines()
    .map(|line| line.to_string())
    .collect();
}

fn get_numbers_with_coordinates(grid: &Vec<Vec<char>>) -> Vec<(usize, usize, usize)> {
    let mut numbers_list: Vec<(usize, usize, usize)> = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let current = grid[y][x];
            let is_num = current.to_string().parse::<usize>();
            if is_num.is_ok() {
                numbers_list.push((is_num.unwrap(), x, y));
            }
        }
    }
    numbers_list
}

fn is_symbol(c: char) -> bool {
    !c.is_alphabetic() && !c.is_numeric() && !c.is_whitespace() && c != '.'
}

fn get_symbols_with_coordinates(grid: Vec<Vec<char>>) -> Vec<(char, usize, usize)> {
    let mut symbols_list: Vec<(char, usize, usize)> = Vec::new();
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let current = grid[y][x];
            if is_symbol(current){
                symbols_list.push((current, x, y));
            }
        }
    }
    symbols_list
}

fn get_whole_numbers(numbers_list: Vec<(usize, usize, usize)>) -> Vec<(u32, Vec<(usize, usize)>)> {
    let mut whole_numbers: Vec<(u32, Vec<(usize, usize)>)> = Vec::new();
    let mut current_number: String = "".to_string();
    let mut current_coordinates: Vec<(usize, usize)> = Vec::new();

    for (i, n) in numbers_list.iter().enumerate() {
        if i == 0 { current_number.push(char::from_u32(n.0 as u32 + 48).unwrap()); current_coordinates.push((n.1, n.2)); continue;}
        if numbers_list[i-1].1 + 1 == n.1 && numbers_list[i-1].2 == n.2 { current_number.push(char::from_u32(n.0 as u32 + 48).unwrap()); current_coordinates.push((n.1, n.2));}
        else {
            whole_numbers.push((current_number.parse::<u32>().unwrap(), current_coordinates));
            current_number = "".to_string();
            current_coordinates = Vec::new();
            current_number.push(char::from_u32(n.0 as u32 + 48).unwrap());
            current_coordinates.push((n.1, n.2));
        }
        if i == numbers_list.len() - 1 {
            whole_numbers.push((current_number.parse::<u32>().unwrap(), current_coordinates.clone()));
        }
    }
    whole_numbers
}

fn main() {
    let location: &str = "src/bin/day_3_1_sample.txt";
    let calibration_document = fs::read_to_string(location).unwrap();
    let lines_of_doc: Vec<String> = get_lines(calibration_document);
    //put lines into 2d array
    let grid: Vec<Vec<char>> = lines_of_doc.iter().map(|line| {
        return line.chars().collect();
    }).collect();
    //gather all of the numbers with their coordinates
    let numbers_list: Vec<(usize, usize, usize)> = get_numbers_with_coordinates(&grid);
    //reconstruct the numbers
    let mut whole_numbers: Vec<(u32, Vec<(usize, usize)>)> = get_whole_numbers(numbers_list);


    //gather the symbols
    let symbols_list: Vec<(char, usize, usize)> = get_symbols_with_coordinates(grid);
    //start going through what's been gathered and try to determine adjacent symbols
    let neighbor_offsets = vec![(-1, 0), (1, 0), (0, -1), (0, 1), (-1, 1), (-1, -1), (1, -1), (1, 1)];

    let mut total = 0;
    for s in symbols_list {
        println!("{0}:{1},{2}", s.0, s.1, s.2);
    }
}