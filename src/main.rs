use bitset::BitSet;

const fn collatz_1_step(n: u128) -> u128 {
    if n.is_multiple_of(2) {
        n >> 1
    } else {
        n * 3 + 1
    }
}

const fn collatz_1_step_shortcut(n: u128) -> u128 {
    if n.is_multiple_of(2) {
        n >> 1
    } else {
        (n * 3 + 1) >> 1
    }
}

const K: usize = 15;

const K_MASK: u128 = (1 << K) - 1;

static D: [u128; 1 << K] = {
    let mut d = [0_u128; 1 << K];
    let mut n = 1;
    while n < 1 << K {
        d[n] = collatz_k_steps_simple(n as u128);
        n += 1;
    }
    d
};

static C: [u128; 1 << K] = {
    let mut c = [1_u128; 1 << K];
    let mut n = 1;
    while n < 1 << K {
        c[n] = collatz_k_steps_simple((1 << K) + (n as u128)) - D[n];
        n += 1;
    }
    c
};

const fn collatz_k_steps_simple(mut n: u128) -> u128 {
    let mut j = 0;
    while j < K {
        n = collatz_1_step_shortcut(n);
        j += 1;
    }
    n
}

fn collatz_k_steps(n: u128) -> u128 {
    let a = n >> K;
    let b = (n & K_MASK) as usize;
    C[b] * a + D[b]
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
                if n < 5 {
                    n = collatz_1_step_shortcut(n);
                } else {
                    n = collatz_k_steps(n);
                }
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
    CollatzChecker::new().check_to(10_000_000_000);
}

/*

## 1 - 10B

Without shortcut:
7m

With shortcut:
5m4s

With k=5 steps at a time:
2m37s

With k=10 steps at a time:
1m46s

With k=15 steps at a time:
1m36s

*/
