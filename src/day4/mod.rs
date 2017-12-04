use std::collections::HashMap;

/// Return true if the input has unique substrings.
///
/// The algorithm works by constructing a hash table from a substring to a boolean. If the
/// substring is already in the hash table, the substring is not unique.
///
/// We use a hash table as it has O(1) lookup time, and we need to perform one lookup per
/// substring, so we want it to be indenpendent of the number of substrings.
///
/// # Examples
///
/// ```
/// use aoc17::day4::has_unique_substrings;
///
/// assert_eq!(has_unique_substrings("aa bb cc dd ee"), true);
/// assert_eq!(has_unique_substrings("aa bb cc dd aa"), false);
/// assert_eq!(has_unique_substrings("aa bb cc dd aaa"), true);
/// ```
pub fn has_unique_substrings(s : &str) -> bool {
    let subs : Vec<&str> = s.split(" ").collect();
    let mut map : HashMap<&str, bool> = HashMap::new();
    for sub in subs.iter() {
        let counter = map.entry(sub).or_insert(false);
        if *counter == true {
            // We've already inserted this substring in to the map
            return false
        }
        *counter = true;
    }

    true
}

/// Return true if the input contains no substrings that are anagrams of each other.
///
/// The implementation is very similar to `has_unique_substrings`, except we somehow need to know
/// whether an 'anagram' has been inserted into the hash table already. We exploit the fact that
/// two strings that are anagrams of each other will have identical _sorted_ substrings, and so insert
/// sorted strings into the hash table instead of the substrings themselves.
///
/// # Examples
///
/// ```
/// use aoc17::day4::has_no_anagram_substrings;
///
/// assert_eq!(has_no_anagram_substrings("abcde fghij"), true);
/// assert_eq!(has_no_anagram_substrings("abcde xyz ecdab"), false);
/// assert_eq!(has_no_anagram_substrings("a ab abc abd abf abj"), true);
/// assert_eq!(has_no_anagram_substrings("iiii oiii ooii oooi oooo"), true);
/// assert_eq!(has_no_anagram_substrings("oiii ioii iioi iiio"), false);
/// ```
pub fn has_no_anagram_substrings(s : &str) -> bool {
    let subs : Vec<&str> = s.split(" ").collect();
    let mut map : HashMap<String, bool> = HashMap::new();
    for sub in subs.iter() {
        // This is just like has_unique_substrings, except we exploit the fact that two sequences
        // are 'anagrams' of each other if the sorted sequences are identical.
        let mut as_vec = sub.chars().collect::<Vec<_>>();
        as_vec.sort();
        let sorted : String = as_vec.into_iter().collect();
        let counter = map.entry(sorted).or_insert(false);
        if *counter == true {
            // We've already inserted this substring in to the map
            return false
        }
        *counter = true;
    }

    true
}
