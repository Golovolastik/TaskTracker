use time;
use csv;
use std::error::Error;
use std::fmt;
use std::fs::File;

pub mod menu;

pub enum Menu {
    MainMenu,
    AddTask,
    TaskMenu,
    TaskEdit,
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

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\nDeadline: {} Status: {}\n{:-<35}", self.task, self.deadline, self.status, "")
    }
}

// pub fn change_status() -> Menu {
//     match menu {
//         1 => Menu::MainMenu,
//         2 => Menu::TaskMenu,
//         _ => {
//             println!("Hello!");
//             process::exit(1);
//         }
//     }
// }

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
