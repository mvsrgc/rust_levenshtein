use std::cmp::{max, min};

fn main() {
    println!("Hello, world!");
}

pub fn levenshtein(
    left: &str,
    top: &str,
    insert_cost: isize,
    del_cost: isize,
    _sub_cost: isize,
    match_cost: isize,
    use_confusion_matrix: bool,
) -> Vec<Vec<isize>> {
    let left_of_table_nb_chars = left.len() + 1;
    let top_of_table_nb_chars = top.len() + 1;

    let mut matrix = vec![vec![0; top_of_table_nb_chars]; left_of_table_nb_chars];

    for i in 1..left.len() + 1 {
        matrix[i][0] = i as isize * del_cost;
    }

    for j in 1..top.len() + 1 {
        matrix[0][j] = j as isize * insert_cost;
    }

    let mut sub_cost;
    for j in 1..top.len() + 1 {
        for i in 1..left.len() + 1 {
            if left.chars().nth(i - 1) == top.chars().nth(j - 1) {
                sub_cost = match_cost;
            } else {
                if use_confusion_matrix {
                    let letters = ['a', 'c', 'g', 't'];
                    let confusion_matrix: Vec<Vec<isize>> = vec![
                        vec![1, -1, -2, -1],
                        vec![-1, 1, -3, -1],
                        vec![-2, -3, 1, -2],
                        vec![-1, -1, -2, 1],
                    ];

                    let left_letter_index = letters
                        .iter()
                        .position(|&x| x == left.chars().nth(i - 1).unwrap())
                        .unwrap();

                    let top_letter_index = letters
                        .iter()
                        .position(|&x| x == top.chars().nth(j - 1).unwrap())
                        .unwrap();

                    sub_cost =
                        confusion_matrix[left_letter_index as usize][top_letter_index as usize];
                } else {
                    sub_cost = _sub_cost;
                }
            }

            // max(a,b,c) = max(max(a,b), c)
            matrix[i][j] = max(
                max(matrix[i - 1][j] + insert_cost, matrix[i][j - 1] + del_cost),
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
    fn test_levenshtein() {
        let left = "accgtcg";
        let top = "acgccg";

        let result = levenshtein(left, top, -4, -4, -1, 1, true);

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
