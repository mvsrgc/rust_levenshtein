use std::cmp::max;

fn main() {
    println!("Hello, world!");
}

pub fn levenshtein(left: &str, top: &str) -> Vec<Vec<usize>> {
    let left_of_table_nb_chars = left.len() + 1;
    let top_of_table_nb_chars = top.len() + 1;

    let mut matrix = vec![vec![0; top_of_table_nb_chars]; left_of_table_nb_chars];

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            matrix[i][j] = 1;
        }
    }

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

        print!("[ ][Ø]");
        for char in top.chars().into_iter() {
            print!("[{}]", char);
        }
        println!();

        print!("[Ø]");
        for i in 0..result[0].len() {
            print!("[{}]", result[0][i]);
        }

        println!();

        for i in 1..result.len() {
            print!("[{}]", left.chars().nth(i - 1).unwrap());
            for j in 0..result[i].len() {
                print!("[{}]", result[i - 1][j])
            }
            println!()
        }
    }
}
