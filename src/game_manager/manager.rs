use super::Difficulty;
use crate::game::GameBoard;
use crate::timer::GameTimer;
use crate::utils::is_logging_enabled;
use log::debug;

#[derive(Debug, Clone)]
pub struct GameManager {
    pub timer: GameTimer,
    pub current_difficulty: Difficulty,
}

impl GameManager {
    pub fn new() -> Self {
        if is_logging_enabled() {
            debug!("Creating new GameManager");
        }
        Self {
            timer: GameTimer::new(),
            current_difficulty: Difficulty::Beginner,
        }
    }

    pub fn start_game(&mut self, difficulty: Difficulty) {
        if is_logging_enabled() {
            debug!("Starting game with difficulty: {:?}", difficulty);
        }
        self.current_difficulty = difficulty;
        self.timer.reset();
        self.timer.start();
    }

    pub fn end_game(&mut self, won: bool) {
        if is_logging_enabled() {
            debug!("Ending game. Won: {}", won);
        }
        self.timer.pause();
    }

    pub fn pause_game(&mut self) {
        if is_logging_enabled() {
            debug!("Pausing game");
        }
        self.timer.pause();
    }

    pub fn resume_game(&mut self) {
        if is_logging_enabled() {
            debug!("Resuming game");
        }
        self.timer.start();
    }

    pub fn reset_game(&mut self, game_board: &mut GameBoard) {
        if is_logging_enabled() {
            debug!(
                "Resetting game with current difficulty: {:?}",
                self.current_difficulty
            );
        }
        let (width, height, mines) = self.current_difficulty.get_dimensions();
        *game_board = GameBoard::new(width, height, mines);
        self.timer.reset();
    }
}

impl Default for GameManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_game_manager_initialization() {
        let manager = GameManager::new();

        assert_eq!(manager.current_difficulty, Difficulty::Beginner);
        assert!(!manager.timer.is_running());
        assert_eq!(manager.timer.get_elapsed(), Duration::ZERO);
    }

    #[test]
    fn test_game_manager_default() {
        let manager = GameManager::default();

        assert_eq!(manager.current_difficulty, Difficulty::Beginner);
        assert!(!manager.timer.is_running());
    }

    #[test]
    fn test_start_game() {
        let mut manager = GameManager::new();

        manager.start_game(Difficulty::Expert);

        assert_eq!(manager.current_difficulty, Difficulty::Expert);
        assert!(manager.timer.is_running());
    }

    #[test]
    fn test_start_game_different_difficulties() {
        let mut manager = GameManager::new();

        manager.start_game(Difficulty::Beginner);
        assert_eq!(manager.current_difficulty, Difficulty::Beginner);
        assert!(manager.timer.is_running());

        manager.start_game(Difficulty::Intermediate);
        assert_eq!(manager.current_difficulty, Difficulty::Intermediate);
        assert!(manager.timer.is_running());

        manager.start_game(Difficulty::Expert);
        assert_eq!(manager.current_difficulty, Difficulty::Expert);
        assert!(manager.timer.is_running());
    }

    #[test]
    fn test_end_game_won() {
        let mut manager = GameManager::new();

        manager.start_game(Difficulty::Beginner);
        assert!(manager.timer.is_running());

        manager.end_game(true);
        assert!(!manager.timer.is_running());
    }

    #[test]
    fn test_end_game_lost() {
        let mut manager = GameManager::new();

        manager.start_game(Difficulty::Beginner);
        assert!(manager.timer.is_running());

        manager.end_game(false);
        assert!(!manager.timer.is_running());
    }

    #[test]
    fn test_pause_and_resume_game() {
        let mut manager = GameManager::new();

        manager.start_game(Difficulty::Beginner);
        assert!(manager.timer.is_running());

        manager.pause_game();
        assert!(!manager.timer.is_running());

        manager.resume_game();
        assert!(manager.timer.is_running());
    }

    #[test]
    fn test_reset_game() {
        let mut manager = GameManager::new();
        let mut game_board = GameBoard::new(9, 9, 10);

        manager.start_game(Difficulty::Expert);
        game_board.start_game();
        game_board.reveal_cell(0, 0);

        assert!(game_board.game_started);
        assert!(!game_board.first_click);

        manager.reset_game(&mut game_board);

        assert_eq!(manager.current_difficulty, Difficulty::Expert);
        assert!(!manager.timer.is_running());
        assert_eq!(manager.timer.get_elapsed(), Duration::ZERO);

        assert!(!game_board.game_started);
        assert!(game_board.first_click);
        assert!(!game_board.game_over);
        assert!(!game_board.game_won);
    }

    #[test]
    fn test_reset_game_different_difficulty() {
        let mut manager = GameManager::new();
        let mut game_board = GameBoard::new(9, 9, 10);

        manager.current_difficulty = Difficulty::Expert;

        manager.reset_game(&mut game_board);

        let (width, height, mines) = Difficulty::Expert.get_dimensions();
        assert_eq!(game_board.width, width);
        assert_eq!(game_board.height, height);
        assert_eq!(game_board.mine_count, mines);
    }

    #[test]
    fn test_timer_integration() {
        let mut manager = GameManager::new();

        manager.start_game(Difficulty::Beginner);
        thread::sleep(Duration::from_millis(10));

        let elapsed_before_pause = manager.timer.get_elapsed();
        assert!(elapsed_before_pause > Duration::ZERO);

        manager.pause_game();
        thread::sleep(Duration::from_millis(10));
        let elapsed_after_pause = manager.timer.get_elapsed();
        let time_diff =
            elapsed_after_pause.as_millis() as i64 - elapsed_before_pause.as_millis() as i64;
        assert!(time_diff.abs() < 5);

        manager.resume_game();
        thread::sleep(Duration::from_millis(10));
        let elapsed_after_resume = manager.timer.get_elapsed();
        assert!(elapsed_after_resume > elapsed_after_pause);
    }

    #[test]
    fn test_game_manager_clone() {
        let mut manager = GameManager::new();
        manager.start_game(Difficulty::Intermediate);

        let cloned = manager.clone();
        assert_eq!(manager.current_difficulty, cloned.current_difficulty);
        assert_eq!(manager.timer.is_running(), cloned.timer.is_running());
    }

    #[test]
    fn test_game_manager_debug() {
        let manager = GameManager::new();
        let debug_str = format!("{:?}", manager);
        assert!(debug_str.contains("GameManager"));
        assert!(debug_str.contains("Beginner"));
    }
}
