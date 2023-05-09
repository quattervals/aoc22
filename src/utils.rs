#![allow(dead_code)]

use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

/// Reads a file and returns an Ok(String) with one line
///
/// Suggestion from [rust-by-example](https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html)
///
/// # Arguments
///
/// * `name` - A string slice with the file path to be loaded
///
/// # Examples
///
/// ```
/// if let Ok(lines) = crate::utils::read_lines(file_name) {
///     for line in lines {
///         if let Ok(items) = line {
///             println!("{}", items);
///         }
///     }
/// }
/// ```
pub fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

pub fn string_from_n_to_m(i_string: &str, i_from: usize, i_to: usize) -> String {
    String::from(&i_string[i_from..=i_to])
}

pub fn string_from_n_to_end(i_string: &str, i_from: usize) -> String {
    String::from(&i_string[i_from..])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn string_parsing() {
        assert_eq!(
            "eins",
            string_from_n_to_m("hahaeinszizi", 4, 7)
        );

        assert_eq!("meier", string_from_n_to_end("bliblameier", 6));
    }
}
