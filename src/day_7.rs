use std::{collections::HashMap, hash::Hash};

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

    fn size_of_subdirs(&self) -> u32 {
        let mut size_of_subdirs: u32 = 0;
        for (key, subdir) in &self.sub_dirs {
            size_of_subdirs += subdir.size_of_subdirs();
        }

        return self.size_of_files_in_dir() + size_of_subdirs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let current_dir = match dir.sub_dirs.get_mut("new_dir") {
            Some(x) => x,
            None => todo!(),
        };
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
    fn calc_size_of_subdirs() {
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

        assert_eq!(44 + 34 + 5 + 7 + 3 + 8, dir.size_of_subdirs());
    }
}
