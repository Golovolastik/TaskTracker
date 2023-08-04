use task_tracker::*;
use termion::clear;
use std::io;
use crate::menu::*;

fn main() {
    let mut menu = Menu::MainMenu;
    loop {
        //println!("{}", clear::All);
        show_menu(&menu);
        let mut input = String::from("");
        io::stdin().read_line(&mut input).expect("Can't read the input");
        menu = menu::handle_input(menu, input);
    }
}
