#[macro_use]
extern crate prettytable;

mod command;
mod steam_api;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        command::show_savings();
    } else {
        match args[1].as_str() {
            "add" => {
                for x in 2..args.len() {
                    command::add_game(args[x].clone());
                }
            }
            "remove" => {
                for x in 2..args.len() {
                    command::remove_game(args[x].clone());
                }
            }
            "list" => command::show_all_games(),
            _ => eprintln!("Invalid args."),
        }
    }
}
