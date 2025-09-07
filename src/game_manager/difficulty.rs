#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Expert,
}

impl Difficulty {
    pub fn get_dimensions(&self) -> (usize, usize, usize) {
        match self {
            Difficulty::Beginner => (9, 9, 10),
            Difficulty::Intermediate => (16, 16, 40),
            Difficulty::Expert => (30, 16, 99),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_beginner_difficulty() {
        let difficulty = Difficulty::Beginner;
        let (width, height, mines) = difficulty.get_dimensions();

        assert_eq!(width, 9);
        assert_eq!(height, 9);
        assert_eq!(mines, 10);
    }

    #[test]
    fn test_intermediate_difficulty() {
        let difficulty = Difficulty::Intermediate;
        let (width, height, mines) = difficulty.get_dimensions();

        assert_eq!(width, 16);
        assert_eq!(height, 16);
        assert_eq!(mines, 40);
    }

    #[test]
    fn test_expert_difficulty() {
        let difficulty = Difficulty::Expert;
        let (width, height, mines) = difficulty.get_dimensions();

        assert_eq!(width, 30);
        assert_eq!(height, 16);
        assert_eq!(mines, 99);
    }

    #[test]
    fn test_difficulty_equality() {
        assert_eq!(Difficulty::Beginner, Difficulty::Beginner);
        assert_ne!(Difficulty::Beginner, Difficulty::Intermediate);
        assert_ne!(Difficulty::Intermediate, Difficulty::Expert);
        assert_ne!(Difficulty::Beginner, Difficulty::Expert);
    }

    #[test]
    fn test_difficulty_clone() {
        let original = Difficulty::Expert;
        let cloned = original;
        assert_eq!(original, cloned);
    }

    #[test]
    fn test_difficulty_debug() {
        let difficulty = Difficulty::Beginner;
        let debug_str = format!("{:?}", difficulty);
        assert_eq!(debug_str, "Beginner");
    }

    #[test]
    fn test_mine_density_calculations() {
        let (w1, h1, m1) = Difficulty::Beginner.get_dimensions();
        let (w2, h2, m2) = Difficulty::Intermediate.get_dimensions();
        let (w3, h3, m3) = Difficulty::Expert.get_dimensions();

        let density1 = m1 as f64 / (w1 * h1) as f64;
        let density2 = m2 as f64 / (w2 * h2) as f64;
        let density3 = m3 as f64 / (w3 * h3) as f64;

        assert!(density1 > 0.10 && density1 < 0.25);
        assert!(density2 > 0.10 && density2 < 0.25);
        assert!(density3 > 0.10 && density3 < 0.25);

        assert!(density1 < density3);
    }
}
