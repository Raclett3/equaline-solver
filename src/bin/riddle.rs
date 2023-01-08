use equaline::board::*;
use Operator::*;

fn increment(cell: Cell) -> Cell {
    match cell {
        Cell::Number(x) => Cell::Number(x + 1),
        Cell::Operator(Add) => Cell::Operator(Sub),
        Cell::Operator(Sub) => Cell::Operator(Mul),
        Cell::Operator(Mul) => Cell::Operator(Add),
    }
}

fn decrement(cell: Cell) -> Cell {
    match cell {
        Cell::Number(x) => Cell::Number(x - 1),
        Cell::Operator(Add) => Cell::Operator(Mul),
        Cell::Operator(Sub) => Cell::Operator(Add),
        Cell::Operator(Mul) => Cell::Operator(Sub),
    }
}

fn main() {
    fn dfs(board: &mut Board, problem_number: i64) -> Option<Vec<Vec<Pos>>> {
        if problem_number > 10 {
            return Some(Vec::new());
        }

        let target = (problem_number + 1) * problem_number / 2;

        for solution in solve(board, target) {
            for pos in solution.iter().copied() {
                *board.mut_cell_at(pos)? = increment(board.cell_at(pos)?);
            }

            let solutions_deeper = dfs(board, problem_number + 1);

            for pos in solution.iter().copied() {
                *board.mut_cell_at(pos)? = decrement(board.cell_at(pos)?);
            }

            if let Some(mut solutions_deeper) = solutions_deeper {
                solutions_deeper.insert(0, solution);
                return Some(solutions_deeper);
            }
        }

        None
    }

    const ONE: Cell = Cell::Number(1);
    const ADD: Cell = Cell::Operator(Add);
    let mut board = Board::new(vec![
        vec![ONE, ADD, ONE],
        vec![ADD, ONE, ADD],
        vec![ONE, ADD, ONE],
    ]);

    if let Some(solutions) = dfs(&mut board, 1) {
        for (i, solution) in solutions.into_iter().enumerate() {
            println!("{i}: {solution:?}");
        }
    }
}
