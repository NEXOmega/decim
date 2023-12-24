extern crate serde;
extern crate serde_json;

use std::fs::File;
use std::io::Write;
use std::path::Path;
use zip::write::FileOptions;
use crate::utils;
use crate::utils::game::Game;

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

pub fn list_games() -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for entry in std::fs::read_dir(utils::get_game_dir()).unwrap() {
        let path = entry.unwrap().path();
        let file_name = path.file_name().unwrap().to_str().unwrap();
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
    let mut games_distance: Vec<(Game, usize)> = Vec::new();
    for game in list_games() {
        let distance = edit_distance::edit_distance(&game.get_name(), &search);
        games_distance.push((game, distance));
    }

    games_distance.sort_by(|a, b| a.1.cmp(&b.1));
    games_distance.retain(|x| x.1 <= 5);
    games_distance.truncate(5);

    let mut games: Vec<Game> = Vec::new();
    for game in games_distance {
        games.push(game.0);
    }
    games
}

pub fn search_games_by_tags(search: Vec<&str>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    for game in list_games() {
        let mut found = true;
        for tag in search.clone() {
            if !game.get_tags().contains(&tag.to_string()) {
                found = false;
            }
        }
        if found {
            games.push(game);
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