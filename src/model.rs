use std::fs;
use std::env;

#[derive(Debug)]
pub struct Todo {
    id: u32,
    title: String,
    complete: bool,
}

pub struct TodoApp {
    todos: Vec<Todo>
}

impl TodoApp {
    pub fn new() -> TodoApp {
        TodoApp {
            todos: TodoApp::load_todos()
        }
    }

    pub fn load_todos() -> Vec<Todo> {
        let home_path = env::var("HOME").expect("home path is not set");
        let data_path = [home_path, ".todo".to_string()].join("/");
        let data_str = fs::read_to_string(data_path).unwrap_or(String::from(""));
        TodoApp::from_str(&data_str)
    }

    pub fn list_todos(&self) {
        if self.todos.len() > 0 {
            let content = self.to_string();
            println!("{}", content);
        } else {
            println!("There is no todos!");
        }
    }

    pub fn add_todo(&mut self, title: &str) {
        let idx = (self.todos.len() + 1) as u32;
        let new_todo = Todo {
            id: idx,
            title: title.to_string(),
            complete: false,
        };
        self.todos.push(new_todo);
    }

    pub fn complete_todo(&mut self, id: u32) {
        self.set_complete(id, true);
    }

    pub fn incomplete_todo(&mut self, id: u32) {
        self.set_complete(id, false);
    }

    pub fn edit_todo(&mut self, id: u32, title: &str) {
        self.todos.iter_mut().for_each(|item| {
            if item.id == id {
                item.title = String::from(title)
            }
        });
    }

    pub fn remove_todo(&mut self, id: u32) {
        match self.todos.iter().position(|x| x.id == id) {
            Some(idx) => {self.todos.remove(idx);},
            None => {}
        };
    }

    fn set_complete(&mut self, id: u32, complete: bool) {
        self.todos.iter_mut().for_each(|item| {
            if item.id == id {
                item.complete = complete
            }
        });
    }

    fn save_todos(&self) {
        let home_path = env::var("HOME").expect("home path is not set");
        let data_path = [home_path, ".todo".to_string()].join("/");
        let content = self.to_string();
        fs::write(data_path, content).expect("Save todos failed!");
    }

    pub fn from_str(content: &str) -> Vec<Todo> {
        content.lines()
            .filter(|line| line.trim() != "")
            .map(|line| Todo::from_str(line)).collect()
    }
}


impl ToString for TodoApp {
    fn to_string(&self) -> String {
        self.todos.iter().map(|item| item.to_string()).collect::<Vec<String>>().join("\n")
    }
}

impl Todo {
    pub fn from_str(s: &str) -> Todo {
        // 1. [ ] title 1
        // 2. [X] title 2
        let s = s.trim();
        let task_id = match s.split(". [").nth(0) {
            Some(val) => val.parse::<u32>().expect("invalid task id"),
            None => panic!("invalid task id"),
        };

        let complete = s.contains("[X]");
        let title = if complete {
            s.split("[X]").nth(1)
        } else {
            s.split("[ ]").nth(1)
        };

        let title = match title {
            Some(val) => val.trim().to_string(),
            None => panic!("invalid task title")
        };

        let todo = Todo {
            id: task_id,
            title,
            complete,
        };

        return todo;
    }
}

impl ToString for Todo {
    fn to_string(&self) -> String {
        let complete_sign = if self.complete {"[X]"} else {"[ ]"};
        format!("{}. {} {}", self.id, complete_sign, self.title.trim())
    }
}

#[cfg(test)]
mod tests {

    mod test_to_string {
        use super::super::*;

        #[test]
        fn incomplete_task() {
            let task_str = "1. [ ] test title";
            let todo = Todo::from_str(task_str);
            assert_eq!(todo.to_string(), task_str);
        }

        #[test]
        fn complete_task() {
            let task_str = "3. [X] test title 3";
            let todo = Todo::from_str(task_str);
            assert_eq!(todo.to_string(), task_str);
        }
    }

    mod test_from_str {
        use super::super::*;

        #[test]
        fn incomplete_task() {
            let todo = Todo::from_str("1. [ ] test title");
            assert_eq!(todo.id, 1);
            assert_eq!(todo.title, "test title");
            assert_eq!(todo.complete, false);
        }

        #[test]
        fn complete_task() {
            let todo = Todo::from_str("1. [X] test title");
            assert_eq!(todo.id, 1);
            assert_eq!(todo.title, "test title");
            assert_eq!(todo.complete, true);
        }

        #[test]
        #[should_panic]
        fn invalid_task() {
            Todo::from_str("1 X] test title");
        }

        #[test]
        #[should_panic]
        fn invalid_task_id() {
            Todo::from_str("+. [X] test title");
        }
    }
}
