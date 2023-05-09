#![allow(dead_code)]

use std::isize;

pub fn runner() {
    println! {"21 Day 2"};

    read_and_process_steering_commands("./inputs/21_2_hot.txt");
    read_and_process_aim_commands("./inputs/21_2_hot.txt");
}

fn read_and_process_steering_commands(file_name: &str) {
    let mut depth: isize = 0;
    let mut forward: isize = 0;

    if let Ok(lines) = crate::utils::read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let dings = line.unwrap();
            let parts = dings.split_whitespace().collect::<Vec<&str>>();

            match parts.as_slice() {
                ["forward", dist] => {
                    forward += dist.parse::<isize>().unwrap();
                }
                ["down", vert] => {
                    depth += vert.parse::<isize>().unwrap();
                }
                ["up", vert] => {
                    depth -= vert.parse::<isize>().unwrap();
                }
                _ => {}
            };
        }

        println!("forward {}, depth {}", forward, depth);
        println!("product {}", forward * depth);
    }
}

fn read_and_process_aim_commands(file_name: &str) {
    let mut aim: isize = 0;
    let mut forward: isize = 0;
    let mut depth: isize = 0;

    if let Ok(lines) = crate::utils::read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            let dings = line.unwrap();
            let parts = dings.split_whitespace().collect::<Vec<&str>>();

            match parts.as_slice() {
                ["forward", dist] => {
                    let increment = dist.parse::<isize>().unwrap();
                    forward += increment;
                    depth += aim * increment;
                }
                ["down", vert] => {
                    aim += vert.parse::<isize>().unwrap();
                }
                ["up", vert] => {
                    aim -= vert.parse::<isize>().unwrap();
                }
                _ => {}
            };
        }

        println!("forward {}, depth {}", forward, depth);
        println!("product {}", forward * depth);
    }
}
