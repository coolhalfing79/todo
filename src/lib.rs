use std::{
    fs::{self, File},
    io::{BufReader, Read},
};

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Todo {
    task: String,
    completed: bool,
}

pub fn add_todo(list: &mut Vec<Todo>, task: String) {
    list.push(Todo {
        task,
        completed: false,
    })
}

pub fn delete_todo(list: &mut Vec<Todo>, index: usize) {
    list.remove(index);
}

pub fn update_todo(list: &mut Vec<Todo>, index: usize, task: String) {
    if let Some(elem) = list.get_mut(index) {
        elem.task = task;
    }
}

pub fn mark_complete(list: &mut Vec<Todo>, index: usize) {
    match list.get_mut(index) {
        Some(task) => task.completed = true,
        None => panic!("[ERROR] task {index} not found."),
    }
}

pub fn show_todos(list: &Vec<Todo>) {
    let a = list
        .iter()
        .map(|x| x.task.len())
        .reduce(|accum, item| if accum > item { accum } else { item })
        .expect("[ERROR] Todo list empty");

    let len_tasks = "Tasks".len();
    print!("No. | Tasks");
    for _ in 0..(if a > len_tasks { a - len_tasks } else { 0 }) {
        print!(" ")
    }
    println!(" | Completed");
    print!("---- ");
    for _ in 0..(if a > len_tasks { a } else { len_tasks }) {
        print!("-")
    }
    println!("-- ----------");

    for (index, todo) in list.iter().enumerate() {
        print!("{}   | {}", index, todo.task);
        for _ in 0..(if a > len_tasks {
            a - todo.task.len()
        } else {
            len_tasks - todo.task.len()
        }) {
            print!(" ")
        }
        println!(" | {}", if todo.completed { "✔️" } else { "❌" });
    }
}

pub fn load_data(filename: String) -> Vec<Todo> {
    let file = match File::open(&filename) {
        Ok(file) => file,
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                File::create(&filename).expect("[ERROR] Could not create a file.");
                return vec![];
            }
            _ => todo!(),
        },
    };
    let mut buf_reader = BufReader::new(file);
    let mut content = String::new();
    buf_reader
        .read_to_string(&mut content)
        .expect("[ERRR] Could not read file.");
    serde_json::from_str(&content).expect("[ERROR] could not deserialise")
}

pub fn write_data(filename: String, list: &Vec<Todo>) {
    let content = serde_json::to_string(list).expect("[ERROR] Could not serialise to json");
    fs::write(filename, content).expect("[ERROR] Coud not write to file");
}

pub fn show_json(list: &Vec<Todo>) {
    let content = serde_json::to_string(list).expect("[ERROR] could not serialize to json.");
    println!("{}", content)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_todo() {
        let mut list: Vec<Todo> = vec![];
        add_todo(&mut list, "test".to_owned());
        let added_todo = list.pop().unwrap();
        assert_eq!(
            added_todo.task,
            "test".to_owned(),
            "todo is added correctly"
        );
    }

    #[test]
    fn test_delete_todo() {
        let mut list = vec![Todo {
            task: "test".to_owned(),
            completed: false,
        }];
        delete_todo(&mut list, 0);
        assert_eq!(list.len(), 0, "task is deleted correctly");
    }

    #[test]
    fn test_update_todo() {
        let mut list = vec![Todo {
            task: "test".to_owned(),
            completed: false,
        }];
        update_todo(&mut list, 0, "test changed".to_owned());
        let changed_todo = list.pop().unwrap();
        assert_eq!(
            changed_todo.task,
            "test changed".to_owned(),
            "task is updated correctly"
        );
    }

    #[test]
    fn test_mark_complete() {
        let mut list = vec![Todo {
            task: "test".to_owned(),
            completed: false,
        }];
        mark_complete(&mut list, 0);
        let completed_todo = list.pop().unwrap();
        assert_eq!(
            completed_todo.completed, true,
            "task is marked as completed"
        );
    }
}
