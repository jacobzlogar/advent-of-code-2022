use std::collections::VecDeque;
use aoc_2022::{Result, Helper};

#[derive(Debug)]
enum CommandType {
    Cd,
    Ls
}

// just gonna pretend like it always works
impl From<String> for CommandType {
    fn from(val: String) -> CommandType {
        match val.as_str() {
            "cd" => CommandType::Cd,
            _ => CommandType::Ls,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Directory {
    name: String,
    sub_directories: Vec<Self>,
    files: Vec<File>
}

#[derive(Debug, Clone, PartialEq)]
struct File {
    size: usize,
    name: String
}

#[derive(Debug)]
struct Command {
    command_type: CommandType,
    input: Option<String>,
    output: Option<Vec<String>>
}

#[derive(Debug, Clone)]
struct FileSystem {
    directories: VecDeque<Directory>,
    current_directory: Option<Directory>,
}

fn main() {
    let data = Helper::new(String::from("day7"))
        .get_input()
        .expect("no day7 input")
        .to_vec('\n');

    let mut fs = FileSystem {
        directories: VecDeque::new(),
        current_directory: None,
    };
    let result: Vec<Command> = data.clone().into_iter().enumerate().filter_map(|(idx, val)| {
        if val.starts_with("$") {
            let x: Vec<&str> = val.split_whitespace().collect();
            let cmd_type = CommandType::from(x[1].to_string());
            let cmd = Command {
                command_type: cmd_type,
                input: if x.len().eq(&3) { Some(x[2].to_string()) } else { None },
                output: Some(vec![])
            };

            match cmd.command_type {
                CommandType::Cd => {
                    let dir = Directory {
                        name: cmd.input.as_ref().unwrap().to_string(),
                        sub_directories: vec![],
                        files: vec![],
                    };
                    // push the latest directory to the front
                    fs.directories.push_front(dir.clone());
                    fs.current_directory = Some(dir);
                },
                CommandType::Ls => {
                    let next = data.clone().into_iter().skip(idx).collect::<Vec<String>>();
                    let (directories, files) = get_output(next);
                    fs.current_directory.as_mut().unwrap().sub_directories = directories;
                    fs.directories[0] = fs.current_directory.clone().unwrap();
                    // if len fs.directories > 1 we must be in a child directory right?
                    // update the parent directory fs.directories[1]
                }
            }


            // if cd store the directory name, only update after another cd
            // handle .. ?
            // Directory { name: "/" }
            // ls, grab the lines until the next command, add directories and contents to current directory
            // Directory { name: "/", sub_directories: [{ nane: "blgtdv", contents: [], sub_directories: [] }, {name: "dbrfcz"}], contents: [] }

            return Some(cmd);
        }
        None
    }).collect();
    dbg!(&fs);
}


fn get_output(data: Vec<String>) -> (Vec<Directory>, Vec<File>) {
    let mut directories = vec![];
    let mut files = vec![];
    data.into_iter().for_each(|line| {
        let parsed = line.split_whitespace().collect::<Vec<&str>>();
        match parsed[0].parse::<usize>() {
            Ok(v) => {files.push(File {
                name: parsed[1].to_string(),
                size: v,
            })},
            Err(_) => ()
        }
        match parsed[0] {
            "dir" => {
                directories.push(Directory {
                    name: parsed[1].to_string(),
                    sub_directories: vec![],
                    files: vec![],
                })
            },
            _ => ()
        };
    });
    (directories, files)
}
