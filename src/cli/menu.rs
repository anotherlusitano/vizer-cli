use selthi::Select;

pub fn menu() -> Result<&'static str, ()> {
    print!("\x1B[2J\x1B[1;1H");

    let options = vec!["replay", "quit"];

    let ans = Select::new("Select what you want to do", options)
        .without_help_message()
        .prompt();

    match ans {
        Some(option) => Ok(option),
        None => Err(println!("Couldn't get option!")),
    }
}
