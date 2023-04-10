extern crate kissat;

// The number of bits we need to represent a number.
const BITS: usize = 4;
const ROWS: usize = 9;
const COLUMNS: usize = 9;

type Board = [[[kissat::Var; BITS]; COLUMNS]; ROWS];

fn print_solution(solution: &kissat::Solution, board: &Board) {
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let mut number = 0;
            for bit in 0..BITS {
                if solution.get(board[row][column][bit]) == Some(true) {
                    number += 1 << (BITS - bit - 1);
                }
            }
            print!("{} ", number + 1);
        }
        println!("");
    }
}

// Assigns a number to a cell.
// `number` must be between 0 and 8 (both inclusive).
fn assign(solver: &mut kissat::Solver, board: &Board, row: usize, column: usize, number: i32) {
    assert!(number >= 0);
    assert!(number <= 8);
    for bit in 0..BITS {
        if number & (1 << (BITS - bit - 1)) != 0 {
            solver.add1(board[row][column][bit]);
        } else {
            solver.add1(!board[row][column][bit]);
        }
    }
}

// Returns a variable that is true iff the two cells are not equal.
fn neq(
    solver: &mut kissat::Solver,
    board: &Board,
    row_a: usize,
    column_a: usize,
    row_b: usize,
    column_b: usize,
) -> kissat::Var {
    let mut neq = solver.xor(board[row_a][column_a][0], board[row_b][column_b][0]);
    for bit in 1..BITS {
        let neq_ = solver.xor(board[row_a][column_a][bit], board[row_b][column_b][bit]);
        neq = solver.or(neq, neq_);
    }
    neq
}

fn main() {
    let mut solver = kissat::Solver::new();

    let mut board = [[[solver.var(); BITS]; COLUMNS]; ROWS];
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            for bit in 0..BITS {
                board[row][column][bit] = solver.var();
            }
        }
    }

    // Each cell must have a number between 0 and 8 (both inclusive).
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            let mut is_eight = board[row][column][0];
            for bit in 1..BITS {
                is_eight = solver.and(is_eight, !board[row][column][bit]);
            }
            solver.add2(!board[row][column][0], is_eight);
        }
    }

    // Each cell must be unique in the row it belongs to.
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            for other_column in (column + 1)..COLUMNS {
                let ne = neq(&mut solver, &board, row, column, row, other_column);
                solver.add1(ne);
            }
        }
    }
    // Each cell must be unique in the column it belongs to.
    for row in 0..ROWS {
        for column in 0..COLUMNS {
            for other_row in (row + 1)..ROWS {
                let ne = neq(&mut solver, &board, row, column, other_row, column);
                solver.add1(ne);
            }
        }
    }

    // Each cell must be unique in the 3x3 sub-grid it belongs to.
    for row_base in 0..ROWS {
        if row_base % 3 != 0 {
            continue;
        }
        for column_base in 0..COLUMNS {
            if column_base % 3 != 0 {
                continue;
            }
            for row_a in row_base..(row_base + 3) {
                for row_b in row_base..(row_base + 3) {
                    for column_a in column_base..(column_base + 3) {
                        for column_b in column_base..(column_base + 3) {
                            // When (`row_a`, `column_a`) equals to (`row_b`, `column_b`),
                            // then we shouldn't check the inequality (it always fails).
                            // When (`row_a`, `column_a`) > (`row_b`, `column_b`), then
                            // we don't need to check the inequality (it's already been checked).
                            if row_a > row_b || (row_a == row_b && column_a >= column_b) {
                                continue;
                            }
                            let ne = neq(&mut solver, &board, row_a, column_a, row_b, column_b);
                            solver.add1(ne);
                        }
                    }
                }
            }
        }
    }

    assign(&mut solver, &board, 0, 0, 0);
    assign(&mut solver, &board, 0, 1, 3);

    // Print the cells if there is a solution.
    match solver.sat() {
        Some(solution) => print_solution(&solution, &board),
        None => println!("No solutions are found"),
    }
}
