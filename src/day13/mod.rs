//! Solutions for day 13.

/// Return a `Vec` representation of the `firewall`.
///
/// # Examples
///
/// ```
/// use aoc17::day13::create_firewall;
///
/// assert_eq!(create_firewall("0: 3
/// 1: 2
/// 4: 4
/// 6: 4"), vec![(0, 3), (1, 2), (4, 4), (6, 4)]);
/// ```
pub fn create_firewall(firewall : &str) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    for line in firewall.lines() {
        let mut row : Vec<_> = line.split(": ")
                                   .map(|x| x.parse::<usize>().unwrap())
                                   .collect();
        let length = row.pop().unwrap();
        let idx = row.pop().unwrap();
        ret.push((idx, length));
    }

    ret
}

/// Return the severity score of packet transmission for the given `firewall`.
///
/// The packet starts it journey through the firewall after a `delay` time units.
///
/// # Examples
///
/// ```
/// use aoc17::day13::{create_firewall, severity};
///
/// assert_eq!(severity(&create_firewall("0: 3
/// 1: 2
/// 4: 4
/// 6: 4"), 0), 24);
///
/// assert_eq!(severity(&create_firewall("0: 3
/// 1: 2
/// 4: 4
/// 6: 4"), 10), 0);
/// ```
pub fn severity(firewall : &Vec<(usize, usize)>, delay : usize) -> usize {
    firewall.iter().map(|&(idx, length)| {
        if ((idx + delay) % (2*(length - 1))) == 0 {
            idx*length
        } else {
            0
        }
    }).sum()
}

/// Return true if a packet transmission passes the given `firewall`.
///
/// The packet starts it journey through the firewall after a `delay` time units.
///
/// # Examples
///
/// ```
/// use aoc17::day13::{create_firewall, transmitted};
///
/// assert_eq!(transmitted(&create_firewall("0: 3
/// 1: 2
/// 4: 4
/// 6: 4"), 0), false);
///
/// assert_eq!(transmitted(&create_firewall("0: 3
/// 1: 2
/// 4: 4
/// 6: 4"), 10), true);
/// ```
pub fn transmitted(firewall : &Vec<(usize, usize)>, delay : usize) -> bool {
    !firewall.iter().any(|&(idx, length)| {
        (idx + delay) % (2*(length - 1)) == 0
    })
}
