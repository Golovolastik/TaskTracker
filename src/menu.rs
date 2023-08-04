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
        Menu::TaskMenu => show_task_menu(),
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

fn show_task_menu() {
    println!("Press to action");
    println!("0. Exit to main menu");
    println!("Enter the number of task to make it done");
    println!("{:-<25}", "");
    show_tasks(&read_csv().expect("Can't read the file"));
}

fn show_tasks(array: &Vec<Task>) {
    for (idx, task) in array.iter().enumerate() {
        println!("{}. {}", idx+1, task);
    }
}

pub fn handle_input(menu: Menu, input: String) -> Menu {
    match menu {
        Menu::MainMenu => main_menu_input(input),
        Menu::AddTask => {todo!()},
        Menu::TaskMenu => task_menu_input(input),
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

fn task_menu_input(input: String) -> Menu {
    match input.trim() {
        "0" => Menu::MainMenu,
        str => {
            check_task_input(str)
            //Menu::TaskMenu
        },
        _ => {
            println!("Wrong input! Try again.");
            Menu::TaskMenu
        },
    }
}

fn check_task_input(input: &str) -> Menu {
    if let Err(_) = input.parse::<i32>() {
        println!("Wrong input! Try again.");
        return Menu::TaskMenu;
    }
    Menu::TaskMenu
}