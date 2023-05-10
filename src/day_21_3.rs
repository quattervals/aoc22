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
    let max_col = 12;

    let mut signal: Vec<u32> = Vec::with_capacity(max_col * num_lines);

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
    let (gamma, epsilon) = gamma_epsilon(&signal, num_lines, max_col);
    println!("power is {}", gamma * epsilon);

    let signal_2d: Vec<Vec<u32>> = signal.chunks(max_col).map(|x| x.to_vec()).collect();
    let oxi_rating = find_rating(&signal_2d, max_col, false);
    let co2_rating = find_rating(&signal_2d, max_col, true);

    println!("life support rating is {}", oxi_rating * co2_rating);

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
        vec_of_bin_to_dec(&gamma_bin),
        vec_of_bin_to_dec(&epsilon_bin),
    )
}

fn vec_of_bin_to_dec(v: &Vec<u32>) -> u32 {
    v.iter().fold(0, |acc, digit| (acc << 1) + digit)
}

fn find_most_common_at_col(signal: &Vec<&Vec<u32>>, col: usize) -> u32 {
    let mut count_one: u32 = 0;
    for item in signal {
        if item[col] == 1 {
            count_one += 1;
        }
    }

    if count_one * 2 >= signal.len() as u32 {
        return 1;
    } else {
        return 0;
    }
}

fn matching_indices(signal: &Vec<&Vec<u32>>, col: usize, bit: u32) -> Vec<usize> {
    let mut indices: Vec<usize> = Vec::new();

    for (ix, item) in signal.iter().enumerate() {
        if item[col] == bit {
            indices.push(ix);
        }
    }
    indices
}

fn find_rating(signal: &Vec<Vec<u32>>, max_col: usize, least: bool) -> u32 {
    let mut reduced_signal: Vec<&Vec<u32>> = signal.iter().collect();

    for col in 0..max_col {
        let mut most_common = find_most_common_at_col(&reduced_signal, col) != 0;

        if least {
            most_common = !most_common;
        }

        let indices: Vec<usize> = matching_indices(&reduced_signal, col, most_common as u32);

        reduced_signal = reduced_signal
            .into_iter()
            .enumerate()
            .filter(|&(ix, _)| indices.contains(&ix))
            .map(|(_, item)| item)
            .collect();

        if reduced_signal.len() == 1 {
            break;
        }
    }

    vec_of_bin_to_dec(reduced_signal[0])

}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_rating() {
        let flat_signal: Vec<u32> = vec![
            0, 0, 1, 0, 0, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 1, 0, 1, 1, 1, 1, 0, 1, 0, 1, 0, 1, 1, 1,
            1, 0, 0, 1, 1, 1, 1, 1, 1, 0, 0, 1, 0, 0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0,
            1, 0,
        ];

        let signal: Vec<Vec<u32>> = flat_signal.chunks(5).map(|x| x.to_vec()).collect();

        let max_col = 5;

        assert_eq!(23, find_rating(&signal, max_col, false));
        assert_eq!(10, find_rating(&signal, max_col, true));
    }

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
