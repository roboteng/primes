#![feature(test)]

extern crate test;

use fastdiv::{FastDiv, PrecomputedDivU32};
use std::iter::Peekable;

struct NSeive {
    n: usize,
    count: usize,
}

impl NSeive {
    fn new(n: usize) -> NSeive {
        NSeive { n, count: 0 }
    }
}

impl Iterator for NSeive {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        self.count += self.n;
        Some(self.count)
    }
}

pub struct PrimeSeive {
    index: usize,
    seives: Vec<Peekable<NSeive>>,
}

impl PrimeSeive {
    pub fn new() -> PrimeSeive {
        PrimeSeive {
            index: 1,
            seives: vec![],
        }
    }
}

impl Iterator for PrimeSeive {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        'inc: loop {
            self.index += 1;
            's: for seive in &mut self.seives {
                while let Some(next_val) = seive.peek() {
                    match next_val.cmp(&self.index) {
                        std::cmp::Ordering::Less => {
                            seive.next();
                        }
                        std::cmp::Ordering::Equal => {
                            continue 'inc;
                        }
                        std::cmp::Ordering::Greater => {
                            continue 's;
                        }
                    }
                }
            }

            self.seives.push(NSeive::new(self.index).peekable());
            return Some(self.index);
        }
    }
}

pub struct IntSeive {
    index: usize,
    primes: Vec<usize>,
}

impl IntSeive {
    pub fn new() -> IntSeive {
        IntSeive {
            index: 1,
            primes: vec![],
        }
    }
}

impl Iterator for IntSeive {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        'inc: loop {
            self.index += 1;
            for prime in &mut self.primes {
                if self.index % *prime == 0 {
                    continue 'inc;
                }
            }

            self.primes.push(self.index);
            return Some(self.index);
        }
    }
}

pub struct FastSeive {
    index: u32,
    primes: Vec<PrecomputedDivU32>,
}

impl FastSeive {
    pub fn new() -> FastSeive {
        FastSeive {
            index: 1,
            primes: vec![],
        }
    }
}

impl Iterator for FastSeive {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        'inc: loop {
            self.index += 1;
            for prime in &mut self.primes {
                if self.index.is_multiple_of(*prime) {
                    continue 'inc;
                }
            }

            self.primes.push(self.index.precompute_div());
            return Some(self.index);
        }
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use crate::{FastSeive, IntSeive, PrimeSeive};

    #[test]
    fn first_ten_primes() {
        let primes: Vec<usize> = PrimeSeive::new().take(10).collect();

        assert_eq!(primes, vec!(2, 3, 5, 7, 11, 13, 17, 19, 23, 29));
    }

    #[test]
    fn first_ten_primes_int() {
        let primes: Vec<usize> = IntSeive::new().take(10).collect();

        assert_eq!(primes, vec!(2, 3, 5, 7, 11, 13, 17, 19, 23, 29));
    }

    #[test]
    fn first_ten_primes_fast() {
        let mut vec: Vec<u32> = vec![];
        for p in FastSeive::new().take(10) {
            vec.push(p);
        }

        assert_eq!(vec, vec!(2, 3, 5, 7, 11, 13, 17, 19, 23, 29));
    }

    #[bench]
    fn internal_structs(b: &mut Bencher) {
        b.iter(|| {
            let s = PrimeSeive::new();
            for _ in s.take(100) {}
        })
    }

    #[bench]
    fn internal_int(b: &mut Bencher) {
        b.iter(|| {
            let s = IntSeive::new();
            for _ in s.take(100) {}
        })
    }

    #[bench]
    fn internal_fast(b: &mut Bencher) {
        b.iter(|| {
            let s = FastSeive::new();
            for _ in s.take(100) {}
        })
    }
}
