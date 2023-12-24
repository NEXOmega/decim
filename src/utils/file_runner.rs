use binaryornot::is_binary;
use std::process::Command;
//TODO See to redirect process output to null
pub fn run_file(full_path: String) {
    if is_binary(full_path.clone()).expect("Error while checking if file is binary") {
        run_binary(full_path);
        return;
    }
    let extension = get_extension(get_file_name_with_extension(full_path.clone()));
    match extension.as_str() {
        "html" => open_html(full_path),
        "exe" => run_exe(full_path),
        "sh" => run_sh(full_path),
        "py" => run_python(full_path),
        _ => println!("Extension {} not supported", extension),
    }
}
fn get_extension(file_name: String) -> String {
    let (_, extension) = file_name.rsplit_once(".").unwrap();

    extension.to_string()
}

fn get_file_name_with_extension(file_name: String) -> String {
    let mut file_name_with_extension = String::new();
    let mut is_extension = false;
    for c in file_name.chars() {
        if is_extension {
            file_name_with_extension.push(c);
        }
        if c == '.' {
            is_extension = true;
        }
    }
    file_name_with_extension
}

fn open_html(full_path: String) {
    let mut command = Command::new("xdg-open");
    command.arg(full_path);
    command.spawn().expect("Error while starting the script");
}

fn run_binary(full_path: String) {
    let mut command = Command::new(full_path);
    command.spawn().expect("Error while starting the script");
}

fn run_exe(full_path: String) {
    let mut command = Command::new(full_path);
    command.spawn().expect("Error while starting the script");
}

fn run_sh(full_path: String) {
    let mut command = Command::new("sh");
    command.arg(full_path);
    command.spawn().expect("Error while starting the script");
}

fn run_python(full_path: String) {
    let mut command = Command::new("python");
    command.arg(full_path);
    command.spawn().expect("Error while starting the script");
}