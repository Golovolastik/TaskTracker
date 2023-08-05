use csv;
use std::error::Error;
use std::fmt;
use crate::menu::*;
use std::io;

pub mod menu;
pub mod date;

pub enum Menu {
    MainMenu,
    AddTask,
    TaskMenu,
}

pub enum AddTaskMenu {
    TaskDescription,
    Deadline
}

pub struct Task {
    task: String,
    deadline: String,
    status: String,
}

impl Task {
    fn new(task: String, deadline: String) -> Self {
        Task {
            task,
            status: "active".to_string(),
            deadline,
        }
    }

    fn destructure(&self) -> Vec<String> {
        vec![self.task.clone(), self.deadline.clone(), self.status.clone()]
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\nDeadline: {} Status: {}\n{:-<35}", self.task, self.deadline, self.status, "")
    }
}
fn init() -> (Menu, Vec<Task>) {
    let menu = Menu::MainMenu;
    let array = read_csv().expect("Can't read the file");
    (menu, array)
}

pub fn run() {
    let (mut menu, mut array) = init();
    loop {
        show_menu(&menu, &array);
        let mut input = String::from("");
        io::stdin().read_line(&mut input).expect("Can't read the input");
        menu = menu::handle_input(menu, input, &mut array);
    }
}

pub fn read_csv() -> Result<Vec<Task>, Box<dyn Error>> {
    let mut array = Vec::new();
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b';')
        .has_headers(false)
        .from_path("task.csv")?;
    for result in rdr.records() {
        let record = result?;
        let status = &record[2];
        let task = Task {
            task: String::from(&record[0]),
            deadline: String::from(&record[1]),
            status: status.to_string(),
        };
        array.push(task);
    }
    Ok(array)
}

fn write_to_csv(array: &mut Vec<Task>) -> Result<(), Box<dyn Error>> {
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(b';')
        .from_path("task.csv")?;
    for task in array {
        wtr.write_record(&task.destructure())?;
    }
    wtr.flush()?;
    Ok(())
}