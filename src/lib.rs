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

#[cfg(test)]
mod tests {
    use crate::PrimeSeive;

    #[test]
    fn first_ten_primes() {
        let primes: Vec<usize> = PrimeSeive::new().take(10).collect();

        assert_eq!(primes, vec!(2, 3, 5, 7, 11, 13, 17, 19, 23, 29));
    }
}
