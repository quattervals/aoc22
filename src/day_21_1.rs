#![allow(dead_code)]

pub fn runner() {
    println! {"21 Day 1"};

    read_depth_meter("./inputs/21_1_hot.txt");
}

fn read_depth_meter(file_name: &str) {
    let mut depth_readings: Vec<usize> = Vec::new();

    if let Ok(lines) = crate::utils::read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            depth_readings.push(line.unwrap().parse::<usize>().unwrap());
        }
    }

    let depth_increases = count_depth_reading_increases(&depth_readings);

    println!("[1]: The number of depth increases is {depth_increases}");

    let filtered_depth_increases = count_filtered_depth_reading_increases(&depth_readings);

    println!("[2]: The number of filtered depth increases is {filtered_depth_increases}");
}

fn count_depth_reading_increases(depth_readings: &Vec<usize>) -> usize {
    depth_readings.windows(2).filter(|w| w[0] < w[1]).count()
}

fn count_filtered_depth_reading_increases(depth_readings: &Vec<usize>) -> usize {
    depth_readings
        .windows(4)
        .filter(|w| w[..3].iter().sum::<usize>() < w[1..].iter().sum())
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_depth_readings() {
        let depth_readings: Vec<usize> = vec![1, 2];
        assert_eq!(1, count_depth_reading_increases(&depth_readings));

        let depth_readings: Vec<usize> = vec![1, 2, 2, 3, 1, 6];
        assert_eq!(3, count_depth_reading_increases(&depth_readings));

        let depth_readings: Vec<usize> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(7, count_depth_reading_increases(&depth_readings));
    }
    #[test]
    fn test_filtered_depth_readings() {
        let depth_readings: Vec<usize> = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];
        assert_eq!(5, count_filtered_depth_reading_increases(&depth_readings));
    }
}
