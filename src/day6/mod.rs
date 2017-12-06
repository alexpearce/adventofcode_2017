//! Solutions for day 6

use std::collections::HashMap;

/// Return memory re-allocation information on `s`.
///
/// Two pieces of information are returned:
///
/// 1. The total number of iterations taken until a state is repeated;
/// 2. The number if internal iterations taken from the repeated state first occuring to it being
///    seen again.
///
/// # Examples
///
/// ```
/// use aoc17::day6::iterations_until_cycle;
///
/// let (niterations, ninner) = iterations_until_cycle("0 2 7 0");
/// assert_eq!(niterations, 5);
/// assert_eq!(ninner, 4);
/// ```
pub fn iterations_until_cycle(s : &str) -> (usize, usize) {
    let mut banks : Vec<_> = s.split_whitespace()
                              .map(|x| x.parse::<usize>().unwrap())
                              .collect();
    let nbanks = banks.len();
    let mut hm = HashMap::new();
    let mut inner = 0;
    loop {
        // Find the index of the largest element.
        // Takes the index of the first element in case of multiple elements having the largest
        // value.
        let mut max_index = 0;
        let mut max = banks[max_index];
        for (i, v) in banks.iter().enumerate().skip(1) {
            if *v > max {
                max = *v;
                max_index = i;
            }
        }

        // Redistribute the memory.
        let mut memory = banks[max_index];
        banks[max_index] -= memory;
        for i in (0..nbanks).cycle().skip(max_index + 1) {
            banks[i] += 1;
            memory -= 1;
            if memory == 0 {
                break;
            }
        }

        // Insert into the hash map:
        //   1. The memory layout of this iteration; and
        //   2. The iteration number.
        // If the memory layout is already in the map, `HashMap::insert` will return the previous
        // value, i.e. the iteration number at which we first saw the current memory layout.
        let nentries = hm.len();
        if let Some(old) = hm.insert(banks.clone(), nentries) {
            inner = nentries - old;
            break;
        }
    }

    (hm.len() + 1, inner)
}
