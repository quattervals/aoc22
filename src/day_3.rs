pub fn day_3_greeter() {
    println! {"Day 3"};

    dinger("./inputs/3_items_hot.txt");
}

fn dinger(file_name: &str) {
    let mut prio_count = 0;
    if let Ok(lines) = crate::utils::read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(items) = line {
                let all_items: Vec<char> = items.chars().collect();
                let (first, second) = collection_split(&all_items);

                let common_item: char = match find_common_item(first, second) {
                    None => '-',
                    Some(x) => x,
                };
                prio_count += item_value(common_item);
            }
        }

        println!("total prio count {}", prio_count);
    }
}

fn collection_split(all_items: &[char]) -> (&[char], &[char]) {
    let total_item_count = all_items.len();
    let items_first_compartment = &all_items[0..total_item_count / 2];
    let items_second_compartment = &all_items[total_item_count / 2..total_item_count];

    return (items_first_compartment, items_second_compartment);
}

fn find_common_item(first_compartment: &[char], second_compartment: &[char]) -> Option<char> {
    for item_in_first in first_compartment {
        if second_compartment.contains(item_in_first) {
            return Some(item_in_first.clone());
        }
    }
    return None;
}

fn item_value(item: char) -> u32 {
    if item.is_ascii_uppercase() {
        return (item as u32) - 65 + 27;
    }

    if item.is_ascii_lowercase() {
        return (item as u32) + 1 - 97;
    }
    return 0;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn finding_items() {
        let all_items = vec!['a', 'B', 'x', 'a'];
        let (f, s) = collection_split(&all_items);
        assert_eq!('a', find_common_item(f, s).unwrap());

        let all_items = vec!['a', 'B', 'x', 'M'];
        let (f, s) = collection_split(&all_items);
        assert_eq!(None, find_common_item(f, s));
    }

    #[test]
    fn value_of_digits() {
        assert_eq!(27, item_value('A'));
        assert_eq!(1, item_value('a'));
    }

    #[test]
    fn move_parts_of_vector_to_other_vector() {
        let mut v_from = vec!["a", "b", "c"];
        let mut v_to = vec!["x", "y", "z"];

        v_to.append(&mut v_from.drain(1..).collect());

        println!("now v_from {:?}", v_from);
        println!("now v_to {:?}", v_to);
    }
}
