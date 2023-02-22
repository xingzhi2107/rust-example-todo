use std::fs;
use std::env;

#[derive(Debug)]
pub struct Todo {
    id: u32,
    title: String,
    complete: bool,
}

impl Todo {
    pub fn load_todos() -> Vec<Todo> {
        let home_path = env::var("HOME").expect("home path is not set");
        let data_path = [home_path, ".todo".to_string()].join("/");
        let data_str = fs::read_to_string(data_path).unwrap_or(String::from(""));
        Todo::from_content(&data_str)
    }

    pub fn list_todos() {
        let todos = Todo::load_todos();
        if todos.len() > 0 {
            let content = Todo::todos_to_content(&todos);
            println!("{}", content);
        } else {
            println!("There is no todos!");
        }
    }

    pub fn add_todo(title: &str) {
        let mut todos = Todo::load_todos();
        let idx = (todos.len() + 1) as u32;
        let new_todo = Todo {
            id: idx,
            title: title.to_string(),
            complete: false,
        };
        todos.push(new_todo);
        Todo::save_todos(&todos);
    }

    pub fn complete_todo(id: u32) {
        Todo::set_complete(id, true);
    }

    pub fn incomplete_todo(id: u32) {
        Todo::set_complete(id, true);
    }

    pub fn edit_todo(id: u32, title: &str) {
        let mut todos = Todo::load_todos();

        todos.iter_mut().for_each(|item| {
            if item.id == id {
                item.title = String::from(title)
            }
        });

        Todo::save_todos(&todos);
    }

    pub fn remove_todo(id: u32) {
        let todos = Todo::load_todos().into_iter()
            .filter(|item| item.id != id)
            .collect::<Vec<Todo>>();
        Todo::save_todos(&todos);
    }

    fn set_complete(id: u32, complete: bool) {
        let mut todos = Todo::load_todos();

        todos.iter_mut().for_each(|item| {
            if item.id == id {
                item.complete = complete
            }
        });

        Todo::save_todos(&todos);
    }

    fn save_todos(todos: &Vec<Todo>) {
        let home_path = env::var("HOME").expect("home path is not set");
        let data_path = [home_path, ".todo".to_string()].join("/");
        let content = Todo::todos_to_content(todos);
        fs::write(data_path, content).expect("Save todos failed!");
    }

    fn todos_to_content(todos: &Vec<Todo>) -> String {
        todos.iter().map(|item| item.to_string()).collect::<Vec<String>>().join("\n")
    }

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

    pub fn from_content(content: &str) -> Vec<Todo> {
        content.lines()
            .filter(|line| line.trim() != "")
            .map(|line| Todo::from_str(line)).collect()
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
