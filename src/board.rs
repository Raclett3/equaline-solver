pub type Pos = (isize, isize);

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cell {
    Number(i64),
    Operator(Operator),
}

impl Cell {
    fn number(self) -> Option<i64> {
        match self {
            Cell::Number(x) => Some(x),
            _ => None,
        }
    }

    fn operator(self) -> Option<Operator> {
        match self {
            Cell::Operator(op) => Some(op),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Board {
    board: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(board: Vec<Vec<Cell>>) -> Self {
        Board { board }
    }

    pub fn cell_at(&self, (x, y): Pos) -> Option<Cell> {
        Some(
            *self
                .board
                .get(usize::try_from(y).ok()?)?
                .get(usize::try_from(x).ok()?)?,
        )
    }

    pub fn mut_cell_at(&mut self, (x, y): Pos) -> Option<&mut Cell> {
        Some(
            self.board
                .get_mut(usize::try_from(y).ok()?)?
                .get_mut(usize::try_from(x).ok()?)?,
        )
    }

    pub fn calc(&self, path: &[Pos]) -> Option<i64> {
        if path.len() % 2 != 1 {
            return None;
        }

        let mut acc = self.cell_at(path[0])?.number()?;

        for (op, num) in path[1..]
            .iter()
            .cloned()
            .map(|pos| self.cell_at(pos))
            .scan(None, |last, x| {
                let item = last.map(|last| (last, x));
                if last.is_none() {
                    *last = Some(x)
                } else {
                    *last = None
                }
                Some(item)
            })
            .flatten()
        {
            let number = num?.number()?;

            match op?.operator()? {
                Operator::Add => {
                    acc += number;
                }
                Operator::Sub => {
                    acc -= number;
                }
                Operator::Mul => {
                    acc *= number;
                }
            }
        }

        Some(acc)
    }

    fn contains_path(&self, path: &[Pos]) -> bool {
        path.iter().cloned().all(|pos| self.cell_at(pos).is_some())
    }
}

pub fn solve(board: &Board, target: i64) -> Vec<Vec<Pos>> {
    fn dfs(board: &Board, target: i64, current_path: &mut Vec<Pos>, ans: &mut Vec<Vec<Pos>>) {
        static ADJACENT_POS: [Pos; 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

        if !board.contains_path(current_path) {
            return;
        }

        if board.calc(current_path) == Some(target) {
            ans.push(current_path.to_vec());
        }

        let (last_x, last_y) = *current_path.last().unwrap();

        for (dx, dy) in ADJACENT_POS {
            let next_pos = (last_x + dx, last_y + dy);
            if current_path.contains(&next_pos) {
                continue;
            }
            current_path.push(next_pos);
            dfs(board, target, current_path, ans);
            current_path.pop();
        }
    }

    let mut ans = Vec::new();

    for pos in board
        .board
        .iter()
        .enumerate()
        .flat_map(|(y, row)| (0..row.len()).map(move |x| (x as isize, y as isize)))
    {
        let source_cell = board.cell_at(pos).unwrap();

        if source_cell.number().is_none() {
            continue;
        }

        let mut current_path = vec![pos];

        dfs(board, target, &mut current_path, &mut ans);
    }

    ans
}

#[cfg(test)]
mod test {
    use super::Cell::*;
    use super::Operator::*;
    use super::{solve, Board};

    #[test]
    fn test_calc() {
        let board = Board::new(vec![
            vec![Number(1), Operator(Add), Number(3)],
            vec![Operator(Sub), Number(5), Operator(Add)],
            vec![Number(7), Operator(Mul), Number(9)],
        ]);

        assert_eq!(board.calc(&[]), None);
        assert_eq!(board.calc(&[(1, 0)]), None);
        assert_eq!(board.calc(&[(0, 0), (0, 1)]), None);
        assert_eq!(board.calc(&[(0, 0), (0, 1), (0, 2), (1, 2)]), None);
        assert_eq!(board.calc(&[(1, 1)]), Some(5));
        assert_eq!(board.calc(&[(1, 1), (1, 0), (0, 0)]), Some(6));
        assert_eq!(
            board.calc(&[(2, 2), (1, 2), (0, 2), (0, 1), (0, 0)]),
            Some(62)
        );
    }

    #[test]
    fn test_solve() {
        let board = Board::new(vec![
            vec![Number(1), Operator(Add), Number(3)],
            vec![Operator(Sub), Number(5), Operator(Add)],
            vec![Number(7), Operator(Mul), Number(9)],
        ]);

        assert_eq!(
            solve(&board, 81),
            vec![vec![(0, 0), (1, 0), (2, 0), (2, 1), (1, 1), (1, 2), (2, 2)]],
        );
    }
}
