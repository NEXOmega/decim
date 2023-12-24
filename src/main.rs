// Last Update: 19/09/21 22:00:00
mod utils;
mod managers;

#[macro_use]
extern crate text_io;

fn main() -> std::io::Result<()>{

    utils::get_config_dir();

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        managers::commands_manager::handle_command();
        return Ok(());
    }

    ctrlc::set_handler(move || {
        save_all();
    }).expect("Error setting Ctrl-C handler");

    println!("Hello, world!");

    loop {
        println!("============");
        println!("1>Start Game Manager\n3>Exit");
        let cat:i32 = read!();

        match cat {
            1 => managers::game_manager::dialog(),
            _ => {
                save_all();
                println!("A plus tard !");
                break;
            }
        }
    }

    Ok(())
}

fn save_all() {
    println!("All config saved.");
}
