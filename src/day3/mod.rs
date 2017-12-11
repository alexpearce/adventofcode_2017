//! Solutions to day 3.

use std::collections::HashMap;
use std::iter;
use std::ops::Add;

#[derive(Debug,Hash,PartialEq,Eq,Copy,Clone)]
struct Coordinate {
    x : isize,
    y : isize
}

impl<'a, 'b> Add<&'b Coordinate> for &'a Coordinate {
    type Output = Coordinate;

    fn add(self, other : &'b Coordinate) -> Coordinate {
        Coordinate {
            x: self.x + other.x,
            y: self.y + other.y
        }
    }
}

/// Return the (x, y) coordinates of the number in a spiral grid.
///
/// The spiral pattern looks like this:
///
/// ```text
/// 17  16  15  14  13
/// 18   5   4   3  12
/// 19   6   1   2  11
/// 20   7   8   9  10
/// 21  22  23---> ...
/// ```
///
/// We take the number 1 to be at coordinate (0, 0), with the x-axis increasing to the right and
/// the y-axis increasing upwards.
///
/// The implementation here proceeds as follows:
///
/// 0. Label each 'ring' of numbers with an index $n$. Notice that the lower-right number in each
///    ring is equal to $(2n - 1)^{3}$. The sides of the $n$th ring have width $2n - 1$.
/// 1. For the input `num`, find the ring $n$ it lies in. The coordinates of the lower-right corner
///    of the ring are $(n - 1, -(n - 1))$.
/// 2. Compute the number at and coordinates of the other three vertices of this ring.
/// 3. Find out which two vertices `num` lies between, and then using the two vertex coordinates,
///    compute the coordinates of `num`.
///
/// I don't think this is optimal.
///
/// # Examples
///
/// ```
/// use aoc17::day3::spiral_coordinates;
///
/// assert_eq!(spiral_coordinates(1), (0, 0));
/// assert_eq!(spiral_coordinates(12), (2, 1));
/// assert_eq!(spiral_coordinates(21), (-2, -2));
/// ```
pub fn spiral_coordinates(num : isize) -> (isize, isize) {
    assert!(num > 0);

    let mut n : isize = 1;
    let mut bottom_right : isize;
    let mut ndigits_per_side : isize;
    loop {
        ndigits_per_side = 2*n - 1;
        bottom_right = ndigits_per_side*ndigits_per_side;
        if num <= bottom_right {
            break;
        }
        n += 1;
    }
    let mut coord = Coordinate { x: n - 1,  y: -(n - 1) };
    let bottom_left = bottom_right - ndigits_per_side + 1;
    let top_left = bottom_left - ndigits_per_side + 1;
    let top_right = top_left - ndigits_per_side + 1;

    if (bottom_left < num) && (num <= bottom_right) {
        coord.x -= bottom_right - num;
        return (coord.x, coord.y);
    } else {
        coord.x -= bottom_right - bottom_left;
    }

    if (top_left < num) && (num <= bottom_left) {
        coord.y += bottom_left - num;
        return (coord.x, coord.y);
    } else {
        coord.y += bottom_left - top_left;
    }

    if (top_right < num) && (num <= top_left) {
        coord.x += top_left - num;
        return (coord.x, coord.y);
    } else {
        coord.x += top_left - top_right;
        coord.y -= top_right - num;
    }

    (coord.x, coord.y)
}


/// Return the first number in the spiral that is larger than the argument.
///
/// The "spiral" here is different to that defined in `spiral_coordinates`. Rather than each entry
/// being an increment of the previous, each entry is the sum of all surrounding entries. The
/// initial entry, at $(0, 0)$, is defined to have a value of one. Then, the first few elements of
/// the spiral are:
///
/// ```text
/// 147  142  133  122   59
/// 304    5    4    2   57
/// 330   10    1    1   54
/// 351   11   23   25   26
/// 362  747  806--->   ...
/// ```
///
/// # Examples
///
/// ```
/// use aoc17::day3::first_spiral_number_larger_than;
///
/// assert_eq!(first_spiral_number_larger_than(6), 10);
/// // assert_eq!(first_spiral_number_larger_than(750), 806);
/// ```
pub fn first_spiral_number_larger_than(num : isize) -> isize {
    assert!(num > 0);

    let mut d = HashMap::new();
    let mut n = 1;

    let surrounding_coordinates = [
        // Top row
        Coordinate { x: -1, y: 1 },
        Coordinate { x: 0, y: 1 },
        Coordinate { x: 1, y: 1 },
        // Middle row
        Coordinate { x: -1, y: 0 },
        Coordinate { x: 1, y: 0 },
        // Bottom row
        Coordinate { x: -1, y: -1 },
        Coordinate { x: 0, y: -1 },
        Coordinate { x: 1, y: -1 }
    ];

    // Do the first ring by hand
    d.insert(Coordinate { x: 0, y: 0 }, 1);
    n += 1;
    if num < 1 {
        return 1;
    }

    let mut current_coordinate = Coordinate { x: 0, y: 0 };
    loop {
        // Loop around the current ring
        let moves = iter::repeat((1, 0)).take(1)
             .chain(iter::repeat((0, 1)).take(2*n - 3))
             .chain(iter::repeat((-1, 0)).take(2*n - 2))
             .chain(iter::repeat((0, -1)).take(2*n - 2))
             .chain(iter::repeat((1, 0)).take(2*n - 2));
        for (x, y) in moves {
            let mv = Coordinate { x, y };
            current_coordinate = &current_coordinate + &mv;
            let mut val = 0;
            for neighbour_offset in surrounding_coordinates.iter() {
                let neighbour = &current_coordinate + neighbour_offset;
                val += d.get(&neighbour).unwrap_or(&0);
            }
            d.insert(current_coordinate.clone(), val);
            if num < val {
                return val;
            }
        }
        n += 1;
    }
}
