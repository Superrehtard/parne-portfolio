pub mod tictactoe;
pub mod typing_test;

use crate::commands::{CommandOutput, LineStyle};
use tictactoe::TicTacToe;
use typing_test::TypingTest;

#[derive(Clone)]
pub enum ActiveGame {
    TicTacToe(TicTacToe),
    TypingTest(TypingTest),
}

impl ActiveGame {
    pub fn handle_input(&mut self, input: &str) -> CommandOutput {
        match self {
            ActiveGame::TicTacToe(game) => game.handle_input(input),
            ActiveGame::TypingTest(game) => game.handle_input(input),
        }
    }

    pub fn is_finished(&self) -> bool {
        match self {
            ActiveGame::TicTacToe(game) => game.game_over,
            ActiveGame::TypingTest(game) => game.finished,
        }
    }
}
