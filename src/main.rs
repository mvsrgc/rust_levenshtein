use std::cmp::{max, min};

fn main() {
    println!("Hello, world!");
}

pub fn levenshtein(left: &str, top: &str) -> Vec<Vec<isize>> {
    let left_of_table_nb_chars = left.len() + 1;
    let top_of_table_nb_chars = top.len() + 1;

    let mut matrix = vec![vec![0; top_of_table_nb_chars]; left_of_table_nb_chars];

    for i in 1..left.len() + 1 {
        matrix[i][0] = i as isize * -2;
    }

    for j in 1..top.len() + 1 {
        matrix[0][j] = j as isize * -2;
    }

    let mut sub_cost;
    for j in 1..top.len() + 1 {
        for i in 1..left.len() + 1 {
            if left.chars().nth(i - 1) == top.chars().nth(j - 1) {
                sub_cost = 1;
            } else {
                sub_cost = -1;
            }

            // max(a,b,c) = max(max(a,b), c)
            matrix[i][j] = max(
                max(matrix[i - 1][j] - 2, matrix[i][j - 1] - 2),
                matrix[i - 1][j - 1] + sub_cost,
            );
        }
    }

    matrix
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let left = "acgcttg";
        let top = "aggctg";
        let result = levenshtein(left, top);

        // Top row of labels
        print!("[ ][Ø]");
        for char in top.chars().into_iter() {
            print!("[{}]", char);
        }
        println!();

        // First row with Ø label
        print!("[Ø]");
        for i in 0..result[0].len() {
            print!("[{}]", result[0][i]);
        }

        println!();

        // Remaining lines
        for i in 1..result.len() {
            print!("[{}]", left.chars().nth(i - 1).unwrap());
            for j in 0..result[i].len() {
                print!("[{}]", result[i][j])
            }
            println!()
        }
    }
}
