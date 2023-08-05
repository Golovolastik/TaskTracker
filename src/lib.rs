use time;
use csv;
use std::error::Error;
use std::fmt;
use std::fs::File;
use crate::menu::*;
use std::io;

pub mod menu;

pub enum Menu {
    MainMenu,
    AddTask(AddTaskMenu),
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
    fn new(task: String, deadline: u8) -> Self {
        Task {
            task,
            status: "active".to_string(),
            deadline: "construct_date()".to_string(),
        }
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
        //println!("{}", clear::All);
        show_menu(&menu, &array);
        let mut input = String::from("");
        io::stdin().read_line(&mut input).expect("Can't read the input");
        menu = menu::handle_input(menu, input, &mut array);
    }
}

fn construct_date(offset: u8) -> String {

    "1".to_string()
}

pub fn read_csv() -> Result<Vec<Task>, Box<dyn Error>> {
    let file_path = String::from("task.csv");
    let file = File::open(file_path)?;
    let mut array = Vec::new();
    let mut rdr = csv::ReaderBuilder::new().delimiter(b';').from_path("task.csv")?;
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

#[cfg(test)]
mod tests {
    #[test]
    fn date_construction() {
        println!("{}", time::OffsetDateTime::now_utc());
        assert!(time::OffsetDateTime::now_utc().year() >= 2022);
    }
}