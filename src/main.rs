// Last Update: 19/09/21 22:00:00
mod utils;
mod managers;


fn main() -> std::io::Result<()>{

    utils::get_config_dir();

    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        managers::commands_manager::handle_command();
        return Ok(());
    }

    Ok(())
}
