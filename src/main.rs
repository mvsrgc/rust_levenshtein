use std::{cmp::min, str::Chars};

fn main() {
    println!("Hello, world!");
}

pub fn levenshtein(left: &str, top: &str) -> Vec<Vec<usize>> {
    let left_of_table_nb_chars = left.len() + 1;
    let top_of_table_nb_chars = top.len() + 1;

    let matrix = vec![vec![0; top_of_table_nb_chars]; left_of_table_nb_chars];

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let left = "accgtcg";
        let top = "acgccg";
        let result = levenshtein(left, top);

        print!("[ ][⌀]");
        for char in top.chars().into_iter() {
            print!("[{}]", char);
        }
        println!();
        for i in 0..result.len() {
            if i == 0 {
                print!("[⌀]");
            } else if i != 0 {
                print!("[{}]", left.chars().nth(i - 1).unwrap());
            }
            for j in 0..result[i].len() {
                print!("[{}]", result[i][j])
            }
            println!()
        }
    }
}
