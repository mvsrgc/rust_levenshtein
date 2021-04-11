use std::cmp::{max, min};
#[macro_use]
extern crate prettytable;
use prettytable::{Cell, Row, Table};

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
    confusion_matrix_chars: Vec<char>,
    confusion_matrix: Vec<Vec<isize>>,
) -> Vec<Vec<isize>> {
    let left_of_table_nb_chars = left.len() + 1;
    let top_of_table_nb_chars = top.len() + 1;

    let mut matrix = vec![vec![0; top_of_table_nb_chars]; left_of_table_nb_chars];

    // Populate first column.
    for i in 1..left.len() + 1 {
        matrix[i][0] = i as isize * del_cost;
    }

    // Populate first row.
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
                    let left_letter_index = confusion_matrix_chars
                        .iter()
                        .position(|&x| x == left.chars().nth(i - 1).unwrap())
                        .unwrap();

                    let top_letter_index = confusion_matrix_chars
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
        let left = "acgcttg";
        let top = "aggctg";
        let confusion_matrix_chars = vec!['a', 'c', 'g', 't'];
        let confusion_matrix: Vec<Vec<isize>> = vec![
            vec![1, -1, -2, -1],
            vec![-1, 1, -3, -1],
            vec![-2, -3, 1, -2],
            vec![-1, -1, -2, 1],
        ];

        let result = levenshtein(
            left,
            top,
            -2,
            -2,
            -1,
            1,
            false,
            confusion_matrix_chars,
            confusion_matrix,
        );

        let mut table = Table::new();

        let title_row = Row::new(
            vec![" ", "Ø"]
                .into_iter()
                .chain(top.split_inclusive(|_| true))
                .map(|s| Cell::new(s))
                .collect(),
        );

        let mut first_row = Row::new(vec![Cell::new("Ø")]);
        for i in 0..result[0].len() {
            first_row.add_cell(Cell::new(&result[0][i].to_string()));
        }

        table.set_titles(title_row);
        table.add_row(first_row);

        for i in 1..result.len() {
            let mut row = Row::new(vec![]);
            let cell = Cell::new(&left.chars().nth(i - 1).unwrap().to_string());
            row.add_cell(cell);
            for j in 0..result[i].len() {
                let cell = Cell::new(&result[i][j].to_string());
                row.add_cell(cell);
            }
            table.add_row(row);
        }

        table.printstd();
    }
}
