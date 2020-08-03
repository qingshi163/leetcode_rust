const N1: i32 = 0b1;
const N2: i32 = 0b10;
const N3: i32 = 0b100;
const N4: i32 = 0b1000;
const N5: i32 = 0b10000;
const N6: i32 = 0b100000;
const N7: i32 = 0b1000000;
const N8: i32 = 0b10000000;
const N9: i32 = 0b100000000;
const N_ALL: i32 = 0b111111111;

#[derive(Copy, Clone)]
struct Cell(char, i32);
#[derive(Clone)]
struct Snapshot {
    pub board: [[Cell; 9]; 9],
}
impl Snapshot {
    pub fn new(origin: &[Vec<char>]) -> Snapshot {
        let mut _self = Snapshot {
            board: [[Cell('0', N_ALL); 9]; 9],
        };
        all().for_each(|(i, j)| {
            _self.board[i][j].0 = origin[i][j];
        });
        _self.hint_init();
        _self
    }
    pub fn hint_set(&mut self, i: usize, j: usize, n: i32) {
        self.board[i].iter_mut().for_each(|x| x.1 &= !n);
        self.board.iter_mut().for_each(|row| row[j].1 &= !n);
        let bi = i - i % 3;
        let bj = j - j % 3;
        for a in 0..3 {
            for b in 0..3 {
                self.board[bi+a][bj+b].1 &= !n;
            }
        }
    }
    pub fn hint_init(&mut self) {
        all().for_each(|(i, j)| {
            if !is_nil(self.board[i][j].0) {
                self.hint_set(i, j, c2n(self.board[i][j].0));
            }
        });
    }
    pub fn try_solve(&self, i: usize, j: usize, c: char) -> Option<Snapshot> {
        let mut ss = self.clone();
        ss.board[i][j].0 = c;
        ss.hint_set(i, j, c2n(c));
        ss.solve()
    }
    pub fn solve(&self) -> Option<Snapshot> {
        let (i, j) = match all().filter(|&(i, j)| is_nil(self.board[i][j].0)).min_by(
            |&(i1, j1), &(i2, j2)| {
                self.board[i1][j1].1.count_ones()
                    .cmp(&self.board[i2][j2].1.count_ones())
            },
        ) {
            Some((i, j)) => (i, j),
            None => return Some(self.clone()),
        };
        if self.board[i][j].1.count_ones() == 0 {
            return None;
        }
        for a in 0..9 {
            if self.board[i][j].1 & (1 << a) != 0 {
                if let Some(res) = self.try_solve(i, j, n2c(1 << a)) {
                    return Some(res);
                }
            }
        }
        None
    }
}
fn is_nil(c: char) -> bool {
    c == '.'
}
fn c2n(c: char) -> i32 {
    match c {
        '1' => N1,
        '2' => N2,
        '3' => N3,
        '4' => N4,
        '5' => N5,
        '6' => N6,
        '7' => N7,
        '8' => N8,
        '9' => N9,
        _ => 0,
    }
}
fn n2c(n: i32) -> char {
    match n {
        N1 => '1',
        N2 => '2',
        N3 => '3',
        N4 => '4',
        N5 => '5',
        N6 => '6',
        N7 => '7',
        N8 => '8',
        N9 => '9',
        _ => '.',
    }
}
fn all() -> impl Iterator<Item = (usize, usize)> {
    (0..9).flat_map(move |i| (0..9).map(move |j| (i, j)))
}

#[allow(dead_code)]
pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
    let ss = Snapshot::new(&board);
    match ss.solve() {
        Some(res) => {
            all().for_each(|(i, j)| {
                board[i][j] = res.board[i][j].0;
            });
        }
        None => println!("Error: Not find Solutino."),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_37() {
        let mut board = vec![
            vec!['8', '3', '.', '.', '.', '7', '2', '5', '9'],
            vec!['5', '9', '7', '2', '8', '3', '4', '1', '6'],
            vec!['.', '.', '.', '6', '.', '5', '7', '8', '.'],
            vec!['.', '5', '.', '.', '7', '2', '.', '4', '8'],
            vec!['.', '.', '.', '5', '6', '8', '9', '.', '.'],
            vec!['7', '8', '.', '.', '4', '1', '.', '.', '5'],
            vec!['.', '1', '8', '.', '.', '4', '3', '6', '2'],
            vec!['2', '.', '4', '.', '3', '.', '5', '7', '.'],
            vec!['3', '7', '5', '.', '.', '6', '8', '9', '.'],
        ];
        solve_sudoku(&mut board);
        for row in board {
            for cell in row {
                print!("{} ", cell);
            }
            println!();
        }
    }
}
