use ndarray::prelude::*;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn runner() {
    println! {"Day 8"};

    looper("./inputs/8_hot.txt");
}

fn looper(file_name: &str) -> u32 {
    let f = BufReader::new(File::open(file_name).unwrap());
    let dim = f.lines().count();
    let mut flat_forrest: Vec<u32> = Vec::with_capacity(dim * dim);

    if let Ok(lines) = crate::utils::read_lines(file_name) {
        for line in lines {
            if let Ok(line) = line {
                flat_forrest.extend(
                    line.chars()
                        .map(|c| c.to_digit(10).unwrap())
                        .collect::<Vec<u32>>(),
                );
            }
        }
    }

    let forrest = Array::from_shape_vec((dim, dim), flat_forrest).unwrap();
    println!(
        "Number of visible trees {}",
        count_visible_trees_in_forrest(&forrest)
    );

    println!(
        "Max Scoring Tree has score {}",
        highest_tree_view_score(&forrest)
    );
    return highest_tree_view_score(&forrest);
}

fn count_visible_trees_in_forrest(
    forrest: &ArrayBase<ndarray::OwnedRepr<u32>, Dim<[usize; 2]>>,
) -> u32 {
    let dim = forrest.dim().0;
    let mut count_visible_trees = 0;
    for row in 1..dim - 1 {
        for col in 1..dim - 1 {
            let max = &forrest[(row, col)];

            let max_from_north = forrest.slice(s![..row, col]).iter().max().unwrap().clone();
            let max_from_west = forrest.slice(s![row, ..col]).iter().max().unwrap().clone();
            let max_from_east = forrest
                .slice(s![row, col + 1..])
                .iter()
                .max()
                .unwrap()
                .clone();
            let max_from_south = forrest
                .slice(s![row + 1.., col])
                .iter()
                .max()
                .unwrap()
                .clone();

            if max_from_north < *max
                || max_from_west < *max
                || max_from_east < *max
                || max_from_south < *max
            {
                count_visible_trees += 1;
            }
        }
    }

    count_visible_trees += 4 * dim as u32 - 4;
    return count_visible_trees;
}

fn highest_tree_view_score(forrest: &ArrayBase<ndarray::OwnedRepr<u32>, Dim<[usize; 2]>>) -> u32 {
    let dim = forrest.dim().0;
    let mut view_score: Vec<u32> = Vec::new();

    for row in 1..dim - 1 {
        for col in 1..dim - 1 {
            let max = &forrest[(row, col)];

            let mut north_count = 0;
            for i in forrest.slice(s![..row, col]).iter().rev() {
                north_count += 1;
                if i >= max {
                    break;
                }
            }

            let mut west_count = 0;
            for i in forrest.slice(s![row, ..col]).iter().rev() {
                west_count += 1;
                if i >= max {
                    break;
                }
            }

            let mut east_count = 0;
            for i in forrest.slice(s![row, col + 1..]).iter() {
                east_count += 1;
                if i >= max {
                    break;
                }
            }

            let mut south_count = 0;
            for i in forrest.slice(s![row + 1.., col]).iter() {
                south_count += 1;
                if i >= max {
                    break;
                }
            }
            let count_of_tree = north_count * west_count * east_count * south_count;

            view_score.push(count_of_tree as u32);
        }
    }

    return *view_score.iter().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn work_with_ndarray() {
        let vek: Vec<u32> = vec![0, 1, 2, 3, 4, 5, 6, 7, 8];
        let a = Array::from_shape_vec((3, 3), vek).unwrap();
        assert_eq!(array![1, 4], a.slice(s![0..2, 1]));
    }

    #[test]
    fn slice_the_forrest() {
        let dim = 5;
        let flat_forrest: Vec<u32> = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];

        let forrest = Array::from_shape_vec((dim, dim), flat_forrest).unwrap();

        let row = 2;
        let col = 3;
        /*
        30373
        25512
        65332
        33549
        35390
        */

        //north_to_pos
        assert_eq!(array![7, 1], forrest.slice(s![..row, col]));
        assert_eq!(
            false,
            forrest.slice(s![..row, col]).iter().max().unwrap() < &forrest[(row, col)]
        );

        //west_to_pos
        assert_eq!(array![6, 5, 3], forrest.slice(s![row, ..col]));
        assert_eq!(
            false,
            forrest.slice(s![row, ..col]).iter().max().unwrap() < &forrest[(row, col)]
        );

        //pos_to_east
        assert_eq!(array![2], forrest.slice(s![row, col + 1..]));
        assert_eq!(
            true,
            forrest.slice(s![row, col + 1..]).iter().max().unwrap() < &forrest[(row, col)]
        );

        //pos_to_south
        assert_eq!(array![4, 9], forrest.slice(s![row + 1.., col]));
        assert_eq!(
            false,
            forrest.slice(s![row + 1.., col]).iter().max().unwrap() < &forrest[(row, col)]
        );
    }

    #[test]
    fn number_of_trees_fn() {
        let dim = 5;
        let flat_forrest: Vec<u32> = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];

        let forrest = Array::from_shape_vec((dim, dim), flat_forrest).unwrap();
        assert_eq!(21, count_visible_trees_in_forrest(&forrest));
    }

    #[test]
    fn tree_view_score_test() {
        let dim = 5;
        let flat_forrest: Vec<u32> = vec![
            3, 0, 3, 7, 3, 2, 5, 5, 1, 2, 6, 5, 3, 3, 2, 3, 3, 5, 4, 9, 3, 5, 3, 9, 0,
        ];

        let forrest = Array::from_shape_vec((dim, dim), flat_forrest).unwrap();

        highest_tree_view_score(&forrest);
        assert_eq!(8, highest_tree_view_score(&forrest));
    }

    #[test]
    fn live_test() {
        assert_eq!(444528, looper("./inputs/8_hot.txt"));
    }
}
