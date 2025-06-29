use crate::bitset::BitSet;

pub struct ShiftedSieve {
    offsets: Vec<usize>,
    primes: Vec<usize>    
}

pub enum SieveLimit {
    // just limit search to a specific number
    Int(usize),
    // limit search to the max value where it can only find primes
    // i.e the largest prime in the list squared
    ManyPrimes,
    // limit search to always find at least one prime
    // i.e twice the largest prime in the list
    // TODO: use better results on prime gaps
    WithPrime,
    // limit search to cut off just before the next prime
    // note: very inefficient but it's cool so...
    NextPrime
}

impl ShiftedSieve {
    pub fn new_unshifted(primes: Vec<usize>) -> ShiftedSieve {
        ShiftedSieve {
            offsets: vec![0; primes.len()],
            primes
        }
    }

    pub fn new(primes: Vec<usize>, offsets: Vec<usize>) -> ShiftedSieve {
        ShiftedSieve { offsets, primes }
    }
}

impl ShiftedSieve {
    pub fn find_unset(&self, elimit: SieveLimit) -> Option<Vec<usize>> {
        let limit = match elimit {
            SieveLimit::Int(i) => i,
            SieveLimit::ManyPrimes => self.primes.last()? * self.primes.last()?,
            SieveLimit::WithPrime => self.primes.last()? * 2,
            SieveLimit::NextPrime => ShiftedSieve::new_unshifted(self.primes.clone()).find_unset(SieveLimit::WithPrime)?[0],
        };

        let mut bitset = BitSet::with_bits(limit);
        bitset.set(0);
        bitset.set(1);

        for (prime, offset) in self.primes.iter().zip(self.offsets.iter()) {
            let mut cur = offset % prime;

            while cur < limit {
                bitset.set(cur);
                cur += prime;
            }
        }

        return Some(bitset.find_all_unset());
    }
}