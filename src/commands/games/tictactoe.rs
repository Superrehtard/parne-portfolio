use crate::commands::{games, CommandOutput, LineStyle};

#[derive(Clone)]
pub struct TicTacToe {
    board: [Option<char>; 9],
    player_turn: bool,
    pub game_over: bool,
}

impl TicTacToe {
    pub fn new() -> Self {
        Self {
            board: [None; 9],
            player_turn: true,
            game_over: false,
        }
    }

    pub fn start_output() -> CommandOutput {
        let mut lines = vec![
            (String::new(), LineStyle::Normal),
            ("  === TIC-TAC-TOE ===".to_string(), LineStyle::Accent),
            (
                "  You are X. Enter a number 1-9 to place your mark.".to_string(),
                LineStyle::Normal,
            ),
            (
                "  Type 'quit' to exit the game.".to_string(),
                LineStyle::Muted,
            ),
            (String::new(), LineStyle::Normal),
        ];

        let board_lines = Self::render_empty_board();
        lines.extend(board_lines);
        lines.push((String::new(), LineStyle::Normal));
        lines.push(("  Your move (1-9):".to_string(), LineStyle::Accent));

        CommandOutput::new(lines, false, false)
    }

    pub fn handle_input(&mut self, input: &str) -> CommandOutput {
        let input = input.trim().to_lowercase();

        if self.game_over {
            return Self::start_output();
        }

        // Parse move
        let pos: usize = match input.parse::<usize>() {
            Ok(n) if n >= 1 && n <= 9 => n - 1,
            _ => {
                return CommandOutput::new(
                    vec![(
                        "  Invalid move! Enter a number 1-9.".to_string(),
                        LineStyle::Error,
                    )],
                    false,
                    false,
                );
            }
        };

        // Check if position is taken
        if self.board[pos].is_some() {
            return CommandOutput::new(
                vec![(
                    "  That spot is taken! Try another.".to_string(),
                    LineStyle::Error,
                )],
                false,
                false,
            );
        }

        // Player move
        self.board[pos] = Some('X');

        if let Some(winner) = self.check_winner() {
            self.game_over = true;
            return self.game_over_output(winner);
        }

        if self.is_full() {
            self.game_over = true;
            return self.game_over_output(' ');
        }

        // Computer move
        self.computer_move();

        if let Some(winner) = self.check_winner() {
            self.game_over = true;
            return self.game_over_output(winner);
        }

        if self.is_full() {
            self.game_over = true;
            return self.game_over_output(' ');
        }

        // Show board and prompt next move
        let mut lines = vec![(String::new(), LineStyle::Normal)];
        lines.extend(self.render_board());
        lines.push((String::new(), LineStyle::Normal));
        lines.push(("  Your move (1-9):".to_string(), LineStyle::Accent));

        CommandOutput::new(lines, false, false)
    }

    fn computer_move(&mut self) {
        // Simple AI: try to win, then block, then take center, then random
        if let Some(pos) = self.find_winning_move('O') {
            self.board[pos] = Some('O');
        } else if let Some(pos) = self.find_winning_move('X') {
            self.board[pos] = Some('O');
        } else if self.board[4].is_none() {
            self.board[4] = Some('O');
        } else {
            // Take first available
            for i in 0..9 {
                if self.board[i].is_none() {
                    self.board[i] = Some('O');
                    break;
                }
            }
        }
    }

    fn find_winning_move(&self, player: char) -> Option<usize> {
        let lines = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8], // rows
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8], // cols
            [0, 4, 8],
            [2, 4, 6], // diagonals
        ];

        for line in &lines {
            let vals: Vec<Option<char>> = line.iter().map(|&i| self.board[i]).collect();
            let count = vals.iter().filter(|&&v| v == Some(player)).count();
            let empty = vals.iter().position(|&v| v.is_none());

            if count == 2 {
                if let Some(empty_idx) = empty {
                    return Some(line[empty_idx]);
                }
            }
        }
        None
    }

    fn check_winner(&self) -> Option<char> {
        let lines = [
            [0, 1, 2],
            [3, 4, 5],
            [6, 7, 8],
            [0, 3, 6],
            [1, 4, 7],
            [2, 5, 8],
            [0, 4, 8],
            [2, 4, 6],
        ];

        for line in &lines {
            if let Some(ch) = self.board[line[0]] {
                if self.board[line[1]] == Some(ch) && self.board[line[2]] == Some(ch) {
                    return Some(ch);
                }
            }
        }
        None
    }

    fn is_full(&self) -> bool {
        self.board.iter().all(|c| c.is_some())
    }

    fn game_over_output(&self, winner: char) -> CommandOutput {
        let mut lines = vec![(String::new(), LineStyle::Normal)];
        lines.extend(self.render_board());
        lines.push((String::new(), LineStyle::Normal));

        match winner {
            'X' => {
                lines.push(("  You win! Congratulations!".to_string(), LineStyle::Accent));
            }
            'O' => {
                lines.push((
                    "  Computer wins! Better luck next time.".to_string(),
                    LineStyle::Error,
                ));
            }
            _ => {
                lines.push(("  It's a draw!".to_string(), LineStyle::Muted));
            }
        }

        lines.push((
            "  Type 'ttt' to play again, or 'quit' to exit.".to_string(),
            LineStyle::Muted,
        ));

        CommandOutput::new(lines, false, false)
    }

    fn cell(&self, i: usize) -> String {
        match self.board[i] {
            Some('X') => "X".to_string(),
            Some('O') => "O".to_string(),
            _ => (i + 1).to_string(),
        }
    }

    fn render_board(&self) -> Vec<(String, LineStyle)> {
        vec![
            (
                format!("   {} | {} | {}", self.cell(0), self.cell(1), self.cell(2)),
                LineStyle::Normal,
            ),
            ("   ---------".to_string(), LineStyle::Muted),
            (
                format!("   {} | {} | {}", self.cell(3), self.cell(4), self.cell(5)),
                LineStyle::Normal,
            ),
            ("   ---------".to_string(), LineStyle::Muted),
            (
                format!("   {} | {} | {}", self.cell(6), self.cell(7), self.cell(8)),
                LineStyle::Normal,
            ),
        ]
    }

    fn render_empty_board() -> Vec<(String, LineStyle)> {
        vec![
            ("   1 | 2 | 3".to_string(), LineStyle::Normal),
            ("   ---------".to_string(), LineStyle::Muted),
            ("   4 | 5 | 6".to_string(), LineStyle::Normal),
            ("   ---------".to_string(), LineStyle::Muted),
            ("   7 | 8 | 9".to_string(), LineStyle::Normal),
        ]
    }
}
