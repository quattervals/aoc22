#![allow(dead_code)]


use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn runner() {
    println! {"21 Day 3"};

    read_file("./inputs/21_3_hot.txt");
}

fn read_file(file_name: &str) {
    let f: BufReader<File> = BufReader::new(File::open(file_name).unwrap());
    let num_lines: usize = f.lines().count();
    let mut signal: Vec<u32> = Vec::with_capacity(5 * num_lines);

    if let Ok(lines) = crate::utils::read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            signal.extend(
                line.unwrap()
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect::<Vec<u32>>(),
            );
        }
    }

    let (gamma, epsilon) = gamma_epsilon(&signal, num_lines, 12);

    println!("power is {}", gamma*epsilon);
}

fn gamma_epsilon(flat_signal: &Vec<u32>, rows: usize, cols: usize) -> (u32, u32) {
    let mut gamma_bin: Vec<u32> = Vec::new();
    let mut epsilon_bin: Vec<u32> = Vec::new();

    for i in 0..cols {
        let count = flat_signal
            .iter()
            .skip(i as usize)
            .step_by(cols as usize)
            .filter(|b| **b == 1)
            .sum::<u32>();

        if count > rows as u32 / 2 {
            gamma_bin.push(1);
            epsilon_bin.push(0);
        } else {
            gamma_bin.push(0);
            epsilon_bin.push(1);
        }
    }

    (
        gamma_bin
            .into_iter()
            .fold(0, |acc, digit| (acc << 1) + digit),
        epsilon_bin
            .into_iter()
            .fold(0, |acc, digit| (acc << 1) + digit),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slice_the_signal() {
        let flat_signal: Vec<u32> = vec![
            0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1,
            1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0,
            1, 0,
        ];
        assert_eq!(
            7,
            flat_signal
                .iter()
                .step_by(5)
                .filter(|b| **b == 1)
                .sum::<u32>()
        );

        assert_eq!(
            5,
            flat_signal
                .iter()
                .skip(1)
                .step_by(5)
                .filter(|b| **b == 1)
                .sum::<u32>()
        );
    }

    #[test]
    fn get_gamma_epsilon_as_decimal() {
        let flat_signal: Vec<u32> = vec![
            0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1,
            1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0,
            1, 0,
        ];

        assert_eq!((22, 9), gamma_epsilon(&flat_signal, 12, 5));
    }
}
