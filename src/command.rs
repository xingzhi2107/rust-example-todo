use crate::model::{TodoApp};



pub fn deal_input(todo_app: &mut TodoApp, input: &str) {
    let mut commands: Vec<Box<dyn Command>> = vec![
        Box::new(HelpCommand {}),
        Box::new(ListCommand {todo_app}),
        Box::new(AddCommand {todo_app}),
        // Box::new(EditCommand {}),
        // Box::new(CompleteCommand {}),
        // Box::new(IncompleteCommand {}),
        // Box::new(RemoveCommand {}),
        Box::new(QuitCommand {}),
    ];
    let input = input.trim();
    for cmd in commands.iter_mut() {
        let is_hint = cmd.match_command(input);
        if is_hint {
            cmd.exec_command(input);
            return
        }
    }
    panic!("Invalid input");
}

pub trait Command {
    fn match_command(&self, cmd_str: &str) -> bool;
    fn exec_command(&mut self, cmd_str: &str);
}

struct HelpCommand;
impl Command for HelpCommand {
    fn match_command(&self, cmd_str: &str) -> bool {
        cmd_str == "help"
    }

    fn exec_command(&mut self, _cmd_str: &str) {
        println!("\
Commands:
    help        print this help text
    list        list all task
    add         add new task, example 'add new task title'
    edit        edit exists task, example 'edit 1 new task title for 1'
    complete    complete task
    incomplete  incomplete task
    remove      remove task
    quit        quit
        ")
    }
}

struct ListCommand<'a> {
    todo_app: &'a TodoApp
}

impl Command for ListCommand<'_> {
    fn match_command(&self, cmd_str: &str) -> bool {
        cmd_str == "list"
    }

    fn exec_command(&mut self, _cmd_str: &str) {
        self.todo_app.list_todos()
    }
}

struct AddCommand<'a> {
    todo_app: &'a mut TodoApp
}
impl Command for AddCommand<'_> {
    fn match_command(&self, cmd_str: &str) -> bool {
        cmd_str.starts_with("add ")
    }

    fn exec_command(&mut self, cmd_str: &str) {
        let new_title = cmd_str.replace("add ", "");
        self.todo_app.add_todo(&new_title);
    }
}

// struct EditCommand;
// impl Command for EditCommand {
//     fn match_command(&self, cmd_str: &str) -> bool {
//         cmd_str.starts_with("edit ")
//     }
//
//     fn exec_command(&self, cmd_str: &str) {
//         let args_str = cmd_str.replace("edit ", "");
//         let id_str = args_str.trim().split(" ").nth(0).expect("Invalid edit command");
//         let title_str = args_str.trim().split(" ").nth(1).expect("Invalid edit command");
//         let id = id_str.trim().parse::<u32>().expect("Invalid task id: {}");
//         Todo::edit_todo(id, title_str);
//     }
// }
//
// struct CompleteCommand;
// impl Command for CompleteCommand {
//     fn match_command(&self, cmd_str: &str) -> bool {
//         cmd_str.starts_with("complete ")
//     }
//
//     fn exec_command(&self, cmd_str: &str) {
//         let id_str = cmd_str.replace("complete ", "");
//         let id = id_str.trim().parse::<u32>().expect("Invalid task id: {}");
//         Todo::complete_todo(id);
//     }
// }
//
// struct IncompleteCommand;
// impl Command for IncompleteCommand {
//     fn match_command(&self, cmd_str: &str) -> bool {
//         cmd_str.starts_with("remove ")
//     }
//
//     fn exec_command(&self, cmd_str: &str) {
//         let id_str = cmd_str.replace("remove ", "");
//         let id = id_str.trim().parse::<u32>().expect("Invalid task id: {}");
//         Todo::remove_todo(id);
//     }
// }
//
// struct RemoveCommand;
// impl Command for RemoveCommand {
//     fn match_command(&self, cmd_str: &str) -> bool {
//         cmd_str.starts_with("incomplete ")
//     }
//
//     fn exec_command(&self, cmd_str: &str) {
//         let id_str = &cmd_str[11..];
//         let id = id_str.parse::<u32>().expect("Invalid task id: {}");
//         Todo::incomplete_todo(id);
//     }
// }
//
struct QuitCommand;
impl Command for QuitCommand {
    fn match_command(&self, cmd_str: &str) -> bool {
        cmd_str.starts_with("quit")
    }

    fn exec_command(&mut self, _cmd_str: &str) {
        std::process::exit(0)
    }
}
