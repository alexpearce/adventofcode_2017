//! Solutions for day 8.

use std::collections::HashMap;

/// Operations that return integers
#[derive(Debug)]
enum IntegerOperation {
    Increment,
    Decrement
}

/// Operations that return booleans
#[derive(Debug)]
enum BooleanOperation {
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,
    EqualTo,
    NotEqualTo
}

/// Apply the `IntegerOperation` to `x` and `y`.
fn apply_integer_operation(op : &IntegerOperation, x : isize, y : isize) -> isize {
    match *op {
        IntegerOperation::Increment => x + y,
        IntegerOperation::Decrement => x - y
    }
}

/// Apply the `BooleanOperation` to `x` and `y`.
fn apply_boolean_operation(op : &BooleanOperation, x : isize, y : isize) -> bool {
    match *op {
        BooleanOperation::GreaterThan => x > y,
        BooleanOperation::LessThan => x < y,
        BooleanOperation::GreaterThanOrEqualTo => x >= y,
        BooleanOperation::LessThanOrEqualTo => x <= y,
        BooleanOperation::EqualTo => x == y,
        BooleanOperation::NotEqualTo => x != y
    }
}

/// Return a hash table of registers and their values after running the instruction set.
///
/// Also returns the highest register value reached during operations.
///
/// # Arguments
///
/// * `s` - A string slice of the set of instructions to run.
///
/// Each instruction must be of the form:
///
/// ```text
/// REGISTER_NAME_A OP_A NUM_A if REGISTER_NAME_B OP_B NUM_B
/// ```
///
/// Each term must be valid:
///
/// 1. `REGISTER_NAME_A` - The register name being acted on, e.g. `abc`;
/// 2. `OP_A` - Either `inc` or `dec`;
/// 3. `NUM_A` - The number used as input to `OP_A`;
/// 4. `REGISTER_NAME_B` - The register name being queried;
/// 5. `OP_B` - Either `>`, `<`, `<=`, `>=`, `==`, or `!=`; and
/// 6. `NUM_B` - The number used as input to `OP_B`.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
///
/// use aoc17::day8::registers;
///
/// let mut expected = HashMap::new();
/// expected.insert("a", 1);
/// expected.insert("b", 0);
/// expected.insert("c", -10);
///
/// assert_eq!(registers("b inc 5 if a > 1
/// a inc 1 if b < 5
/// c dec -10 if a >= 1
/// c inc -20 if c == 10"), (expected, 10));
/// ```
pub fn registers(s : &str) -> (HashMap<&str, isize>, isize) {
    let mut registers : HashMap<&str, isize> = HashMap::new();
    let mut largest_seen = 0;
    for instruction in s.lines() {
        let mut components = instruction.split_whitespace().collect::<Vec<_>>();
        let num_b = components.pop().unwrap().parse::<isize>().unwrap();
        let op_b_s = components.pop().unwrap();
        let register_b = components.pop().unwrap();
        // Ignore the 'if'
        components.pop();
        let num_a = components.pop().unwrap().parse::<isize>().unwrap();
        let op_a_s = components.pop().unwrap();
        let register_a = components.pop().unwrap();

        // Convert the operations to enum variants
        let op_a = match op_a_s {
            "inc" => IntegerOperation::Increment,
            "dec" => IntegerOperation::Decrement,
            other => panic!("Could not decode operation {}", other)
        };
        let op_b = match op_b_s {
            ">" => BooleanOperation::GreaterThan,
            "<" => BooleanOperation::LessThan,
            ">=" => BooleanOperation::GreaterThanOrEqualTo,
            "<=" => BooleanOperation::LessThanOrEqualTo,
            "==" => BooleanOperation::EqualTo,
            "!=" => BooleanOperation::NotEqualTo,
            other => panic!("Could not decode operation {}", other)
        };

        let should_update;
        // Need to scope access to `registers` as we possibly need to mutate `register_b`, then use
        // the result to possibly mutate `register_a`; we cannot two mutable references at once.
        {
            let rb = registers.entry(register_b).or_insert(0);
            should_update = apply_boolean_operation(&op_b, *rb, num_b);
        }
        let ra = registers.entry(register_a).or_insert(0);
        if should_update {
            *ra = apply_integer_operation(&op_a, *ra, num_a);
        }

        if *ra > largest_seen {
            largest_seen = *ra;
        }
    }

    (registers, largest_seen)
}
