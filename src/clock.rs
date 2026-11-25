/// Discrete simulation time.
///
/// Independent of rendering and real-world time.
/// Time complexity: O(1) per advancement.
/// Space complexity: O(1).
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct SimulationClock {
    tick: u64,
}

impl SimulationClock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }

    pub fn advance(&mut self) -> Option<u64> {
        self.tick = self.tick.checked_add(1)?;
        Some(self.tick)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clock_starts_at_zero() {
        let clock = SimulationClock::new();

        assert_eq!(clock.tick(), 0);
    }

    #[test]
    fn clock_advances_sequentially() {
        let mut clock = SimulationClock::new();

        for expected in 1..=1000 {
            assert_eq!(clock.advance(), Some(expected));
            assert_eq!(clock.tick(), expected);
        }
    }

    #[test]
    fn clock_is_deterministic() {
        let mut first = SimulationClock::new();
        let mut second = SimulationClock::new();

        for _ in 0..100 {
            first.advance().unwrap();
            second.advance().unwrap();
        }

        assert_eq!(first, second);
    }

    #[test]
    fn clock_cannot_overflow() {
        let mut clock = SimulationClock { tick: u64::MAX };

        assert_eq!(clock.advance(), None);
        assert_eq!(clock.tick(), u64::MAX);
    }
}
