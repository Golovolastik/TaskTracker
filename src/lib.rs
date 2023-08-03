use std::process;
use time;
use csv;
use std::error::Error;
use std::fmt;
use std::fs::File;

pub enum Menu {
    MainMenu,
    TaskMenu,
}

pub struct Task {
    task: String,
    deadline: String,
    status: String,
}

#[derive(Debug)]
enum Status {
    Done,
    Active,
    Expired,
}

impl fmt::Debug for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Task")
            .field("task", &self.task)
            .field("date", &self.deadline)
            .field("status", &self.status)
            .finish()
    }
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

pub fn read_csv() -> Result<(), Box<dyn Error>> {
    let file_path = String::from("task.csv");
    let file = File::open(file_path)?;
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_path("task.csv")?;
    let mut array: Vec<Task> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        let status = &record[2];
        let task = Task {
            task: String::from(&record[0]),
            deadline: String::from(&record[1]),
            status: status.to_string(),
        };
        dbg!(&task);
        array.push(task);


    }
    Ok(())
}
