use std::{fs, collections::HashMap};

fn main() {
    let location: &str = "src/bin/day_2_1_sample.txt";
    let games = fs::read_to_string(location).unwrap();
    let lines_of_doc: Vec<String> = games.lines()
                                                        .map(|line| line.to_string())
                                                        .collect();
    let just_games: Vec<&str> = lines_of_doc.iter().map(|game| {
        let the_game: Vec<&str> = game.split(":").collect();
        the_game[1]
    }).collect();

    let format_games: Vec<String> = just_games.iter().map(|g| {
        str::replace(g, ";", ",")
    }).collect();

    let rounds_per_game: Vec<Vec<&str>> = format_games.iter().map(|g| {
        g.split(",").collect()
    }).collect();

    let mut only_possibilites: HashMap<String, i32> = HashMap::new();
    only_possibilites.insert("red".to_string(), 12);
    only_possibilites.insert("green".to_string(), 13);
    only_possibilites.insert("blue".to_string(), 14);

    let possiblities: Vec<bool> = rounds_per_game.iter().map(|g| {
        let mut is_possible = true;
        for round in g {
            let outcome: Vec<&str> = round.split(" ").collect();
            let out_number = &outcome[1].parse::<i32>().unwrap();
            let out_cube = outcome[2].to_string();
            if *out_number > only_possibilites[&out_cube] {
                is_possible = false;
            }
        }
        is_possible
    }).collect();

    let mut total: usize = 0;

    for (i,p) in possiblities.iter().enumerate() {
        if *p {
            total += i+1
        } else {}
    }
    print!("{}", total)
}