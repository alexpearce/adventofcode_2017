//! Solutions for day 5

/// Return the number of jumps it takes to exit the instruction set encoded in the string
///
/// The `offset_rule` defines how the instruction that was just ran should be mutated for the next
/// executation, given the value of that instruction.
///
/// # Examples
///
/// The `offset_rule` increments the just-run instruction by 1.
///
/// ```
/// use aoc17::day5::jumps_until_end;
///
/// assert_eq!(jumps_until_end("0
/// 3
/// 0
/// 1
/// -3", |jump| 1), 5);
/// ```
///
/// The `offset_rule` increments the just-run instruction by 1 if its value was less than 3, else
/// it decrements it by 1.
///
/// ```
/// use aoc17::day5::jumps_until_end;
///
/// let offset_rule = |jump : isize| if jump < 3 { 1 } else { -1 };
/// assert_eq!(jumps_until_end("0
/// 3
/// 0
/// 1
/// -3", offset_rule), 10);
/// ```
pub fn jumps_until_end(s : &str, offset_rule : fn(isize) -> isize) -> usize {
    let instructions : Vec<_> = s.lines().map(|x| x.parse::<isize>().unwrap()).collect();
    let ninstructions = instructions.len();

    let mut position : usize = 0;
    let mut offsets : Vec<isize> = vec![0; ninstructions];
    let mut nexecuted : usize = 0;
    while position < ninstructions {
        let instruction = instructions[position];
        let offset = offsets[position];
        let jump = instruction + offset;
        offsets[position] += offset_rule(jump);
        position = ((position as isize) + jump) as usize;
        nexecuted += 1;
    }

    nexecuted
}
