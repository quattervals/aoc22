use std::collections::HashMap;

pub fn day_7_greeter() {
    println! {"Day 7"};

    looper("./inputs/7_example.txt");
}

fn looper(file_name: &str) {
    if let Ok(lines) = crate::utils::read_lines(file_name) {
        for line in lines {
            if let Ok(items) = line {
                println!("line {:?}", items);
            }
        }
    }
}

use regex::Regex;
fn split_file_listing(line: &str) -> (usize, String) {
    let re = Regex::new(r"(^[0-9]*)\s+([^']+)").unwrap();
    let caps = re.captures(line).unwrap();

    (caps[1].parse::<usize>().unwrap(), String::from(&caps[2]))
}

#[derive(Debug, PartialEq)]
enum ParserStates {
    Cd,
    Ls,
    Limbo,
}
struct Parser<'p> {
    state: ParserStates,
    dir: &'p mut Dir,
    current_dir: String,
}

impl<'p> Parser<'p> {
    fn new(dir: &'p mut Dir) -> Parser<'p> {
        Parser {
            state: ParserStates::Limbo,
            dir,
            current_dir: String::from(""),
        }
    }

    fn parse_line(&mut self, line: &str) {
        if self.state == ParserStates::Limbo && line.contains("$ cd ") {
            self.state = ParserStates::Cd;
            self.current_dir = crate::utils::string_from_n_to_end(line, 5);
            self.dir.name = crate::utils::string_from_n_to_end(line, 5);
        } else if self.state != ParserStates::Cd && line.contains("$ cd ") {
            self.state = ParserStates::Cd;
            self.current_dir = crate::utils::string_from_n_to_end(line, 5);
        } else if line.contains("ls") {
            self.state = ParserStates::Ls;
        } else if self.state == ParserStates::Ls {
            let current_dir: &mut Dir = self.dir.cd(&self.current_dir).unwrap();

            if line.contains("dir") {
                current_dir.add_dir(&crate::utils::string_from_n_to_end(line, 4));
            } else {
                let (file_size, file_name) = split_file_listing(line);

                current_dir.add_file(&file_name, file_size as u32);
            }
        }
    }
}

struct File {
    name: String,
    size: u32,
}

struct Dir {
    name: String,
    files: Vec<File>,
    sub_dirs: HashMap<String, Dir>,
}

impl Dir {
    fn add_file(&mut self, name: &str, size: u32) {
        self.files.push(File {
            name: String::from(name),
            size,
        });
    }
    fn make_empty_dir(i_name: &str) -> Dir {
        Dir {
            name: String::from(i_name),
            files: Vec::new(),
            sub_dirs: HashMap::new(),
        }
    }
    fn add_dir(&mut self, i_name: &str) {
        self.sub_dirs
            .insert(String::from(i_name), Dir::make_empty_dir(i_name));
    }
    fn size_of_files_in_dir(&self) -> u32 {
        self.files.iter().map(|x| x.size).sum()
    }

    fn record_size_of_dir_including_subdirs(&self, size_count: &mut HashMap<String, u32>) -> u32 {
        let mut size_of_subdirs: u32 = 0;
        for (_, subdir) in &self.sub_dirs {
            size_of_subdirs += subdir.record_size_of_dir_including_subdirs(size_count);
        }

        let size_of_dir = self.size_of_files_in_dir() + size_of_subdirs;
        size_count.insert(String::from(&self.name), size_of_dir);
        return size_of_dir;
    }

    fn cd(&mut self, i_name: &str) -> Option<&mut Dir> {
        if self.name == i_name {
            return Some(self);
        } else {
            for (_, subdir) in &mut self.sub_dirs {
                match subdir.cd(i_name) {
                    Some(x) => return Some(x),
                    None => todo!(),
                }
            }
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn navigator_cd() {
        let mut dir = Dir::make_empty_dir("");
        let mut prs = Parser::new(&mut dir);
        let mut accountant: HashMap<String, u32> = HashMap::new();

        prs.parse_line("$ cd /");
        assert_eq!(ParserStates::Cd, prs.state);

        prs.parse_line("$ ls");
        assert_eq!(ParserStates::Ls, prs.state);
        prs.parse_line("dir a");
        prs.parse_line("99 b.txt");

        assert_eq!(
            99,
            prs.dir
                .record_size_of_dir_including_subdirs(&mut accountant)
        );
        assert_eq!(99, *accountant.get("/").unwrap());

        prs.parse_line("$ cd a");
        assert_eq!(ParserStates::Cd, prs.state);
        prs.parse_line("$ ls");
        assert_eq!(ParserStates::Ls, prs.state);

        prs.parse_line("dir e");
        prs.parse_line("23 f");
        prs.parse_line("33 g");
        let current_dir = prs.dir.cd("a").unwrap();
        assert_eq!(23 + 33, current_dir.size_of_files_in_dir());

        prs.parse_line("$ cd e");
        prs.parse_line("$ ls");
        prs.parse_line("584 i");
        let current_dir = prs.dir.cd("e").unwrap();
        assert_eq!(584, current_dir.size_of_files_in_dir());

    }

    #[test]
    fn split_file_listings() {
        assert_eq!(
            (8345, String::from("file.ending")),
            split_file_listing("8345 file.ending")
        );
    }

    #[test]
    fn adding_root_dir() {
        let dir = Dir::make_empty_dir("root");
        assert_eq!("root", dir.name);
        assert_eq!(0, dir.files.len());
        assert_eq!(0, dir.sub_dirs.len());
    }

    #[test]
    fn adding_files_and_dirs() {
        let mut dir = Dir::make_empty_dir("root");

        dir.add_file("eins", 44);
        dir.add_file("zwei", 34);
        dir.add_dir("new_dir");
        assert_eq!(2, dir.files.len());
        assert_eq!(1, dir.sub_dirs.len());
    }

    #[test]
    fn cd_to_a_dir_add_files_and_dirs() {
        let mut dir = Dir::make_empty_dir("root");
        dir.add_file("eins", 44);
        dir.add_dir("new_dir");
        let current_dir = dir.cd("new_dir").unwrap();
        current_dir.add_file("sub_eins", 34);

        assert_eq!(1, current_dir.files.len());
    }
    #[test]
    fn calc_size_of_files() {
        let mut dir = Dir::make_empty_dir("root");

        dir.add_file("eins", 44);
        dir.add_file("zwei", 34);
        dir.add_dir("new_dir");

        assert_eq!(34 + 44, dir.size_of_files_in_dir());
    }

    #[test]
    fn pass_on_size_of_subdirs() {
        let mut dir = Dir::make_empty_dir("root");

        dir.add_file("eins", 44);
        dir.add_file("zwei", 34);
        dir.add_dir("sub");
        let subdir = dir.sub_dirs.get_mut("sub").unwrap();
        subdir.add_file("sub_eins", 5);
        subdir.add_file("sub_zwei", 7);
        subdir.add_dir("subsub");
        let subsubdir = subdir.sub_dirs.get_mut("subsub").unwrap();
        subsubdir.add_file("subsub_eins", 3);
        subsubdir.add_file("subsub_zwei", 8);

        let mut size_count: HashMap<String, u32> = HashMap::new();

        let total_size = dir.record_size_of_dir_including_subdirs(&mut size_count);
        assert_eq!(44 + 34 + 5 + 7 + 3 + 8, total_size);
    }
}
