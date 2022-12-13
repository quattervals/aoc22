#![allow(dead_code)]

#[derive(Debug, PartialEq)]
enum Results {
    IWin,
    ILose,
    Tie,
}

fn calc_points_of_round(win: Results, their_choice: char) -> i32 {
    let material_points = match their_choice {
        'X' => 1, // rock
        'Y' => 2, // paper
        'Z' => 3, // scissor
        _ => 0,
    };

    let win_points = match win {
        Results::IWin => 6,
        Results::ILose => 0,
        Results::Tie => 3,
    };

    material_points + win_points
}

fn calc_points_of_round_2(win: Results, material: char) -> i32 {
    let material_points = match material {
        'A' => 1, // rock
        'B' => 2, // paper
        'C' => 3, // scissor
        _ => 0,
    };

    let win_points = match win {
        Results::IWin => 6,
        Results::ILose => 0,
        Results::Tie => 3,
    };
    material_points + win_points
}

pub fn day_2_greeter() {
    println! {"Day 2"};
    rock_paper_scissor("./inputs/2_rps_hot.txt");
}

fn rock_paper_scissor(file_name: &str) {
    let mut my_points = 0;
    let mut points_according_to_real_plan = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = crate::utils::read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                if ip.chars().count() >= 3 {
                    my_points += calc_points_of_round(
                        i_win(ip.chars().nth(0).unwrap(), ip.chars().nth(2).unwrap()),
                        ip.chars().nth(2).unwrap(),
                    );

                    let (result, material) =
                        i_choose(ip.chars().nth(0).unwrap(), ip.chars().nth(2).unwrap());
                    points_according_to_real_plan += calc_points_of_round_2(result, material);
                } else {
                    println!("skip a line");
                }
            }
        }
    }

    println!("total points {}", my_points);
    println!(
        "total points according to real plan {}",
        points_according_to_real_plan
    );
}

fn i_win(theirs: char, mine: char) -> Results {
    if ((theirs == 'A') && (mine == 'Y'))
        || ((theirs == 'B') && (mine == 'Z'))
        || ((theirs == 'C') && (mine == 'X'))
    {
        return Results::IWin;
    } else if ((theirs == 'A') && (mine == 'Z'))
        || ((theirs == 'B') && (mine == 'X'))
        || ((theirs == 'C') && (mine == 'Y'))
    {
        return Results::ILose;
    } else {
        return Results::Tie;
    }
}

fn i_choose(theirs: char, mine: char) -> (Results, char) {
    let result = match mine {
        'X' => Results::ILose,
        'Y' => Results::Tie,
        'Z' => Results::IWin,
        _ => Results::Tie,
    };

    let lut: [[char; 3]; 3] = [['A', 'C', 'B'], ['B', 'A', 'C'], ['C', 'B', 'A']];

    let row_index = match &theirs {
        'A' => 0,
        'B' => 1,
        'C' => 2,
        _ => 0,
    };

    let col_index = match &result {
        Results::Tie => 0,
        Results::ILose => 1,
        Results::IWin => 2,
    };

    let i_play = lut[row_index][col_index];

    return (result, i_play);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winners() {
        assert_eq!(Results::Tie, i_win('A', 'X'));
        assert_eq!(Results::IWin, i_win('A', 'Y'));
        assert_eq!(Results::IWin, i_win('B', 'Z'));
        assert_eq!(Results::IWin, i_win('C', 'X'));
        assert_eq!(Results::ILose, i_win('A', 'Z'));
        assert_eq!(Results::ILose, i_win('C', 'Y'));
        assert_eq!(Results::Tie, i_win('C', 'Z'));
    }

    #[test]
    fn strategy_clarified() {
        assert_eq!((Results::ILose, 'C'), i_choose('A', 'X'));
        assert_eq!((Results::IWin, 'C'), i_choose('B', 'Z'));
    }
}
