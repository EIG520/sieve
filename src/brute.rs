use crate::shifted_sieve::{ShiftedSieve, SieveLimit};

pub fn get_working_offsets(primes: Vec<usize>) -> Vec<Vec<usize>>{
    println!("prime set: {primes:?}");

    let mut offsets = vec![0; primes.len()];
    let nextprime = ShiftedSieve::new_unshifted(primes.clone()).find_unset(SieveLimit::WithPrime).unwrap()[0];
    let mut valids: Vec<Vec<usize>> = vec![];

    loop {
        let sieve = ShiftedSieve::new(primes.clone(), offsets.clone());
        if sieve.find_unset(SieveLimit::Int(nextprime)).unwrap().is_empty() {
            valids.push(offsets.clone());
        }

        offsets[0] += 1;
        
        for i in 0..offsets.len() {
            if offsets[i] >= primes[i] {
                offsets[i] = 0;
                
                if i < offsets.len() - 1 {
                    offsets[i + 1] += 1;
                } else {
                    return valids;
                }
            }
        }
    }
}