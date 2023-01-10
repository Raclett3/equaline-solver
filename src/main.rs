use equaline::board::{solve, Board, Cell, Operator::*};

fn main() {
    let stdin = std::io::stdin();
    let mut target = String::new();
    stdin.read_line(&mut target).unwrap();
    let target = target.trim().parse().expect("Number");

    let mut cells = Vec::new();
    for line in stdin.lines().flatten() {
        cells.push(
            line.split_whitespace()
                .flat_map(|x| {
                    let cell = match x {
                        "+" => Cell::Operator(Add),
                        "-" => Cell::Operator(Sub),
                        "*" => Cell::Operator(Mul),
                        x => Cell::Number(x.parse().ok()?),
                    };
                    Some(cell)
                })
                .collect::<Vec<_>>(),
        );
    }

    let board = Board::new(cells);
    let mut solutions = solve(&board, target);
    if solutions.is_empty() {
        println!("No solution");
    } else if solutions.len() == 1 {
        println!("Solution: {:?}", solutions[0]);
    } else {
        solutions.sort_by_key(|x| x.len());
        let shortest = solutions.first().unwrap();
        println!("Shortest solution: {shortest:?} ({})", shortest.len());
        let longest = solutions.last().unwrap();
        println!("Longest solution: {longest:?} ({})", longest.len());
    }
}
