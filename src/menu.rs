use crate::{Menu, Task, read_csv};
use std::process;

pub fn header() {
    println!("{:>6}Task Tracker", "");
    println!("{:-<25}", "");
}

pub fn show_menu(menu: &Menu) {
    header();
    match menu {
        Menu::MainMenu => show_main_menu(),
        Menu::TaskMenu => {
            show_tasks(&read_csv().expect("Can't read the file"));
        },
        _ => println!("bye"),
    }
}

fn show_main_menu() {
    println!("Press to action");
    println!("1. Add new task");
    println!("2. Show tasks");
    println!("0. Exit");
    println!("{:-<25}", "");
}

fn show_tasks(array: &Vec<Task>) {
    for (idx, task) in array.iter().enumerate() {
        println!("{}. {}", idx+1, task);
    }
}

pub fn handle_input(menu: Menu, input: String) -> Menu {
    match menu {
        Menu::MainMenu => {
            main_menu_input(input)
        },
        Menu::AddTask => {todo!()},
        Menu::TaskMenu => {todo!()},
        Menu::TaskEdit => {todo!()},
    }
}

fn main_menu_input(input: String) -> Menu {
    match input.trim() {
        "1" => Menu::AddTask,
        "2" => Menu::TaskMenu,
        "0" => process::exit(1),
        _ => {
            println!("Wrong input! Try again.");
            Menu::MainMenu
        }
    }
}