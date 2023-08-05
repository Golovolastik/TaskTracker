use crate::{Menu, Task, write_to_csv};
use std::process;
use std::io;
use crate::date::construct_date;

pub fn handle_input(menu: Menu, input: String, array: &mut Vec<Task>) -> Menu {
    match menu {
        Menu::MainMenu => main_menu_input(input, array),
        Menu::AddTask => {
            new_task_input(input, array);
            Menu::MainMenu
        },
        Menu::TaskMenu => task_menu_input(input, array),
    }
}

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
        Menu::AddTask => println!("Write task description"),
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

fn show_offset_menu() {
    println!("Set deadline");
    println!("1. Today");
    println!("2. Tomorrow");
    println!("3. 3 days");
    println!("4. 1 week");
    println!("5. 2 weeks");
    println!("6. 1 month");
}

fn deadline_input() -> u8 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Can't read the input");
    match input.trim() {
        "1" => 0,
        "2" => 1,
        "3" => 3,
        "4" => 7,
        "5" => 14,
        "6" => 30,
        _ => 1,
    }
}

fn show_tasks(array: &Vec<Task>) {
    for (idx, task) in array.iter().enumerate() {
        println!("{}. {}", idx+1, task);
    }
}

fn new_task_input(input: String, array: &mut Vec<Task>) {
    let description = input.trim().to_string();
    show_offset_menu();
    let offset = deadline_input();
    let deadline = construct_date(offset);
    array.push(Task::new(description, deadline));
}

fn main_menu_input(input: String, array: &mut Vec<Task>) -> Menu {
    match input.trim() {
        "1" => Menu::AddTask,
        "2" => Menu::TaskMenu,
        "0" => {
            if let Err(_) = write_to_csv(array) {
                panic!("Can't write data to file!");
            }
            process::exit(1)
        },
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