use bitset::BitSet;

fn collatz_1_step(n: u128) -> u128 {
    if n.is_multiple_of(2) {
        n >> 1
    } else {
        n * 3 + 1
    }
}

struct CollatzChecker {
    cache: BitSet,
    total_steps: u128,
    total_steps_last_time: u128,
}

impl CollatzChecker {
    fn new() -> Self {
        Self {
            cache: BitSet::with_capacity(1_000_000_000),
            total_steps: 0,
            total_steps_last_time: 0,
        }
    }

    fn check_to(mut self, max: u128) {
        for start in 1..=max {
            if start.is_multiple_of(1_000_000) {
                let total_more_steps = self.total_steps - self.total_steps_last_time;
                self.total_steps_last_time = self.total_steps;
                println!("Checking {} M    (steps: {} M, diff: {:.3} M)", start / 1_000_000, self.total_steps / 1_000_000, total_more_steps as f64 / 1_000_000.0);
            }
            let mut n = start;
            let mut steps = 0;
            while n != 1 {
                if n < 1_000_000_000 && self.cache.test(n as usize) {
                    break;
                }
                n = collatz_1_step(n);
                steps += 1;
            }
            self.total_steps += steps;
            if start < 1_000_000_000 {
                self.cache.set(start as usize, true);
            }
        }
    }
}

fn main() {
    CollatzChecker::new().check_to(2_000_000_000);
}
