//! Solutions for day 9.

/// Return the total score of a stream of groups and the number of garbage characters.
///
/// # Examples
///
/// ```
/// use aoc17::day9::stream_score;
///
/// assert_eq!(stream_score("{}"), (1, 0));
/// assert_eq!(stream_score("{{{}}}"), (6, 0));
/// assert_eq!(stream_score("{{},{}}"), (5, 0));
/// assert_eq!(stream_score("{{{},{},{{}}}}"), (16, 0));
/// assert_eq!(stream_score("{<a>,<a>,<a>,<a>}"), (1, 4));
/// assert_eq!(stream_score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), (9, 8));
/// assert_eq!(stream_score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), (9, 0));
/// assert_eq!(stream_score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), (3, 17));
///
/// assert_eq!(stream_score("<>"), (0, 0));
/// assert_eq!(stream_score("<random characters>"), (0, 17));
/// assert_eq!(stream_score("<<<<>"), (0, 3));
/// assert_eq!(stream_score("<{!>}>"), (0, 2));
/// assert_eq!(stream_score("<!!>"), (0, 0));
/// assert_eq!(stream_score("<!!!>>"), (0, 0));
/// assert_eq!(stream_score("<{o\"i!a,<{i<a>"), (0, 10));
/// ```
pub fn stream_score(s : &str) -> (usize, usize) {
    let group_open = '{';
    let group_close = '}';
    let garbage_open = '<';
    let garbage_close = '>';
    let ignore_next = '!';

    // The score of the sum of all group scores
    let mut tot_score = 0;
    // The score of the outer, containing group
    let mut outer_score = 0;
    // The number of characters inside garbage groups, excluding the ignore_next character and any
    // single character after that
    let mut ngarbage_chars = 0;
    // Was the previous character the ignore character?
    let mut skip = false;
    // Are we inside a garbage group?
    let mut inside_garbage = false;

    for c in s.chars() {
        if skip {
            skip = false;
            continue;
        }
        if c == ignore_next {
            skip = true;
            continue;
        }
        if inside_garbage {
            if c == garbage_close {
                inside_garbage = false;
            } else {
                ngarbage_chars += 1;
            }
            continue
        } else if c == garbage_open {
            inside_garbage = true;
            continue;
        }
        if c == group_open {
            outer_score += 1;
            tot_score += outer_score;
        } else if c == group_close {
            outer_score -= 1;
        }
    }

    (tot_score, ngarbage_chars)
}
