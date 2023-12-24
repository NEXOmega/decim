pub mod game;
pub(crate) mod file_runner;

pub fn get_config_dir() -> String {
    let mut config_dir = String::from(std::env::var("HOME").unwrap());
    config_dir.push_str("/.config");
    config_dir.push_str("/decim");
    if !std::path::Path::new(&config_dir).exists() {
        std::fs::create_dir_all(&config_dir).unwrap();
    }
    config_dir
}
pub fn get_game_dir() -> String {
    let mut game_dir = String::from(get_config_dir());
    game_dir.push_str("/games");
    if !std::path::Path::new(&game_dir).exists() {
        std::fs::create_dir_all(&game_dir).unwrap();
    }
    game_dir
}

pub fn get_backup_dir() -> String {
    let mut backup_dir = String::from(get_config_dir());
    backup_dir.push_str("/backups");
    if !std::path::Path::new(&backup_dir).exists() {
        std::fs::create_dir_all(&backup_dir).unwrap();
    }
    backup_dir
}