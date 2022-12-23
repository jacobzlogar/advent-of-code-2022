use std::collections::{VecDeque, HashMap};
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
    sub_directories: HashMap<String, Self>,
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
    output: Vec<String>
}

#[derive(Debug, Clone)]
struct FileSystem {
    directories: VecDeque<Directory>,
    current_directory: Option<Directory>,
    last_directory: Option<Directory>,
}

fn main() -> Result<()> {
    let data = Helper::new(String::from("day7"))
        .get_input()
        .expect("no day7 input")
        .to_vec('\n');

    let mut fs = FileSystem {
        directories: VecDeque::new(),
        current_directory: None,
        last_directory: None,
    };
    data.clone()
        .into_iter()
        .enumerate()
        .try_for_each(|(idx, val): (usize, String)| -> Result<()> {
            if val.starts_with("$") {
                let cmd = parse_command(val.clone());
                let output = run_command(cmd, fs.clone(), data.clone(), idx.clone())
                    .expect("invalid command");
                fs = output;
            }
            Ok(())
        })?;

    dbg!(&fs.directories[0]);

    Ok(())
}

fn run_command(cmd: Command, mut fs: FileSystem, data: Vec<String>, idx: usize) -> Result<FileSystem> {
    match cmd.command_type {
        CommandType::Cd => {
            let current = fs.current_directory.clone();
            let dir = Directory {
                name: cmd.input.as_ref().unwrap().to_string(),
                sub_directories: HashMap::new(),
                files: vec![],
            };
            // push the latest directory to the front
            fs.directories.push_front(dir.clone());
            if fs.current_directory.clone().is_some() {
                fs.last_directory = Some(fs.current_directory.clone().unwrap());
            }
            fs.current_directory = Some(dir);
        },
        CommandType::Ls => {
            let next = data
                .clone()
                .into_iter()
                .skip(idx + 1)
                .take_while(|i| {
                    !i.starts_with("$")
                })
                .collect::<Vec<String>>();

            let (directories, files) = get_output(next, fs.current_directory.clone());
            let c = Directory {
                sub_directories: directories.clone(),
                files,
                ..fs.current_directory.clone().unwrap()
            };
            if c.name.eq(&fs.current_directory.clone().unwrap().name) {
                let mut curr = fs.current_directory.clone().unwrap();
                for dir in directories {
                    curr.sub_directories.insert(dir.0, dir.1);
                }
            }
            dbg!(&fs.current_directory);
            // dbg!(&fs.last_directory);
            dbg!(&c);
        }
    }
    Ok(fs)
}

fn parse_command(data: String) -> Command {
    let val: Vec<&str> = data.split_whitespace().collect();
    let cmd_type = CommandType::from(val[1].to_string());
    Command {
        command_type: cmd_type,
        input: if val.len().eq(&3) { Some(val[2].to_string()) } else { None },
        output: vec![]
    }
}


fn get_output(data: Vec<String>, current: Option<Directory>) -> (HashMap<String, Directory>, Vec<File>) {
    let mut directories = HashMap::new();
    let mut files = vec![];
    data.into_iter().for_each(|line| {
        let parsed = line.split_whitespace().collect::<Vec<&str>>();
        match parsed[0].parse::<usize>() {
            Ok(v) => {
                files.push(File {
                    name: parsed[1].to_string(),
                    size: v,
                })
            },
            Err(_) => ()
        }
        match parsed[0] {
            "dir" => {
                directories.insert(parsed[1].to_string(), Directory {
                    name: parsed[1].to_string(),
                    sub_directories: HashMap::new(),
                    files: vec![],
                });
            },
            _ => ()
        };
    });
    (directories, files)
}
