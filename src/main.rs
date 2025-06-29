use sieve::{brute::get_working_offsets, shifted_sieve::{ShiftedSieve, SieveLimit}};

fn main() {
    // Generate 4 million primes
    let mut primes = vec![2];

    for _ in 0..5 {
        let sieve = ShiftedSieve::new_unshifted(primes.clone());
        primes.extend_from_slice(&sieve.find_unset(SieveLimit::ManyPrimes).unwrap());
    }

    for i in 1..7 {
        let working = get_working_offsets(primes[..i].to_vec());

        for offsets in working.iter() {
            println!("{offsets:?}");
        }

        println!("{}\n", working.len());
    }
}
