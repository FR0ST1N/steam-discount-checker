use crate::steam_api::{get_data, Game};
use prettytable::Table;
use std::{
    fs::File,
    io::{BufRead, BufReader, Write},
};

const FILE_NAME: &str = "/steamdc.list";

pub fn show_savings() {
    let mut table = Table::new();
    table.add_row(row!["Name", "Discount %"]);

    let mut games: Vec<Game> = get_games();

    // filter out items not on sale
    games = games
        .iter()
        .filter(|game| game.is_on_sale == "1")
        .cloned()
        .collect();

    // Sort by savings
    games.sort_by(|l, r| {
        let a: f32 = l.savings.parse().unwrap();
        let b: f32 = r.savings.parse().unwrap();
        b.partial_cmp(&a).unwrap()
    });

    // print table
    if games.len() > 0 {
        for game in games {
            table.add_row(row![game.title, game.savings]);
        }
        table.printstd();
    } else {
        println!("No games from your list is currently on sale.");
    }
}

fn get_games() -> Vec<Game> {
    let reader = BufReader::new(get_file());

    let mut games: Vec<Game> = Vec::new();
    for line in reader.lines() {
        let id = line.unwrap();

        match get_data(id.to_string()) {
            Ok(game) => games.push(game[0].clone()),
            Err(..) => println!("Error Fetching {}.", id),
        };
    }
    games
}

fn get_file() -> File {
    let file_path = get_file_path();
    let file = match File::open(file_path) {
        Ok(f) => f,
        Err(..) => panic!("Game list not found.\nTry adding game with:\nsteamdc add [id]"),
    };
    file
}

fn get_file_path() -> String {
    dirs_next::config_dir()
        .unwrap()
        .into_os_string()
        .into_string()
        .unwrap()
        + FILE_NAME
}

pub fn add_game(id: String) {
    let file_path = get_file_path();
    match get_data(id.clone()) {
        Ok(game) => {
            // add to list
            let mut file = std::fs::OpenOptions::new()
                .write(true)
                .create(true)
                .append(true)
                .open(file_path)
                .unwrap();
            if let Err(e) = writeln!(file, "{}", id) {
                panic!("Couldn't write to game list: {}", e);
            }
            println!("{} added to the list.", game[0].title);
        }
        Err(..) => {
            panic!("Game not found.");
        }
    }
}

pub fn remove_game(id: String) {
    let reader = BufReader::new(get_file());

    // Read game list into vec
    let mut games: Vec<String> = Vec::new();
    for line in reader.lines() {
        games.push(line.unwrap());
    }

    // Remove id
    games.retain(|game| game != id.as_str());

    // Write to file
    let file_path = get_file_path();
    let mut file = std::fs::OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(file_path)
        .unwrap();
    for game in games {
        if let Err(e) = writeln!(file, "{}", game) {
            panic!("Couldn't write to game list: {}", e);
        }
    }

    println!("Removed {}.", id);
}

pub fn show_all_games() {
    let mut table = Table::new();
    table.add_row(row!["ID", "Name"]);

    let games: Vec<Game> = get_games();

    // print list
    for game in games {
        table.add_row(row![game.steam_app_id, game.title]);
    }
    table.printstd();
}
