use task_tracker::*;
use termion::clear;
use std::io;

fn main() {
    let mut status = Menu::MainMenu;
    loop {
        println!("{}", clear::All);
        show_menu(&mut status);
        let mut input = String::from("");
        io::stdin().read_line(&mut input).expect("Can't read the input");
        status = change_status(input);
    }
}
