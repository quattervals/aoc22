use std::collections::HashMap;
use std::ops;

pub fn runner() {
    println! {"Day 9"};

    looper("./inputs/9_hot.txt");
}

fn looper(file_name: &str) -> u32 {
    let mut head = Pos(0, 0);
    let mut tail = Pos(0, 0);
    let mut visited_pos: HashMap<Pos, usize> = HashMap::new();

    let mut tails = vec![Pos(0, 0); 10];
    let mut visited_long_pos: HashMap<Pos, usize> = HashMap::new();

    if let Ok(lines) = crate::utils::read_lines(file_name) {
        for line in lines {
            if let Ok(line) = line {
                println!("line {}", line);

                let splits: Vec<_> = line.split_whitespace().collect();

                move_head_and_tail(
                    &mut head,
                    &mut tail,
                    &mut visited_pos,
                    splits[0],
                    splits[1].parse().unwrap(),
                );

                move_head_and_long_tail(
                    &mut tails,
                    &mut visited_long_pos,
                    splits[0],
                    splits[1].parse().unwrap(),
                );
            }
        }
        println!(
            "number of short visited positions {}",
            visited_pos.iter().count()
        );
        println!(
            "number of long visited positions {}",
            visited_long_pos.iter().count()
        );
    }

    return 1;
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct Pos(i32, i32);

impl ops::Add<Pos> for Pos {
    type Output = Self;

    fn add(self, other: Pos) -> Pos {
        Pos(self.0 + other.0, self.1 + other.1)
    }
}
impl ops::Sub<Pos> for Pos {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1)
    }
}

impl Pos {
    fn dist_to(&self, other: &Self) -> Self {
        Self(other.0 - self.0, other.1 - self.1)
    }
}

fn travel_tail(distance: &Pos) -> Pos {
    let mut ret = Pos(0, 0);

    let is_diagonal = (distance.0.abs() > 0 && distance.1.abs() > 1)
        || (distance.0.abs() > 1 && distance.1.abs() > 0);

    let thresh = if is_diagonal { 0 } else { 1 };

    if distance.0.abs() > thresh {
        ret.0 = distance.0.signum();
    }
    if distance.1.abs() > thresh {
        ret.1 = distance.1.signum();
    }
    ret
}

fn move_head_and_long_tail(
    tails: &mut Vec<Pos>,
    visited_pos: &mut HashMap<Pos, usize>,
    direction: &str,
    steps: u32,
) {
    let dir = match direction {
        "R" => Pos(1, 0),
        "U" => Pos(0, 1),
        "L" => Pos(-1, 0),
        "D" => Pos(0, -1),
        _ => Pos(0, 0),
    };

    for _ in 0..steps {
        tails[0] = tails[0] + dir;

        for i in 1..tails.len() {
            let dist_to_previous = tails[i].dist_to(&tails[i - 1]);

            let tail_travels = travel_tail(&dist_to_previous);
            tails[i] = tails[i] + tail_travels;
        }

        visited_pos.insert(tails[tails.len() - 1], 1);
    }
}

fn move_head_and_tail(
    head: &mut Pos,
    tail: &mut Pos,
    visited_pos: &mut HashMap<Pos, usize>,
    direction: &str,
    steps: u32,
) {
    let dir = match direction {
        "R" => Pos(1, 0),
        "U" => Pos(0, 1),
        "L" => Pos(-1, 0),
        "D" => Pos(0, -1),
        _ => Pos(0, 0),
    };

    for _ in 0..steps {
        *head = *head + dir;
        let dist_to_moved_head = tail.dist_to(head);

        let tail_travels = travel_tail(&dist_to_moved_head);
        *tail = *tail + tail_travels;

        visited_pos.insert(*tail, 1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn operations() {
        assert_eq!(Pos(1, 1), Pos(2, 3) - Pos(1, 2));
        assert_eq!(Pos(-5, -3), Pos(2, 4).dist_to(&Pos(-3, 1)))
    }

    #[test]
    fn distances() {
        let rel_pos = Pos(1, -1);
        let head_translation = Pos(1, 0);
        let new_dist = rel_pos.dist_to(&head_translation);
        assert_eq!(Pos(0, 0), travel_tail(&new_dist));
        assert_eq!(Pos(0, 0), travel_tail(&Pos(-1, 1).dist_to(&Pos(-1, 1))));
        assert_eq!(Pos(0, 0), travel_tail(&Pos(1, 0).dist_to(&Pos(1, 0))));

        println!("test {:?}", travel_tail(&Pos(1, -1).dist_to(&Pos(0, 1))));

        assert_eq!(Pos(-1, 1), travel_tail(&Pos(1, -1).dist_to(&Pos(0, 1))));
    }

    #[test]
    fn tail_follows() {
        let mut head = Pos(0, 0);
        let mut tail = Pos(0, 0);
        let mut visited_pos: HashMap<Pos, usize> = HashMap::new();

        move_head_and_tail(&mut head, &mut tail, &mut visited_pos, "R", 4);
        assert_eq!(Pos(3, 0), tail);
        assert_eq!(Pos(4, 0), head);
        move_head_and_tail(&mut head, &mut tail, &mut visited_pos, "U", 4);
        assert_eq!(Pos(4, 3), tail);
        assert_eq!(Pos(4, 4), head);
        move_head_and_tail(&mut head, &mut tail, &mut visited_pos, "L", 3);
        assert_eq!(Pos(2, 4), tail);
        assert_eq!(Pos(1, 4), head);
        move_head_and_tail(&mut head, &mut tail, &mut visited_pos, "D", 1);
        assert_eq!(Pos(2, 4), tail);
        assert_eq!(Pos(1, 3), head);
        move_head_and_tail(&mut head, &mut tail, &mut visited_pos, "R", 4);
        assert_eq!(Pos(4, 3), tail);
        assert_eq!(Pos(5, 3), head);
        move_head_and_tail(&mut head, &mut tail, &mut visited_pos, "D", 1);
        assert_eq!(Pos(4, 3), tail);
        assert_eq!(Pos(5, 2), head);
    }

    #[test]
    fn work_with_map() {
        let mut visited_pos: HashMap<Pos, usize> = HashMap::new();

        visited_pos.insert(Pos(1, 2), 1);
        visited_pos.insert(Pos(2, 2), 1);
        visited_pos.insert(Pos(1, 2), 9);

        assert_eq!(2, visited_pos.iter().count());
    }
    #[test]
    fn long_tail_follows() {
        let mut tails = vec![Pos(0, 0); 10];
        let mut visited_pos: HashMap<Pos, usize> = HashMap::new();

        move_head_and_long_tail(&mut tails, &mut visited_pos, "R", 5);
        assert_eq!(Pos(5, 0), tails[0]);
        assert_eq!(Pos(4, 0), tails[1]);
        assert_eq!(Pos(1, 0), tails[4]);
        assert_eq!(Pos(0, 0), tails[5]);
        assert_eq!(Pos(0, 0), tails[9]);
        assert_eq!(5, visited_pos.iter().count());
    }
}
