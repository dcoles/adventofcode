//! Advent of Code 2022: Day 7
//! https://adventofcode.com/2022/day/7

use std::collections::HashMap;
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    let input = Input::from_file(format!("{}/input.txt", env!("CARGO_MANIFEST_DIR"))).expect("failed to read input");

    // Part 1
    println!("Part 1: {}", part1(&input));

    // Part 2
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &Input) -> usize {
    let dirs = walk_tree(&input);
    let dir_sizes = calculate_directory_sizes(&dirs);

    let mut sum = 0;
    for path in dir_sizes.keys() {
        let size = *dir_sizes.get(path).unwrap();
        if size <= 100000 {
            sum += size;
        }
    }

    sum
}

fn part2(input: &Input) -> usize {
    let dirs = walk_tree(&input);
    let dir_sizes = calculate_directory_sizes(&dirs);

    let free = 70000000 - dir_sizes.get(&PathBuf::from("/")).unwrap();
    let short = 30000000 - free;
    let mut candidates: Vec<_> = dir_sizes.iter().filter(|(_, &size)| size > short).collect();
    candidates.sort_by_key(|(_, size)| **size);

    *candidates[0].1
}

#[derive(Debug, Clone)]
enum DirEnt {
    Directory(String),
    File(String, usize),
}

fn dir_size(dirs: &HashMap<PathBuf, Vec<DirEnt>>, path: &Path) -> usize {
    let mut total_size = 0;

    for dirent in dirs.get(path).unwrap() {
        match dirent {
            DirEnt::Directory(name) => {
                let mut subdir = path.to_path_buf();
                subdir.push(name);
                total_size += dir_size(dirs, &subdir);
            },
            DirEnt::File(_, size) => total_size += *size,
        }
    }
    
    total_size
}

fn walk_tree(input: &Input) -> HashMap<PathBuf, Vec<DirEnt>> {
    let mut dirs: HashMap<PathBuf, Vec<DirEnt>> = HashMap::new();

    let mut pwd = PathBuf::from("/");
    for task in &input.tasks {
        match task.command[0].as_str() {
            "cd" => {
                let path = PathBuf::from(&task.command[1]);
                for segment in path.components() {
                    match segment {
                        std::path::Component::ParentDir => pwd = pwd.parent().unwrap().to_path_buf(),
                        std::path::Component::Normal(name) => pwd.push(name),
                        _ => (),
                    }
                }
            },
            "ls" => for line in &task.output {
                let (info, name) = line.split_once(" ").unwrap();
                if info == "dir" {
                    // Directory
                    dirs.entry(pwd.clone()).or_default().push(DirEnt::Directory(name.to_string()));
                } else {
                    // File
                    let size = info.parse::<usize>().unwrap();
                    dirs.entry(pwd.clone()).or_default().push(DirEnt::File(name.to_string(), size));
                }
            },
            _ => (),
        }
    }

    dirs
}

fn calculate_directory_sizes(dirs: &HashMap<PathBuf, Vec<DirEnt>>) -> HashMap<PathBuf, usize> {
    let mut dir_sizes: HashMap<PathBuf, usize> = HashMap::new();

    for path in dirs.keys() {
        dir_sizes.entry(path.to_path_buf()).or_insert_with(|| dir_size(&dirs, path.as_path()));
    }

    dir_sizes
}

#[derive(Debug, Clone)]
struct Input {
    tasks: Vec<Task>,
}

impl Input {
    fn from_file(path: impl AsRef<Path>) -> io::Result<Self> {
        let input = fs::read_to_string(path)?;

        let mut tasks = Vec::new();
        let mut task: Option<Task> = None;
        for line in input.lines() {
            if line.starts_with("$ ") {
                if let Some(command) = task {
                    tasks.push(command);
                }

                task = Some(Task {
                    command: line.split_ascii_whitespace().skip(1).map(str::to_string).collect(),
                    output: Vec::new(),
                });
            } else {
                let command = task.as_mut().expect("output before command?");
                command.output.push(line.to_string());
            }
        }

        if let Some(command) = task {
            tasks.push(command);
        }

        Ok(Input { tasks })
    }
}

#[derive(Debug, Clone)]
struct Task {
    command: Vec<String>,
    output: Vec<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part1(&input), 95437);
    }

    #[test]
    fn test_part2() {
        let input = Input::from_file("example1.txt").unwrap();

        assert_eq!(part2(&input), 24933642);
    }
}
