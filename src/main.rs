use clap::{arg, Command};
use std::env;
use std::process;
const ADD: &str = "add";
const DONE: &str = "done";
const DELETE: &str = "delete";
const LIST: &str = "list";
const RECORD: &str = "record";
const FILE_NAME: &str = ".todo";

const TODO: &str = "\u{2610}";
const DON: &str = "\u{2611}";

#[derive(Debug, Clone, PartialEq)]

struct Todo {
    done: bool,
    task: String,
    time: Option<f32>,
}
fn log_file_path() -> String {
    match env::var("HOME") {
        Ok(val) => [&val, FILE_NAME].join("/"),
        Err(_) => format!("./{}", FILE_NAME), 
    }
}

fn format(todo: &Vec<Todo>) {
    let mut a = 0;
    for t in todo {
        a += 1;
        if t.done == true {
            println!("{} {:03}: {:?}", DON, a - 1, t);
        } else {
            println!("{} {:03}: {:?}", TODO, a - 1, t);
        };
    }
}


fn main() {
    let command = Command::new("todo")
        .version("0.1.0")
        .about("simple command-line todo list")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(
            Command::new(ADD)
                .about("add the task")
                .arg(arg!(<TASK>).required(true)),
        )
        .subcommand(
            Command::new(DONE)
                .about("done the task")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(
            Command::new(DELETE)
                .about("delete the task")
                .arg(arg!(<INDEX>).required(true)),
        )
        .subcommand(Command::new(LIST).about("show todo list"))
        .subcommand(
            Command::new(RECORD)
                .about("record elapsed time")
                .arg(arg!(<INDEX>).required(true))
                .arg(arg!(<TIME>).required(true)),
        );
    let mut todo = vec![
        Todo {
            done: false,
            task: "sport".to_string(),
            time: Some(2.0),
        },
        Todo {
            done: false,
            task: "biology".to_string(),
            time: Some(9.0),
        },
        Todo {
            done: false,
            task: "programming".to_string(),
            time: None,
        },
    ];

    match command.get_matches().subcommand().unwrap() {
        (ADD, s) => {
            let subject = s.value_of("TASK").unwrap();
            let t = Todo {
                done: false,
                task: subject.to_string(),
                time: None,
            };
            todo.push(t);
            format(&todo);
        }

        (DONE, i) => {
            let index = i
                .value_of("INDEX")
                .unwrap()
                .parse::<u32>()
                .unwrap_or_else(|_| {
                    eprintln!("failed, <INDEX> should be integer");
                    process::exit(1);
                });

            let mut a = 0;

            for t in &mut todo {
                if a  == index {
                    t.done = true;
                }
                a += 1;
            }
            format(&todo);

        }
        (DELETE, i) => {
            let index = i
                .value_of("INDEX")
                .unwrap()
                .parse::<usize>()
                .unwrap_or_else(|_| {
                    eprintln!("failed, <INDEX> should be integer");
                    process::exit(1);
                });

            todo.remove(index);
            format(&todo);
        }
        (LIST, _) => {
            format(&todo);
        }
        (RECORD, it) => {
            let index = it
                .value_of("INDEX")
                .unwrap()
                .parse::<u32>()
                .unwrap_or_else(|_| {
                    eprintln!("failed, <INDEX> should be integer");
                    process::exit(1);
                });
            let time = it
                .value_of("TIME")
                .unwrap()
                .parse::<f32>()
                .unwrap_or_else(|_| {
                    eprintln!("failed, <TIME> should be float");
                    process::exit(1);
                });
            let mut a = 0;
            for t in &mut todo {
                if a == index {
                    t.time = Some(time);
                }
                a += 1;
            }
            format(&todo);
        }

        _ => unreachable!(),
    };
}
