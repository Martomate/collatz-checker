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

const K: usize = 10;
const MOD_K: usize = 24;

const K_MASK: u128 = (1 << K) - 1;
const MOD_K_MASK: u128 = (1 << MOD_K) - 1;

static D: [u128; 1 << K] = {
    let mut d = [0_u128; 1 << K];
    let mut n = 1;
    while n < 1 << K {
        d[n] = collatz_k_steps_simple(n as u128, K as u8);
        n += 1;
    }
    d
};

static C: [u128; 1 << K] = {
    let mut c = [1_u128; 1 << K];
    let mut n = 1;
    while n < 1 << K {
        c[n] = collatz_k_steps_simple((1 << K) + (n as u128), K as u8) - D[n];
        n += 1;
    }
    c
};

const fn collatz_k_steps_coefficients(b: u128, k: u8) -> (u128, u128) {
    let d = collatz_k_steps_simple(b, k);
    let c = collatz_k_steps_simple((1 << k) + b, k) - d;
    (c, d)
}

const fn collatz_k_steps_simple(mut n: u128, k: u8) -> u128 {
    let mut j = 0;
    while j < k {
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

fn eligible_congruence(max_k: u8) -> Vec<u128> {
    let mut mods = vec![0];
    for k in 1..=max_k {
        let half = 1 << (k - 1);
        let mut new_mods = Vec::with_capacity(mods.len() * 2);
        for m in mods {
            for b in [m, m + half] {
                let (c, d) = collatz_k_steps_coefficients(b, k);
                if !(c < (1 << k) && d <= b) {
                    new_mods.push(b);
                }
            }
        }
        mods = new_mods;
    }
    mods.sort();
    mods
}

struct CollatzChecker {
    steps_since_last_print: u128,
}

impl CollatzChecker {
    fn new() -> Self {
        Self {
            steps_since_last_print: 0,
        }
    }

    fn check_to(mut self, max: u128) {
        let eligible_mods = eligible_congruence(MOD_K as u8);
        let eligible_mods_lookup = {
            let mut set = BitSet::with_capacity(1 << MOD_K);
            for &m in eligible_mods.iter() {
                set.set(m as usize, true);
            }
            set
        };

        for bin in 0..=(max >> MOD_K) {
            if bin.is_multiple_of((1_000_000_000 >> MOD_K) + 1) {
                println!(
                    "Checking {} B    (step ratio: {:.3} %)",
                    (bin << MOD_K) / 1_000_000_000,
                    self.steps_since_last_print as f64 * 100.0 / 1_000_000_000.0
                );
                self.steps_since_last_print = 0;
            }

            let mut steps = 0;
            for &m in eligible_mods.iter() {
                let start = (bin << MOD_K) ^ m;

                let mut n = start;
                while n > 4 {
                    // we're not yet in the trivial cycle

                    n = collatz_k_steps(n);
                    steps += 1;

                    if n < start {
                        // we have already checked the numbers below and they are not counter-examples
                        break;
                    }
                    if !eligible_mods_lookup.test((n & MOD_K_MASK) as usize) {
                        break;
                    }
                }
            }
            self.steps_since_last_print += steps;
        }
    }
}

fn main() {
    CollatzChecker::new().check_to(1_000_000_000_000);
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

With k=10 steps at a time and restrictions mod 2^10:
20s

With k=10 steps at a time and restrictions mod 2^20:
17s

With k=10 steps at a time and restrictions mod 2^24 and super-cache:
0.75s

================================================================================

## 1 - 1000B

With k=10 steps at a time and restrictions mod 2^24 and super-cache:
62.13s

With some more optimizations:
51.37s

*/

#[cfg(test)]
mod tests {
    use super::eligible_congruence;

    #[test]
    fn eligible_congruence_1() {
        assert_eq!(eligible_congruence(1), vec![1]);
    }

    #[test]
    fn eligible_congruence_2() {
        assert_eq!(eligible_congruence(2), vec![3]);
    }

    #[test]
    fn eligible_congruence_3() {
        assert_eq!(eligible_congruence(3), vec![3, 7]);

        /*

        d(3, 3):
            3
            (3*3+1) / 2 = 5
            (3*5+1) / 2 = 8
            4
        c(3, 3):
            8+3 = 11
            (3*11+1) / 2 = 17
            (3*17+1) / 2 = 26
            13
            -> 13 - 4 = 9 = 3^2
        3:
            f^3(8n + 3) = 9n + 4 !< 8n + 3

        d(7, 3):
            7
            (3*7+1) / 2 = 11
            (3*11+1) / 2 = 17
            (3*17+1) / 2 = 26
        c(7, 3):
            8+7 = 15
            (3*15+1) / 2 = 23
            (3*23+1) / 2 = 35
            (3*35+1) / 2 = 53
            -> 53 - 26 = 27 = 3^3
        7:
            f^3(8n + 3) = 27n + 26 ?< 8n + 3

         */
    }

    #[test]
    fn eligible_congruence_4() {
        assert_eq!(eligible_congruence(4), vec![7, 11, 15]);

        /*

        d(3, 4):
            3
            (3*3+1) / 2 = 5
            (3*5+1) / 2 = 8
            4
            2
        c(3, 4):
            16+3 = 19
            (3*19+1) / 2 = 29
            (3*29+1) / 2 = 44
            22
            11
            -> 11 - 2 = 9 = 3^2
        3:
            f^4(16n + 3) = 9n + 2 < 16n + 3 => 3 mod 16 is ruled out

         */
    }

    #[test]
    fn eligible_congruence_5() {
        assert_eq!(eligible_congruence(5), vec![7, 15, 27, 31]);

        // Based on Wikipedia
    }
}
