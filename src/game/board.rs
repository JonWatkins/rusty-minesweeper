use super::cell::{Cell, CellContent, CellState};
use crate::utils::{is_logging_enabled, is_show_mines_enabled};
use log::{debug, error};
use rand::Rng;

#[derive(Debug, Clone)]
pub struct GameBoard {
    pub board: Vec<Vec<Cell>>,
    pub width: usize,
    pub height: usize,
    pub mine_count: usize,
    pub game_over: bool,
    pub game_won: bool,
    pub first_click: bool,
    pub game_started: bool,
}

impl GameBoard {
    pub fn new(width: usize, height: usize, mine_count: usize) -> Self {
        if is_logging_enabled() {
            debug!(
                "Creating new GameBoard: {}x{} with {} mines",
                width, height, mine_count
            );
        }

        let board = vec![vec![Cell::default(); width]; height];

        let mut game_board = Self {
            board,
            width,
            height,
            mine_count,
            game_over: false,
            game_won: false,
            first_click: true,
            game_started: false,
        };

        // If show_mines flag is enabled, place mines immediately for debugging
        if is_show_mines_enabled() {
            if is_logging_enabled() {
                debug!("Show mines flag enabled - placing mines immediately for debugging");
            }
            // Place mines excluding a random position to avoid immediate game over
            let mut rng = rand::thread_rng();
            let exclude_x = rng.gen_range(0..width);
            let exclude_y = rng.gen_range(0..height);
            game_board.place_mines(exclude_x, exclude_y);
            game_board.first_click = false; // Mark as if first click already happened
        }

        game_board
    }

    pub fn place_mines(&mut self, exclude_x: usize, exclude_y: usize) {
        if is_logging_enabled() {
            debug!(
                "Placing {} mines, excluding position ({}, {})",
                self.mine_count, exclude_x, exclude_y
            );
        }

        let mut rng = rand::thread_rng();
        let mut mines_placed = 0;

        while mines_placed < self.mine_count {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);

            if (x == exclude_x && y == exclude_y)
                || matches!(self.board[y][x].content, CellContent::Mine)
            {
                continue;
            }

            self.board[y][x].content = CellContent::Mine;
            mines_placed += 1;
        }

        if is_logging_enabled() {
            debug!("Successfully placed {} mines", mines_placed);
        }

        self.calculate_numbers();
    }

    fn calculate_numbers(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                if matches!(self.board[y][x].content, CellContent::Mine) {
                    continue;
                }

                let mine_count = self.count_adjacent_mines(x, y);
                if mine_count > 0 {
                    self.board[y][x].content = CellContent::Number(mine_count);
                } else {
                    self.board[y][x].content = CellContent::Empty;
                }
            }
        }
    }

    fn count_adjacent_mines(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0
                    && nx < self.width as i32
                    && ny >= 0
                    && ny < self.height as i32
                    && matches!(
                        self.board[ny as usize][nx as usize].content,
                        CellContent::Mine
                    )
                {
                    count += 1;
                }
            }
        }

        count
    }

    pub fn reveal_cell(&mut self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height || !self.game_started {
            if is_logging_enabled() {
                debug!(
                    "Cannot reveal cell ({}, {}): out of bounds or game not started",
                    x, y
                );
            }
            return false;
        }

        let cell_state = self.board[y][x].state;

        match cell_state {
            CellState::Hidden => {
                if is_logging_enabled() {
                    debug!("Revealing cell ({}, {})", x, y);
                }

                if self.first_click {
                    if is_logging_enabled() {
                        debug!("First click detected, placing mines");
                    }
                    self.place_mines(x, y);
                    self.first_click = false;
                }

                self.board[y][x].state = CellState::Revealed;

                match self.board[y][x].content {
                    CellContent::Mine => {
                        if is_logging_enabled() {
                            error!("Mine hit at ({}, {}) - game over!", x, y);
                        }
                        self.game_over = true;
                        self.reveal_all_mines();
                        return true;
                    }
                    CellContent::Empty => {
                        if is_logging_enabled() {
                            debug!("Empty cell revealed, revealing adjacent cells");
                        }
                        self.reveal_adjacent_cells(x, y);
                    }
                    CellContent::Number(_) => {
                        if is_logging_enabled() {
                            debug!("Number cell revealed");
                        }
                    }
                }

                self.check_win_condition();
                true
            }
            _ => {
                if is_logging_enabled() {
                    debug!(
                        "Cannot reveal cell ({}, {}): already revealed or flagged",
                        x, y
                    );
                }
                false
            }
        }
    }

    fn reveal_adjacent_cells(&mut self, x: usize, y: usize) {
        let mut cells_to_reveal = Vec::new();

        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }

                let nx = x as i32 + dx;
                let ny = y as i32 + dy;

                if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                    let nx = nx as usize;
                    let ny = ny as usize;

                    if matches!(self.board[ny][nx].state, CellState::Hidden) {
                        cells_to_reveal.push((nx, ny));
                    }
                }
            }
        }

        for (nx, ny) in cells_to_reveal {
            self.reveal_cell(nx, ny);
        }
    }

    fn reveal_all_mines(&mut self) {
        for row in &mut self.board {
            for cell in row {
                if matches!(cell.content, CellContent::Mine) {
                    cell.state = CellState::Revealed;
                }
            }
        }
    }

    fn check_win_condition(&mut self) {
        let mut unrevealed_non_mines = 0;

        for row in &self.board {
            for cell in row {
                if matches!(cell.state, CellState::Hidden)
                    && !matches!(cell.content, CellContent::Mine)
                {
                    unrevealed_non_mines += 1;
                }
            }
        }

        if unrevealed_non_mines == 0 {
            if is_logging_enabled() {
                debug!(
                    "Win condition met! All non-mine cells revealed. Unrevealed non-mines: {}",
                    unrevealed_non_mines
                );
            }
            self.game_won = true;
            self.game_over = true;
        }
    }

    pub fn toggle_flag(&mut self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height || self.game_over || !self.game_started {
            return false;
        }

        let cell = &mut self.board[y][x];

        match cell.state {
            CellState::Hidden => {
                cell.state = CellState::Flagged;
                true
            }
            CellState::Flagged => {
                cell.state = CellState::Hidden;
                true
            }
            _ => false,
        }
    }

    pub fn reset(&mut self) {
        self.board = vec![vec![Cell::default(); self.width]; self.height];
        self.game_over = false;
        self.game_won = false;
        self.first_click = true;
        self.game_started = false;
    }

    pub fn start_game(&mut self) {
        if is_logging_enabled() {
            debug!("Starting game");
        }
        self.game_started = true;
    }

    #[cfg(test)]
    pub fn get_cell_content(&self, x: usize, y: usize) -> CellContent {
        self.board[y][x].content
    }

    #[cfg(test)]
    pub fn get_cell_state(&self, x: usize, y: usize) -> CellState {
        self.board[y][x].state
    }

    #[cfg(test)]
    pub fn set_mine(&mut self, x: usize, y: usize) {
        self.board[y][x].content = CellContent::Mine;
    }

    #[cfg(test)]
    pub fn count_total_mines(&self) -> usize {
        let mut count = 0;
        for row in &self.board {
            for cell in row {
                if matches!(cell.content, CellContent::Mine) {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_initialization() {
        let board = GameBoard::new(10, 8, 15);

        assert_eq!(board.width, 10);
        assert_eq!(board.height, 8);
        assert_eq!(board.mine_count, 15);
        assert_eq!(board.game_over, false);
        assert_eq!(board.game_won, false);
        assert_eq!(board.first_click, true);
        assert_eq!(board.game_started, false);

        assert_eq!(board.board.len(), 8);
        assert_eq!(board.board[0].len(), 10);
        for row in &board.board {
            for cell in row {
                assert_eq!(cell.content, CellContent::Empty);
                assert_eq!(cell.state, CellState::Hidden);
            }
        }
    }

    #[test]
    fn test_mine_placement_excludes_first_click() {
        let mut board = GameBoard::new(5, 5, 5);
        board.start_game();

        board.place_mines(2, 2);

        assert_ne!(board.get_cell_content(2, 2), CellContent::Mine);
        assert_eq!(board.count_total_mines(), 5);
    }

    #[test]
    fn test_number_calculation_simple_case() {
        let mut board = GameBoard::new(3, 3, 0);
        board.start_game();

        board.set_mine(1, 1);
        board.calculate_numbers();
        assert_eq!(board.get_cell_content(0, 0), CellContent::Number(1));
        assert_eq!(board.get_cell_content(0, 1), CellContent::Number(1));
        assert_eq!(board.get_cell_content(0, 2), CellContent::Number(1));
        assert_eq!(board.get_cell_content(1, 0), CellContent::Number(1));
        assert_eq!(board.get_cell_content(1, 2), CellContent::Number(1));
        assert_eq!(board.get_cell_content(2, 0), CellContent::Number(1));
        assert_eq!(board.get_cell_content(2, 1), CellContent::Number(1));
        assert_eq!(board.get_cell_content(2, 2), CellContent::Number(1));

        assert_eq!(board.get_cell_content(1, 1), CellContent::Mine);
    }

    #[test]
    fn test_number_calculation_corner_mine() {
        let mut board = GameBoard::new(3, 3, 0);
        board.start_game();

        board.set_mine(0, 0);
        board.calculate_numbers();
        assert_eq!(board.get_cell_content(0, 1), CellContent::Number(1));
        assert_eq!(board.get_cell_content(1, 0), CellContent::Number(1));
        assert_eq!(board.get_cell_content(1, 1), CellContent::Number(1));

        assert_eq!(board.get_cell_content(0, 2), CellContent::Empty);
        assert_eq!(board.get_cell_content(1, 2), CellContent::Empty);
        assert_eq!(board.get_cell_content(2, 0), CellContent::Empty);
        assert_eq!(board.get_cell_content(2, 1), CellContent::Empty);
        assert_eq!(board.get_cell_content(2, 2), CellContent::Empty);
    }

    #[test]
    fn test_number_calculation_multiple_mines() {
        let mut board = GameBoard::new(3, 3, 0);
        board.start_game();

        board.set_mine(0, 0);
        board.set_mine(1, 1);
        board.calculate_numbers();

        assert_eq!(board.get_cell_content(0, 1), CellContent::Number(2));

        assert_eq!(board.get_cell_content(1, 0), CellContent::Number(2));

        assert_eq!(board.get_cell_content(2, 2), CellContent::Number(1));
    }

    #[test]
    fn test_count_adjacent_mines() {
        let mut board = GameBoard::new(3, 3, 0);
        board.start_game();

        board.set_mine(0, 0);
        board.set_mine(2, 2);

        assert_eq!(board.count_adjacent_mines(1, 1), 2);

        assert_eq!(board.count_adjacent_mines(0, 2), 0);

        assert_eq!(board.count_adjacent_mines(1, 0), 1);

        assert_eq!(board.count_adjacent_mines(0, 1), 1);

        assert_eq!(board.count_adjacent_mines(1, 2), 1);
    }

    #[test]
    fn test_reveal_cell_basic() {
        let mut board = GameBoard::new(3, 3, 1);
        board.start_game();

        let result = board.reveal_cell(1, 1);
        assert!(result);
        assert_eq!(board.get_cell_state(1, 1), CellState::Revealed);
        assert_eq!(board.first_click, false);
    }

    #[test]
    fn test_reveal_cell_out_of_bounds() {
        let mut board = GameBoard::new(3, 3, 1);
        board.start_game();

        let result = board.reveal_cell(5, 5);
        assert!(!result);
    }

    #[test]
    fn test_reveal_cell_game_not_started() {
        let mut board = GameBoard::new(3, 3, 1);
        let result = board.reveal_cell(1, 1);
        assert!(!result);
    }

    #[test]
    fn test_reveal_cell_already_revealed() {
        let mut board = GameBoard::new(3, 3, 1);
        board.start_game();

        let result1 = board.reveal_cell(1, 1);
        assert!(result1);

        let result2 = board.reveal_cell(1, 1);
        assert!(!result2);
    }

    #[test]
    fn test_reveal_cell_cascade_empty() {
        let mut board = GameBoard::new(3, 3, 0);
        board.start_game();

        board.set_mine(2, 2);
        board.calculate_numbers();

        board.reveal_cell(0, 0);

        for y in 0..3 {
            for x in 0..3 {
                if !(x == 2 && y == 2) {
                    assert_eq!(board.get_cell_state(x, y), CellState::Revealed);
                }
            }
        }

        assert_eq!(board.get_cell_state(2, 2), CellState::Hidden);
    }

    #[test]
    fn test_game_over_on_mine_hit() {
        let mut board = GameBoard::new(3, 3, 1);
        board.start_game();

        board.set_mine(1, 1);
        board.calculate_numbers();
        board.first_click = false;

        let result = board.reveal_cell(1, 1);
        assert!(result);
        assert!(board.game_over);
        assert!(!board.game_won);

        assert_eq!(board.get_cell_state(1, 1), CellState::Revealed);
    }

    #[test]
    fn test_win_condition() {
        let mut board = GameBoard::new(2, 2, 1);
        board.start_game();

        board.set_mine(1, 1);
        board.calculate_numbers();
        board.first_click = false;

        board.reveal_cell(0, 0);
        board.reveal_cell(0, 1);
        board.reveal_cell(1, 0);

        assert!(board.game_won);
        assert!(board.game_over);
    }

    #[test]
    fn test_toggle_flag() {
        let mut board = GameBoard::new(3, 3, 1);
        board.start_game();

        let result1 = board.toggle_flag(1, 1);
        assert!(result1);
        assert_eq!(board.get_cell_state(1, 1), CellState::Flagged);

        let result2 = board.toggle_flag(1, 1);
        assert!(result2);
        assert_eq!(board.get_cell_state(1, 1), CellState::Hidden);

        board.reveal_cell(0, 0);
        let result3 = board.toggle_flag(0, 0);
        assert!(!result3);
    }

    #[test]
    fn test_toggle_flag_out_of_bounds() {
        let mut board = GameBoard::new(3, 3, 1);
        board.start_game();

        let result = board.toggle_flag(5, 5);
        assert!(!result);
    }

    #[test]
    fn test_toggle_flag_game_over() {
        let mut board = GameBoard::new(3, 3, 1);
        board.start_game();

        board.game_over = true;

        let result = board.toggle_flag(1, 1);
        assert!(!result);
    }

    #[test]
    fn test_reset() {
        let mut board = GameBoard::new(3, 3, 1);
        board.start_game();

        board.reveal_cell(1, 1);
        board.toggle_flag(0, 0);

        board.reset();

        assert_eq!(board.game_over, false);
        assert_eq!(board.game_won, false);
        assert_eq!(board.first_click, true);
        assert_eq!(board.game_started, false);

        for row in &board.board {
            for cell in row {
                assert_eq!(cell.state, CellState::Hidden);
                assert_eq!(cell.content, CellContent::Empty);
            }
        }
    }

    #[test]
    fn test_start_game() {
        let mut board = GameBoard::new(3, 3, 1);

        assert_eq!(board.game_started, false);

        board.start_game();

        assert_eq!(board.game_started, true);
    }

    #[test]
    fn test_edge_case_single_cell_board() {
        let mut board = GameBoard::new(1, 1, 0);
        board.start_game();

        board.reveal_cell(0, 0);

        assert!(board.game_won);
        assert!(board.game_over);
    }

    #[test]
    fn test_edge_case_all_mines_except_one() {
        let mut board = GameBoard::new(2, 2, 3);
        board.start_game();

        board.set_mine(0, 0);
        board.set_mine(0, 1);
        board.set_mine(1, 0);
        board.calculate_numbers();
        board.first_click = false;

        board.reveal_cell(1, 1);

        assert!(board.game_won);
        assert!(board.game_over);
    }
}
