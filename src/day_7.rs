use std::{collections::HashMap, path::PathBuf};

pub fn day_7_greeter() {
    println! {"Day 7"};

    looper("./inputs/7_hot.txt");
}

fn looper(file_name: &str) {
    let mut sizes: HashMap<PathBuf, usize> = HashMap::new();

    let mut stich: Vec<String> = Vec::new();

    if let Ok(lines) = crate::utils::read_lines(file_name) {
        for line in lines {
            if let Ok(items) = line {
                println!("{}", items);

                if items.starts_with("$ ls") || items.starts_with("dir") {
                    continue;
                }
                let splits: Vec<_> = items.split_whitespace().collect();

                match splits.as_slice() {
                    ["$", "cd", ".."] => {
                        stich.pop();
                    }

                    ["$", "cd", name] => {
                        stich.push(name.to_string());
                    }
                    [size, _name] => {
                        let size: usize = size.parse().unwrap();
                        for idx in 0..stich.len() {
                            let current_path = PathBuf::from_iter(&stich[..=idx]);
                            *sizes.entry(current_path).or_insert(0) += size;
                        }
                    }
                    _ => {}
                };
            }
        }
    }

    let sum: usize = sizes
        .iter()
        .map(|i| if *i.1 <= 100_000 { *i.1 } else { 0 })
        .sum();

    println!("sum {}", sum);

    const TOTAL_STORAGE: usize = 70_000_000;
    const STORAGE_FOR_UPDATE: usize = 30_000_000;
    let used_size = sizes.get(&PathBuf::from("/")).unwrap().clone();

    let minimal_size_to_delete = used_size + STORAGE_FOR_UPDATE - TOTAL_STORAGE;

    let candidate_dir_sizes: Vec<usize> = sizes
        .values()
        .cloned()
        .filter(|v| *v >= minimal_size_to_delete as usize)
        .collect();

    let min_dir_size = candidate_dir_sizes.iter().min().unwrap();
    println!("min dir size {}", min_dir_size);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample() {
        looper("./inputs/7_example.txt");
    }
}
