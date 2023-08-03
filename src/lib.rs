use std::process;
use time;

pub enum Menu {
    MainMenu,
    TaskMenu,
}

#[warn(dead_code)]
pub struct Task {
    task: String,
    deadline: time::Date,
    status: Status,
}

#[warn(dead_code)]
enum Status {
    Done,
    InProcess,
    Expired,
}

pub fn header() {
    println!("{:>6}Task Tracker", "");
    println!("{:-<25}", "");
}

pub fn show_main_menu() {
    println!("Press to action");
    println!("1. Add new task");
    println!("2. Choose task");
    println!("3. Clear all");
    println!("0. Exit");
    println!("{:-<25}", "");
}

pub fn show_menu(status: &mut Menu) {
    header();
    match status {
        Menu::MainMenu => show_main_menu(),
        Menu::TaskMenu => {
            println!("Hello!");
        },
    }
}

pub fn change_status(input: String) -> Menu {
    let status = input.trim().parse().unwrap_or(-1);
    match status {
        1 => Menu::MainMenu,
        2 => Menu::TaskMenu,
        _ => {
            println!("Hello!");
            process::exit(1);
        }
    }
}
