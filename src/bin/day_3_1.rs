use std::fs;

fn main() {
    let location: &str = "src/bin/day_3_1_sample.txt";
    let calibration_document = fs::read_to_string(location).unwrap();
    let lines_of_doc: Vec<String> = calibration_document.lines()
                                                        .map(|line| line.to_string())
                                                        .collect();
    //put lines into 2d array
    let grid: Vec<Vec<char>> = lines_of_doc.iter().map(|line| {
        return line.chars().collect();
    }).collect();
    //gather all of the numbers with their coordinates
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
    //reconstruct the numbers
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

    //maybe gather the symbols too
    //start going through what's been gathered and try to determine adjacent symbols
    let neighbor_offsets = vec![(-1, 0), (1, 0), (0, -1), (0, 1), (-1, 1), (-1, -1), (1, -1), (1, 1)];

    let mut total = 0;
    for w in whole_numbers {
        for c in w.1 {
             
        }
    }
}