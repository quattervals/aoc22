pub fn day_6_greeter() {
    println! {"Day 6"};

    looper("./inputs/6_code_hot.txt");
}

fn looper(file_name: &str) {
    if let Ok(lines) = crate::utils::read_lines(file_name) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(items) = line {
                let signal: Vec<char> = items.chars().collect();

                let marker_len = 14;

                for n in 0..signal.len() - marker_len {
                    let marker_hypothesis = &signal[n..n + marker_len];
                    if !(1..marker_hypothesis.len())
                        .any(|i| marker_hypothesis[i..].contains(&marker_hypothesis[i - 1]))
                    {

                        println!("end marker is {}", n + marker_len);
                        break;
                    }
                }
            }
        }
    }
}

