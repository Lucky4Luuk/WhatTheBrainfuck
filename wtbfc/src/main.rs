use std::env;

use console::Style;

pub mod lexer;
pub mod parser;

fn print_styled(text: impl AsRef<str>, style: &Style) {
    print!("{}", style.apply_to(text.as_ref()));
}

fn main() {
    let main_style = Style::new().green();
    let version_style = Style::new().cyan();
    let error_style = Style::new().red();

    // print!("{} {} {}", main_style.apply_to(">>"), main_style.apply_to("wtbfc"), version_style.apply_to("0.1-0a"));
    print_styled(">> wtbfc ", &main_style);
    print_styled("0.1-0a", &version_style);

    let args: Vec<String> = env::args().collect();
    let name = &args[0];
    let dir = env::current_dir().map_err(|e| print_styled("failed to get current directory", &error_style)).unwrap();

    // println!("{}", main_style.apply_to(format!(" - {}", name)));
    print_styled(format!("- {}\n", name), &main_style);
    // println!("{}", main_style.apply_to(format!(">> working directory: {}", dir.display())));
    print_styled(format!(">> working directory: {}", dir.display()), &main_style);
}
