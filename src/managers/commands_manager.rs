use clap::{arg, Command};
use tabled::{settings::Style, Table};
use crate::managers::game_manager;
use crate::managers::game_manager::{search_games, display_game, start_game, delete_game, search_games_by_tag, load_game, backup, backup_all, search_games_by_tags};

pub fn handle_command() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("start", sub_m)) => {
            let game_name = sub_m.get_one::<String>("NAME").unwrap();
            start_game(format!("{}.json", game_name));
        },
        Some(("list", _sub_m)) => {
            let games = search_games(String::from(""));
            let table = Table::new(&games).with(Style::modern()).to_string();
            println!("{}", table);
        },
        Some(("delete", sub_m)) => {
            let game_name = sub_m.get_one::<String>("NAME").unwrap();
            delete_game(game_name.to_string());
        },
        Some(("search", sub_m)) => {
            let game_name = sub_m.get_one::<String>("NAME").unwrap();
            let games = search_games(game_name.to_string());
            let table = Table::new(&games).with(Style::modern()).to_string();
            println!("{}", table);
        },
        Some(("search-tag", sub_m)) => {
            let tag = sub_m.get_one::<String>("TAG").unwrap();
            let games = search_games_by_tag(tag.to_string());
            let table = Table::new(&games).with(Style::modern()).to_string();
            println!("{}", table);
        },
        Some(("search-tags", _sub_m)) => {
            let tags = _sub_m.get_many::<String>("TAGS").unwrap();
            let tags: Vec<_> = tags.map(|x| x.as_str()).collect();
            let games = search_games_by_tags(tags);
            let table = Table::new(&games).with(Style::modern()).to_string();
            println!("{}", table);
        },
        Some(("create", sub_m)) => {
            let name = sub_m.get_one::<String>("NAME").unwrap();
            let description: Vec<_> = sub_m.get_many::<String>("DESCRIPTION").expect("Error while getting description").map(|x| x.as_str()).collect();
            let description = description.join(" ");
            let version = sub_m.get_one::<String>("VERSION").unwrap();
            let executable = sub_m.get_one::<String>("EXECUTABLE").unwrap();
            let mut game = crate::utils::game::Game::new(name.to_string(), description.to_string());
            game.edit_version(version.to_string());
            game.edit_executable(executable.to_string());
            crate::managers::game_manager::save_game(game);
        },
        Some(("edit", _sub_m)) => {
            let game_name = _sub_m.get_one::<String>("NAME").unwrap();
            let game = load_game(format!("{}.json", game_name));
            if game.is_none() {
                println!("Game not found");
                return;
            }
            let mut game = game.unwrap();
            if let Some(description) = _sub_m.get_many::<String>("DESCRIPTION") {
                let description: Vec<_> = description.map(|x| x.as_str()).collect();
                let description = description.join(" ");
                game.description = description;
            }

            if let Some(version) = _sub_m.get_one::<String>("VERSION") {
                game.edit_version(version.to_string());
            }

            if let Some(save_location) = _sub_m.get_one::<String>("SAVE_LOCATION") {
                game.edit_save_location(save_location.to_string());
            }

            if let Some(executable) = _sub_m.get_one::<String>("EXECUTABLE") {
                game.edit_executable(executable.to_string());
            }

            if let Some(tags) = _sub_m.get_many::<String>("TAGS") {
                let tags: Vec<_> = tags.map(|x| x.as_str()).collect();
                for tag in tags {
                    game.add_tag(tag.to_string());
                }
            }

            if let Some(tags) = _sub_m.get_many::<String>("REMOVE_TAGS") {
                let tags: Vec<_> = tags.map(|x| x.as_str()).collect();
                for tag in tags {
                    game.remove_tag(tag.to_string());
                }
            }

            game_manager::edit_game(game);
            println!("Game {} edited", game_name);
        },
        Some(("backup", _sub_m)) => {
            let game = _sub_m.get_one::<String>("NAME").unwrap();
            let game = load_game(format!("{}.json", game));
            if game.is_none() {
                println!("Game not found");
                return;
            }
            backup(game.unwrap());
        },
        Some(("backup_all", _sub_m)) => {
            backup_all();
        },
        _ => println!("Command not found"),
    }
}

pub fn cli() -> Command {
    Command::new("decim")
        .author("NEXOmega")
        .version("0.1.0")
        .about("A game manager")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommand(
            Command::new("start")
                .short_flag('S')
                .long_flag("start")
                .about("Start a game")
                .arg(arg!(<NAME> "The name of the game to start"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("delete")
                .about("Delete a game")
                .short_flag('D')
                .arg(arg!(<NAME> "The name of the game to delete"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("search")
                .about("Search a game")
                .short_flag('s')
                .arg(arg!(<NAME> "The name of the game to search"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("search-tag")
                .about("Search a game by tag")
                .arg(arg!(<TAG> "The tag of the game to search"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("search-tags")
                .about("Search a game by tags")
                .arg(arg!(<TAGS>... "The tags of the game to search"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("create")
                .about("Create a game")
                .arg(arg!(<NAME> "The name of the game to create").required(true))
                .arg(arg!(<DESCRIPTION> "The description of the game to create").required(true).num_args(1..))
                .arg(arg!(<VERSION> "The version of the game to create")
                    .short('v')
                    .long("version")
                    .default_value("0.1.0")
                    .required(false)
                )
                .arg(arg!(<EXECUTABLE> "The executable of the game to create")
                    .short('e')
                    .long("executable")
                    .default_value("")
                    .required(false)
                )
        )
        .subcommand(
            Command::new("edit")
                .about("Edit a game")
                .arg(arg!(<NAME> "The name of the game to edit"))
                .arg(arg!(<DESCRIPTION>... "The description of the game to edit")
                    .short('d')
                    .long("description")
                    .required(false)
                    .num_args(1..)
                )
                .arg(arg!(<VERSION> "The version of the game to edit")
                    .short('v')
                    .long("version")
                    .required(false)
                )
                .arg(arg!(<SAVE_LOCATION> "The save location of the game to edit")
                    .short('s')
                    .long("save_location")
                    .required(false)
                )
                .arg(arg!(<EXECUTABLE> "The executable of the game to edit")
                    .short('e')
                    .long("executable")
                    .required(false)
                )
                .arg(arg!(<TAGS>... "Add Tags")
                    .short('t')
                    .long("add_tags")
                    .num_args(1..)
                    .required(false)
                )
                .arg(arg!(<REMOVE_TAGS>... "Remove Tags")
                    .short('r')
                    .long("remove_tags")
                    .num_args(1..)
                    .required(false)
                )
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("backup")
                .about("Backup a game")
                .arg(arg!(<NAME> "The name of the game to backup"))
                .arg_required_else_help(true)
        )
        .subcommand(
            Command::new("backup_all")
                .about("Backup all games")
        )
}