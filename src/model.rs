use std::fs;
use std::fs::OpenOptions;
use std::env::*;

struct Todo {
    id: u32,
    title: String,
    complete: bool,
}

impl Todo {
    fn get_todos() -> Vec<Todo> {
        let home_path = std::env::var("HOME").expect("home path is not set");
        let data_path = [home_path, ".todo".to_string()].join("/");
        let data_str = fs::read_to_string(data_path).unwrap_or(String::from(""));

        data_str.lines().map(|line| Todo::from_str(line)).collect()
    }

    fn from_str(s: &str) -> Todo {
        // 1. [ ] title 1
        // 2. [X] title 2
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

#[cfg(test)]
mod tests {
    use super::*;

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
