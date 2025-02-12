use std::error::Error;


const NUM_COLS : usize = 7;
const NUM_ROWS : usize = 6;

#[repr(u8)]
#[derive(Clone, Copy, PartialEq)]
enum Piece {
    Red,
    Yellow,
    Blank,
}

enum ValidInput {
    Col(usize),
    Quit,
}

struct State {
    board: [[Piece; NUM_COLS]; NUM_ROWS],
    player: Piece,
    has_won: bool,
}

impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Piece::Red => "🔴",
            Piece::Yellow => "🟡",
            Piece::Blank => "⬛",
        };
        f.write_str(c)
    }
}

impl State {
    fn new() -> Self {
        let board = [[Piece::Blank; NUM_COLS]; NUM_ROWS];
        Self { board, player: Piece::Red, has_won: false }
    }

    fn prompt_user(&self) -> ValidInput {
        use ValidInput::*;
        loop {
            let mut buf = String::new();
            if std::io::stdin().read_line(&mut buf).is_err() {
                println!("bad input!");
                continue;
            }
            let buf = buf.trim();
            match buf {
                "1"|"2"|"3"|"4"|"5"|"6"|"7" => {
                    let col : usize = buf.parse().unwrap();
                    if !self.check_valid(col-1) {
                        println!("not a valid move");
                        continue;
                    }
                    break Col(col-1);
                }
                "q" => {
                    break Quit
                }
                _ => {
                    println!("expectd 1-7 or q");
                    continue
                }
            }
        }
    }

    fn check_valid(&self, col: usize) -> bool {
        matches!(self.board[0][col], Piece::Blank)
    }

    fn make_move(&mut self, col: usize, player: Piece) {
        let row = self
            .board
            .iter_mut()
            .rev()
            .find(|r| matches!(r[col], Piece::Blank))
            .unwrap();
        row[col] = player;
    }

    fn has_someone_won(&self, piece: Piece) -> bool {
        for row in 0..self.board.len() {
            for col in 0..self.board[row].len() {
                if self.board[row][col] != piece {
                    continue;
                }

                if 
                    self.is_win_h(row, col, piece) ||
                    self.is_win_v(row, col, piece) ||
                    self.is_win_dr(row, col, piece) ||
                    self.is_win_ur(row, col, piece)
                {
                    return true;
                }
            }
        }
        false
    }

    fn next_player(&mut self) {
        self.player = match self.player {
            Piece::Red => Piece::Yellow,
            Piece::Yellow => Piece::Red,
            Piece::Blank => Piece::Blank,
        }
    }
    
    fn player(&self) -> Piece {
        self.player
    }
    
    fn is_win_h(&self, row: usize, col: usize, piece: Piece) -> bool {
        if col > NUM_COLS-4 {
            return false;
        }
        (0..4).all(|i| self.board[row][col+i] == piece)
    }
    
    fn is_win_v(&self, row: usize, col: usize, piece: Piece) -> bool {
        if row > NUM_ROWS-4 {
            return false;
        }
        (0..4).all(|i| self.board[row+i][col] == piece)
    }

    fn is_win_dr(&self, row: usize, col: usize, piece: Piece) -> bool {
        if row > NUM_ROWS-4 || col>NUM_COLS-4 {
            return false;
        }
        // (0..4).for_each(|i| println!("{}", self.board[row+i][col+i] == piece));
        (0..4).all(|i| self.board[row+i][col+i] == piece)
    }

    fn is_win_ur(&self, row: usize, col: usize, piece: Piece) -> bool {
        if row > NUM_ROWS-4 || col >= 4 {
            return false;
        }
        (0..4).all(|i| self.board[row+i][col-i] == piece)
    }

    fn set_has_won(&mut self, has_won: bool) {
        self.has_won = has_won;
    }
    
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 1..=NUM_COLS {
            i.fmt(f)?;
            " ".fmt(f)?;
        }
        f.write_str("\n")?;
        for row in &self.board {
            for col in row {
                col.fmt(f)?;
            }
            f.write_str("\n")?;
        }
        if self.has_won {
            writeln!(f, "{} has won!!!\n", self.player)?;
        } else {
            writeln!(f, "{} to play:\n", self.player)?;
        }
        Ok(())
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut state = State::new();
    loop {
        println!("{state}");
        let input = state.prompt_user();
        match input {
            ValidInput::Col(col) => {
                state.make_move(col, state.player());
                if state.has_someone_won(state.player()) {
                    state.set_has_won(true);
                    println!("{state}");
                    break;
                }
                state.next_player();
            }
            ValidInput::Quit => {
                break;
            }
        }
    }
    Ok(())
}

#[test]
fn test_dr() {
    use Piece::*;
    let state = State {
        board: [
            [Blank, Blank, Blank, Blank, Blank, Blank, Blank], 
            [Blank, Blank, Blank, Blank, Blank, Blank, Blank], 
            [Yellow, Blank, Blank, Blank, Blank, Blank, Blank], 
            [Blank, Yellow, Blank, Blank, Blank, Blank, Blank], 
            [Blank, Blank, Yellow, Blank, Blank, Blank, Blank], 
            [Blank, Blank, Blank, Yellow, Blank, Blank, Blank], 
        ],
        player: Yellow,
        has_won: false,
    };

    assert!(state.is_win_dr(2, 0, Yellow));
}

