#[derive(Debug, PartialEq)]
enum Results {
    I_WIN,
    I_LOSE,
    TIE,
}

fn calc_points_of_round(win: Results, my_choice: char) -> i32 {
    let material_points = match my_choice {
        'X' => 1,
        'Y' => 2,
        'Z' => 3,
        _ => 0,
    };

    let win_points = match win {
        Results::I_WIN => 6,
        Results::I_LOSE => 0,
        Results::TIE => 3,
        _ => 0,
    };

    material_points + win_points
}

pub fn day_2_greeter() {
    println! {"Day 2"};
    rock_paper_scissor("./inputs/2_rps_hot.txt");
}

fn rock_paper_scissor(file_name: &str) {
    let mut my_points = 0;
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
                } else {
                    println!("skip a line");
                }
            }
        }
    }

    println!("total points {}", my_points);
}

fn i_win(theirs: char, mine: char) -> Results {
    if ((theirs == 'A') && (mine == 'Y'))
        || ((theirs == 'B') && (mine == 'Z'))
        || ((theirs == 'C') && (mine == 'X'))
    {
        return Results::I_WIN;
    } else if ((theirs == 'A') && (mine == 'Z'))
        || ((theirs == 'B') && (mine == 'X'))
        || ((theirs == 'C') && (mine == 'Y'))
    {
        return Results::I_LOSE;
    } else {
        return Results::TIE;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn winners() {
        assert_eq!(Results::TIE, i_win('A', 'X'));
        assert_eq!(Results::I_WIN, i_win('A', 'Y'));
        assert_eq!(Results::I_WIN, i_win('B', 'Z'));
        assert_eq!(Results::I_WIN, i_win('C', 'X'));
        assert_eq!(Results::I_LOSE, i_win('A', 'Z'));
        assert_eq!(Results::I_LOSE, i_win('C', 'Y'));
        assert_eq!(Results::TIE, i_win('C', 'Z'));
    }
}
