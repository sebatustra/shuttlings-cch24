use std::{fmt::Display, sync::Arc};
use axum::{extract::{Path, State}, http::StatusCode};
use rand::Rng;
use crate::AppState;
use rand::SeedableRng;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TileState {
    Empty,
    Cookie,
    Milk
}

#[derive(Debug)]
pub enum BoardError {
    ColumnIsFull,
    GameIsOver,
    InvalidTeam,
    InvalidColumn,
}

impl Display for TileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let emoji = match self {
            TileState::Empty => "⬛",
            TileState::Cookie => "🍪",
            TileState::Milk => "🥛",
        };

        write!(f, "{}", emoji)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub grid: [[TileState; 4]; 4]
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut final_grid: Vec<String> = Vec::new();

        for i in 0..4 {
            let mut current_row = String::from("⬜");
            current_row.extend(self.grid[i].iter().map(|tile| tile.to_string()));
            current_row.push_str("⬜");
            final_grid.push(current_row);
        }

        final_grid.push("⬜⬜⬜⬜⬜⬜\n".to_string());

        write!(f, "{}", final_grid.join("\n"))
    }
}

impl Board {
    pub fn new() -> Self {
        let grid = [[TileState::Empty; 4]; 4];
        Self {
            grid
        }
    }

    pub fn create_random_board(rng: &mut rand::rngs::StdRng) -> Self {
        let mut grid = [[TileState::Empty; 4]; 4]; 

        for row in grid.iter_mut() {
            for tile in row.iter_mut() {
                match rng.gen::<bool>() {
                    true => *tile = TileState::Cookie,
                    false => *tile = TileState::Milk
                }
            }
        }

        Self {
            grid
        }
    }

    pub fn reset(&mut self) {
        let empty_grid = [[TileState::Empty; 4]; 4];

        self.grid = empty_grid
    }

    pub fn place_item(&mut self, team: String, column: usize) -> Result<Option<String>, BoardError> {

        if self.get_winner().is_some() {
            return Err(BoardError::GameIsOver)
        }

        let tile_state = match team.as_str() {
            "cookie" => TileState::Cookie,
            "milk" => TileState::Milk,
            _ => return Err(BoardError::InvalidTeam)
        };

        if column < 1 || column > 4 {
            return Err(BoardError::InvalidColumn)
        }

        let column_index = column - 1;


        let mut found_empty_tile = false;

        for row_index in (0..4).rev() {
            if self.grid[row_index][column_index] == TileState::Empty {
                self.grid[row_index][column_index] = tile_state;
                found_empty_tile = true;
                break
            }
        }

        if !found_empty_tile {
            return Err(BoardError::ColumnIsFull)
        }

        Ok(self.get_winner())
    }

    pub fn get_winner(&self) -> Option<String> {
        let grid = self.grid;
        
        for row in grid.iter() {
            if row.iter().all(|tile| *tile != TileState::Empty && *tile == row[0]) {
                println!("found row that wins");
                return Some(format!("{} wins!\n", row[0]))
            }
        }

        for i in 0..4 {
            let mut column_tiles: Vec<TileState> = Vec::new();
            for j in 0..4 {
                let tile = grid[j][i];
                column_tiles.push(tile);
            }
            if column_tiles.iter().all(|tile| *tile != TileState::Empty && *tile == column_tiles[0]) {
                println!("found column that wins");
                return Some(format!("{} wins!\n", column_tiles[0]))
            }
        }

        let mut first_diagonal: Vec<TileState> = Vec::new();
        for i in 0..4 {
            first_diagonal.push(grid[i][i]);
        }
        if first_diagonal.iter().all(|tile| *tile != TileState::Empty && *tile == first_diagonal[0]) {
            println!("found diagonal that wins");
            return Some(format!("{} wins!\n", first_diagonal[0]))
        }

        let mut second_diagonal: Vec<TileState> = Vec::new();

        for i in 0..4 {
            second_diagonal.push(grid[3 - i][i])
        }

        if second_diagonal.iter().all(|tile| *tile != TileState::Empty && *tile == second_diagonal[0]) {
            println!("found diagonal that wins");
            return Some(format!("{} wins!\n", second_diagonal[0]))
        }

        let mut found_empty_tile = false;
        for row in grid.iter() {
            for tile in row.iter() {
                if *tile == TileState::Empty {
                    found_empty_tile = true;
                    break
                }
            }
        }

        if found_empty_tile == false {
            return Some(String::from("No winner.\n"))
        }

        return None
    }


}

pub async fn create_board(
    State(state): State<Arc<AppState>>
) -> (StatusCode, String) {
    let board = state.board.lock().await;

    match board.get_winner() {
        Some(text) => {
            let mut board_string = board.to_string();
            board_string.push_str(&text);

            (StatusCode::OK, board_string)
        },
        None => (StatusCode::OK, board.to_string())
    }
}

pub async fn reset_board(
    State(state): State<Arc<AppState>>
) -> (StatusCode, String) {
    let mut board = state.board.lock().await;

    board.reset();

    let mut rng = state.rng.lock().await;
    *rng = rand::rngs::StdRng::seed_from_u64(2024);


    (StatusCode::OK, board.to_string())
}

pub async fn place_item(
    Path((team, column)): Path<(String, usize)>,
    State(state): State<Arc<AppState>>,
) -> (StatusCode, String) {
    let mut board = state.board.lock().await;

    match board.place_item(team, column) {
        Ok(winner_option) => {
            match winner_option {
                Some(text) => {
                    let mut board_string = board.to_string();
                    board_string.push_str(&text);

                    (StatusCode::OK, board_string)
                },
                None => (StatusCode::OK, board.to_string())
            }
        },
        Err(error) => {
            return match error {
                BoardError::ColumnIsFull => (StatusCode::SERVICE_UNAVAILABLE, board.to_string()),
                BoardError::GameIsOver => {
                    let winner_option = board.get_winner();
                    match winner_option {
                        Some(text) => {
                            let mut board_string = board.to_string();
                            board_string.push_str(&text);
        
                            (StatusCode::SERVICE_UNAVAILABLE, board_string)
                        },
                        None => (StatusCode::SERVICE_UNAVAILABLE, board.to_string())
                    }
                },
                BoardError::InvalidColumn => (StatusCode::BAD_REQUEST, String::default()),
                BoardError::InvalidTeam => (StatusCode::BAD_REQUEST, String::default()),
            }
        }
    }
}

pub async fn generate_random_board(
    State(state): State<Arc<AppState>>,
) -> (StatusCode, String) {
    let rng = &mut state.rng.lock().await;

    let random_board = Board::create_random_board(rng);

    let winner_option = random_board.get_winner();

    match winner_option {
        Some(text) => {
            let mut board_string = random_board.to_string();
            board_string.push_str(&text);

            (StatusCode::OK, board_string)
        },
        None => (StatusCode::OK, random_board.to_string())
    }
}