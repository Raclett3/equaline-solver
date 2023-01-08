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

pub struct Board {
    board: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(board: Vec<Vec<Cell>>) -> Self {
        Board { board }
    }

    fn cell_at(&self, (x, y): (isize, isize)) -> Option<Cell> {
        Some(
            *self
                .board
                .get(usize::try_from(y).ok()?)?
                .get(usize::try_from(x).ok()?)?,
        )
    }

    pub fn calc(&self, path: &[(isize, isize)]) -> Option<i64> {
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
}

#[cfg(test)]
mod test {
    use super::Board;
    use super::Cell::*;
    use super::Operator::*;

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
}
