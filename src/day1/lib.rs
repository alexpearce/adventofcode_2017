//! Solutions to day 1.

/// Return the sum of all digits that match the following offset digit in the list.
///
/// Sequences are considered as cyclical across the last element and the first.
///
/// # Arguments
///
/// * `sequence` - A sequence of digits
///
/// # Examples
///
/// ```
/// use day1::sequence_sum;
///
/// // When the match with the following digit should be considered
/// assert_eq!(sequence_sum("1122", 1), 3);
/// assert_eq!(sequence_sum("1111", 1), 4);
/// assert_eq!(sequence_sum("1234", 1), 0);
/// assert_eq!(sequence_sum("91212129", 1), 9);
///
/// // When the match with the digit after len(sequence)/2 should be considered
/// assert_eq!(sequence_sum("1212", 2), 6);
/// assert_eq!(sequence_sum("1221", 2), 0);
/// assert_eq!(sequence_sum("123425", 3), 4);
/// assert_eq!(sequence_sum("123123", 3), 12);
/// assert_eq!(sequence_sum("12131415", 4), 4);
/// ```
///
/// # Alternative implementation
///
/// Using a map-reduce method.
///
/// ```rust,notest
/// sequence.chars()
///         .zip(sequence.chars().cycle().skip(offset))
///         .map(|(a, b)| (a.to_digit(10).unwrap(), b.to_digit(10).unwrap()))
///         .fold(0, |acc, (a, b)| if a == b { acc + a } else { acc })
/// ```
pub fn sequence_sum(sequence: &str, offset: usize) -> u32 {
    let mut sum : u32 = 0;
    // Loop over the sequence in pairs of characters, and increment the sum if the two digits
    // match.
    for (a, b) in sequence.chars().zip(sequence.chars().cycle().skip(offset)) {
        // We expect the input to be well-formed (i.e. containing only integers), so don't bother
        // pattern matching the Result and just unwrap
        let first = a.to_digit(10).unwrap();
        let second = b.to_digit(10).unwrap();
        if first == second {
            sum += first;
        }
    }

    sum
}
