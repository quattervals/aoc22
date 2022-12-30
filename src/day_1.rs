#![allow(dead_code)]

pub fn day_1_greeter() {
    println! {"Day 1"};

    read_elves_calories("./calories_example.txt");
}

fn read_elves_calories(file_name: &str) {
    let mut current_elve_count = 0;
    let mut calorie_count = 0;
    let mut elves_calories: Vec<(i32, i32)> = Vec::new();

    // File hosts must exist in current path before this produces output
    if let Ok(lines) = crate::utils::read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let () = match ip.parse::<i32>() {
                    Ok(ccl) => {
                        calorie_count += ccl;
                    }
                    Err(_) => {
                        current_elve_count += 1;
                        elves_calories.push((current_elve_count, calorie_count));
                        calorie_count = 0;
                    }
                };
            }
        }
    }

    elves_calories.sort_by(|a, b| b.1.cmp(&a.1));

    println!("Highest counting elves {:?}", &elves_calories[0..3]);
}
