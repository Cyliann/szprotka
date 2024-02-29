use std::io::{self, Write};

pub fn handle_input() -> (String, String) {
    let mut room = String::new();
    let mut username = String::new();

    username = read(&mut username, "Username: ");
    room = read(&mut room, "Room (can be empty): ");

    (username, room)
}

fn read(variable: &mut String, comment: &str) -> String {
    print!("{}", comment);
    io::stdout().flush().unwrap();
    io::stdin().read_line(variable).unwrap();
    variable.trim().to_string()
}
