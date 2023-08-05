use crate::{Menu, Task, read_csv, AddTaskMenu};
use std::process;

fn header() {
    println!("{:-<25}", "");
    println!("{:>6}Task Tracker", "");
    println!("{:-<25}", "");
}

pub fn show_menu(menu: &Menu, array: &Vec<Task>) {
    header();
    match menu {
        Menu::MainMenu => show_main_menu(),
        Menu::TaskMenu => show_task_menu(array),
        Menu::AddTask(_) => println!("bye"),
    }
}

fn show_main_menu() {
    println!("Press to action");
    println!("1. Add new task");
    println!("2. Show tasks");
    println!("0. Exit");
    println!("{:-<25}", "");
}

fn show_task_menu(array: &Vec<Task>) {
    println!("Press to action");
    println!("0. Exit to main menu");
    println!("Enter the number of task to mark it as done");
    println!("{:-<25}", "");
    show_tasks(&array);
}

fn show_tasks(array: &Vec<Task>) {
    for (idx, task) in array.iter().enumerate() {
        println!("{}. {}", idx+1, task);
    }
}

pub fn handle_input(menu: Menu, input: String, array: &mut Vec<Task>) -> Menu {
    match menu {
        Menu::MainMenu => main_menu_input(input),
        Menu::AddTask(_) => {todo!()},
        Menu::TaskMenu => task_menu_input(input, array),
    }
}

fn add_task_menu_input(input: &str, array: &mut Vec<Task>) -> Menu {
    println!("Write task description.");

    Menu::MainMenu
}

fn main_menu_input(input: String) -> Menu {
    match input.trim() {
        "1" => Menu::AddTask(AddTaskMenu::TaskDescription),
        "2" => Menu::TaskMenu,
        "0" => process::exit(1),
        _ => {
            println!("Wrong input! Try again.");
            Menu::MainMenu
        }
    }
}

fn task_menu_input(input: String, array: &mut Vec<Task>) -> Menu {
    match input.trim() {
        "0" => Menu::MainMenu,
        str => {
            delete_from_list(str, array);
            Menu::TaskMenu
        },
        _ => {
            println!("Wrong input! Try again.");
            Menu::TaskMenu
        },
    }
}

fn check_task_input(input: &str) -> bool{
    if let Err(_) = input.parse::<i32>() {
        println!("Wrong input! Try again.");
        return false;
    } else if input.parse::<i32>().unwrap() < 0 {
        println!("Negative number! Try again.");
        return false;
    }
    true
}

fn delete_from_list(input: &str, array: &mut Vec<Task>) {
    if !check_task_input(input) {
        return;
    }
    let idx:usize = input.parse().unwrap();
    if idx > array.len() {
        println!("Too big number!");
        return;
    } else {
        array.remove(idx-1);
    }
}