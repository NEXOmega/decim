extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use zip::write::FileOptions;
use tabled::{Table, Tabled};
use crate::utils;
use crate::utils::game::Game;

pub fn dialog() {
    println!("Welcome to the game manager!");
    loop {
        println!("What would you like to do?");
        println!("1. Create a new game");
        println!("2. List all games");
        println!("3. Search for a game");
        println!("4. Display Game");
        println!("5. Edit Game");
        println!("6. Launch game");
        println!("7. Exit");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        match input {
            "1" => {
                println!("Enter the name of the game");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                println!("Enter the description of the game");
                let mut input2 = String::new();
                std::io::stdin().read_line(&mut input2).unwrap();
                let input2 = input2.trim();
                let game = create_game(String::from(input), String::from(input2));
                save_game(game);

            },
            "2" => {
                let games = list_games();
                for game in games {
                    println!("{}", game.get_name());
                }
            },
            "3" => search_game(),
            "4" => {
                let game_name = utils::ask(String::from("Enter the name of the game"));
                if !game_exists(game_name.clone()) {
                    println!("Game {} does not exist", game_name);
                    continue;
                }
                let game = load_game(game_name);
                if game.is_none() {
                    continue;
                }
                let game = game.unwrap();
                display_game(game);
            },
            "5" => {
                let game = load_game(utils::ask(String::from("Enter the name of the game")));
                if game.is_none() {
                    continue;
                }
                let mut game = game.unwrap();
                println!("What would you like to edit?");
                println!("1. Name");
                println!("2. Description");
                println!("3. Tags");
                println!("4. Version");
                println!("5. Save Location");
                println!("6. Executable");
                let mut input = String::new();
                std::io::stdin().read_line(&mut input).unwrap();
                let input = input.trim();
                match input {
                    "1" => game.name = utils::ask(String::from("Enter the new name")),
                    "2" => game.description = utils::ask(String::from("Enter the new description")),
                    "3" => {
                        println!("How would you like to edit the tags?");
                        println!("1. Add a tag");
                        println!("2. Remove a tag");
                        println!("3. Add multiple tags");
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();
                        let input = input.trim();
                        match input {
                            "1" => game.add_tag(utils::ask(String::from("Enter the tag to add"))),
                            "2" => game.remove_tag(utils::ask(String::from("Enter the tag to remove"))),
                            "3" => {
                                let mut input = utils::ask(String::from("Enter tags to add (separated by commas)"));
                                input = input.trim().to_string();
                                let tags: Vec<&str> = input.split(",").collect();
                                let tags: Vec<String> = tags.iter().map(|x| String::from(*x)).collect();
                                game.add_tags(tags);
                            },
                            _ => println!("Invalid input"),
                        }
                    },
                    "4" => game.version = utils::ask(String::from("Enter the new version")),
                    "5" => game.save_location = utils::ask(String::from("Enter the new save location")),
                    "6" => game.executable = utils::ask(String::from("Enter the new executable")),
                    _ => println!("Invalid input"),
                }
                edit_game(game);
            },
            "6" => {
                start_game(utils::ask(String::from("Enter the name of the game")));
            },
            "7" => break,
            _ => println!("Invalid input"),
        }
    }
}

pub(crate) fn start_game(string: String) {
    let game = load_game(string);
    if game.is_none() {
        return;
    }
    let game = game.unwrap();
    if game.get_executable() == "" || !Path::new(&game.get_executable()).exists() {
        println!("Game executable does not exist");
        return;
    }
    println!("Launching game...");
    utils::file_runner::run_file(game.get_executable());
}

fn search_game() {
    println!("How would you like to search?");
    println!("1. By name");
    println!("2. By tag");
    println!("3. By tags");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    match input {
        "1" => {
            println!("Enter the name of the game");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            let games = search_games(String::from(input));
            for game in games {
                println!("{}", game.get_name());
            }
        },
        "2" => {
            println!("Enter the tag of the game");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            let games = search_games_by_tag(String::from(input));
            for game in games {
                println!("{}", game.get_name());
            }
        },
        "3" => {
            println!("Enter the tags of the game (separated by commas)");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let input = input.trim();
            let tags: Vec<&str> = input.split(",").collect();
            let games = search_games_by_tags(tags.iter().map(|x| String::from(*x)).collect());
            for game in games {
                println!("{}", game.get_name());
            }
        },
        _ => println!("Invalid input"),
    }
}

pub(crate) fn display_game(game: Game) {
    println!("Name: {}", game.get_name());
    println!("Description: {}", game.get_description());
    println!("Tags: {}", game.get_tags().join(", "));
    println!("Version: {}", game.get_version());
    println!("Save Location: {}", game.get_save_location());
    println!("Executable: {}", game.get_executable());
}

fn create_game(name: String, description: String) -> Game {
    let mut game = Game::new(name, description);

    let input = utils::ask(String::from("Enter tags for the game (separated by commas)"));
    let input = input.trim();
    let tags: Vec<&str> = input.split(",").collect();
    for tag in tags {
        game.add_tag(String::from(tag));
    }

    game.edit_version(utils::ask(String::from("Enter the version of the game")));
    game.edit_save_location(utils::ask(String::from("Enter the save location for the game")));
    game.edit_executable(utils::ask(String::from("Enter the executable for the game")));
    game
}

fn list_games() -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for entry in std::fs::read_dir(utils::get_game_dir()).unwrap() {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        println!("{}", file_name);
        if file_name.ends_with(".json") {
            let game = load_game(String::from(file_name));
            if game.is_none() {
                continue;
            }
            games.push(game.unwrap());
        }
    }
    games

}

fn game_exists(name: String) -> bool {
    for entry in std::fs::read_dir(utils::get_game_dir()).unwrap() {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".json") {
            if file_name == &name {
                return true;
            }
        }
    }
    false
}

pub fn save_game(game: Game) {
    let json = serde_json::to_string(&game).unwrap();
    let mut file = File::create(format!("{}/{}.json", utils::get_game_dir(), game.get_name())).unwrap();
    file.write_all(json.as_bytes()).unwrap();
}

pub fn load_game(name: String) -> Option<Game> {
    let path = format!("{}/{}", utils::get_game_dir(), name);
    if !Path::new(&path).exists() {
        println!("Game {} does not exist", name);
        return None;
    }
    let file = File::open(path).unwrap();
    let game: Game = serde_json::from_reader(file).unwrap();
    Some(game)
}

pub fn delete_game(name: String) {
    std::fs::remove_file(format!("{}/{}.json", utils::get_game_dir(), name)).unwrap();
}

pub fn edit_game(game: Game) {
    delete_game(game.get_name());
    save_game(game);
}

pub fn search_games(search: String) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for entry in std::fs::read_dir(utils::get_game_dir()).unwrap() {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".json") {
            let game = load_game(String::from(file_name));
            if game.is_none() {
                continue;
            }
            let game = game.unwrap();
            if game.get_name().contains(&search) || game.get_description().contains(&search) {
                games.push(game);
            }
        }
    }
    games
}

pub fn search_games_by_tag(search: String) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for entry in std::fs::read_dir(utils::get_game_dir()).unwrap() {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".json") {
            let game = load_game(String::from(file_name));
            if game.is_none() {
                continue;
            }
            let game = game.unwrap();
            for tag in game.get_tags() {
                if tag.contains(&search) {
                    games.push(game.clone());
                }
            }
        }
    }
    games
}

pub fn search_games_by_tags(search: Vec<String>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for entry in std::fs::read_dir(utils::get_game_dir()).unwrap() {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        if file_name.ends_with(".json") {
            let game = load_game(String::from(file_name));
            if game.is_none() {
                continue;
            }
            let game = game.unwrap();
            for tag in game.get_tags() {
                for search_tag in &search {
                    if tag.contains(search_tag) {
                        games.push(game.clone());
                    }
                }
            }
        }
    }
    games
}

pub fn backup(game: Game) {
    let mut backup_dir = utils::get_backup_dir();
    let save_dir = game.get_save_location();
    if save_dir == "" {
        println!("Save location not set for {}", game.get_name());
        return;
    }
    if !Path::new(&save_dir).exists() {
        println!("Save directory does not exist for {}", game.get_name());
        return;
    }

    backup_dir.push_str("/");
    backup_dir.push_str(&game.get_name());
    backup_dir.push_str("/");
    if !Path::new(&backup_dir).exists() {
        std::fs::create_dir_all(&backup_dir).unwrap();
    }
    let date_as_string = chrono::Local::now().format("%Y-%m-%d_%H-%M-%S").to_string();
    let mut zip = zip::ZipWriter::new(File::create(format!("{}/{}--{}-{}.zip", backup_dir, game.get_name(), game.get_version(), date_as_string)).unwrap());
    let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
    for entry in std::fs::read_dir(save_dir).unwrap() {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
        zip.start_file(file_name, options).unwrap();
        let mut file = File::open(path).unwrap();
        std::io::copy(&mut file, &mut zip).unwrap();
    }
    zip.finish().unwrap();

    println!("Backup of {} created", game.get_name());
}

pub fn backup_all() {
    for game in list_games() {
        backup(game);
    }
}