/// Thank you @maneatingape you are a genius
/// https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2023/day12.rs

// # Hot Springs
//
// A [dynamic programming](https://en.wikipedia.org/wiki/Dynamic_programming) approach calculates
// the possible arrangements for each entry in `O(n * w)` complexity where:
//
// * `n` Number of springs
// * `w` "Wiggle" is the amount of extra free space the springs can slide around in the pattern.
//
// We build a table for each entry with a row for each spring and a column for every character
// in the pattern. Adding a trailing `.` character makes bounds checking easier without changing
// the number of arrangements. The result will be the bottom right value.
//
// Using the sample `?###???????? 3,2,1`:
//
// ```none
//     n = 3
//     w = 13 - (3 + 2 + 1) - 3 + 1 = 5
//
//          ?  #  #  #  ?  ?  ?  ?  ?  ?  ?  ?  .
//       ┌----------------------------------------
//     3 |  0  0  0 [0  1  1  1  1] 0  0  0  0  0
//     2 |  0  0  0  0  0  0 [0  1  2  3  4] 0  0
//     1 |  0  0  0  0  0  0  0  0 [0  1  3  6 10]
// ```
//
// Each pattern updates the total at the index one *after* its end, if it can fit at that location
// For example the first spring can only match at indices `[1, 2, 3]` so it updates the total
// at index 4 to 1.
//
// The key insight is that the number of arrangements is then propagated as a prefix sum
// from left to right for each row as long as the character at the index is not a `#` or until
// `wiggle` characters are reached, whichever comes sooner.
//
// To calculate the next row, each matching pattern adds the value from the row above at the
// index one before its start. The first row is a special case where this value is always 1.
//
// As a nice side effect this approach always overwrites values so we can re-use the memory buffer
// for different entries without having to zero out values.
//
// ## Alternate approach
//
// Another way to look at the problem is to search to the left from each matching position
// until a `#` character is found. The previous pattern can't leave any trailing `#` characters
// otherwise it wouldn't be the previous pattern.
//
// Using the same example `?###???????? 3,2,1` and adding a trailing `.`:
//
// * `###` can only match at one location giving:
//     ```none
//          . # # # . . . . . . . . .
//         [0 0 0 0 1 0 0 0 0 0 0 0 0]
//     ````
//
//* The next `##` can match at 4 possible locations:
//     ```none
//         . # # # . # # . . . . . .
//        [0 0 0 0 1 0 0 0 0 0 0 0 0]
//                 <<
//        [0 0 0 0 0 0 0 1 0 0 0 0 0]
//     ```
// * 2nd location:
//     ```none
//         . # # # . . # # . . . . .
//        [0 0 0 0 1 0 0 0 0 0 0 0 0]
//                 <<<<
//        [0 0 0 0 0 0 0 1 1 0 0 0 0]
//     ```
// * 3rd location:
//     ```none
//         . # # # . . . # # . . . .
//        [0 0 0 0 1 0 0 0 0 0 0 0 0]
//                 <<<<<<
//        [0 0 0 0 0 0 0 1 1 1 0 0 0]
//     ```
// * 4th location:
//     ```none
//         . # # # . . . . # # . . .
//        [0 0 0 0 1 0 0 0 0 0 0 0 0]
//                 <<<<<<<<
//        [0 0 0 0 0 0 0 1 1 1 1 0 0]
//     ```
//* The final `#` can also match at 4 possible locations (for brevity only showing the 2nd pattern
//  in a single position):
//     ```none
//         . # # # . # # . # . . . .
//        [0 0 0 0 1 0 0 0 0 0 0 0 0]
//        [0 0 0 0 0 0 0 1 1 1 1 0 0]
//                       <<
//        [0 0 0 0 0 0 0 0 1 0 0 0 0]
//     ```
// * 2nd location:
//     ```none
//         . # # # . # # . . # . . .
//        [0 0 0 0 1 0 0 0 0 0 0 0 0]
//        [0 0 0 0 0 0 0 1 1 1 1 0 0]
//                       <<<<
//        [0 0 0 0 0 0 0 0 1 2 0 0 0]
//     ```
// * 3rd location:
//     ```none
//         . # # # . # # . . . # . .
//        [0 0 0 0 1 0 0 0 0 0 0 0 0]
//        [0 0 0 0 0 0 0 1 1 1 1 0 0]
//                       <<<<<<
//        [0 0 0 0 0 0 0 0 1 2 3 0 0]
//     ```
// * 4th location:
//     ```none
//         . # # # . # # . . . . # .
//        [0 0 0 0 1 0 0 0 0 0 0 0 0]
//        [0 0 0 0 0 0 0 1 1 1 1 0 0]
//                       <<<<<<<<
//        [0 0 0 0 0 0 0 0 1 2 3 4 0]
//     ```
//
// The final result is then the sum of the bottom row with the nuance that any numbers before the
// last `#` don't count as they represent an invalid pattern.
//
// This is equivalent to the prefix sum approach described above but a little clearer to
// understand however slower to calculate.

type Input<'a> = Vec<(&'a [u8], Vec<usize>)>;

fn main(){
    let input: Input<'_> = include_str!("input")
        .lines()
        .map(|line| {
            let (prefix, suffix) = line.split_once(' ').unwrap();
            let first = prefix.as_bytes();
            let second = suffix
                .split(',')
                .map(|s| s.parse().unwrap())
                .collect::<Vec<_>>();
            (first, second)
        })
        .collect();

    println!("Part 1: {}", solve(&input, 1));
    println!("Part 2: {}", solve(&input, 5));
}

pub fn solve(input: &Input<'_>, repeat: usize) -> u64 {
    let mut result = 0;
    let mut pattern = Vec::new();
    let mut springs = Vec::new();
    // Exact size is not too important as long as there's enough space.
    let mut broken = vec![0; 200];
    let mut table = vec![0; 200 * 50];

    for (first, second) in input {
        // Create input sequence reusing the buffers to minimize memory allocations.
        pattern.clear();
        springs.clear();

        for _ in 1..repeat {
            pattern.extend_from_slice(first);
            pattern.push(b'?');
            springs.extend_from_slice(second);
        }

        // Add a trailing '.' so that we don't have to check bounds when testing the last pattern.
        // This has no effect on the number of possible combinations.
        pattern.extend_from_slice(first);
        pattern.push(b'.');
        springs.extend_from_slice(second);

        // Calculate prefix sum of the number of broken springs and unknowns before each index
        // to quickly check if a range can contain a broken spring without checking every element.
        // For example `.??..??...?##` becomes `[0, 0, 1, 2, 2, 2, 3, 4, 4, 4, 4, 5, 6, 7, 7]`.
        let mut sum = 0;
        broken.push(0);

        for (i, &b) in pattern.iter().enumerate() {
            if b != b'.' {
                sum += 1;
            }
            broken[i + 1] = sum;
        }

        // Determine how many spaces each pattern can slide around to speed things up.
        // We only need to check at most that many spaces for each pattern.
        let wiggle = pattern.len() - springs.iter().sum::<usize>() - springs.len() + 1;

        // Count combinations, handling the first row as a special case.
        let size = springs[0];
        let mut sum = 0;
        let mut valid = true;

        for i in 0..wiggle {
            // In order to be a broken spring, an interval must only contains `#` or `?`
            // characters and not have a '#' character immediately before or after.
            if pattern[i + size] == b'#' {
                sum = 0;
            } else if valid && broken[i + size] - broken[i] == size {
                sum += 1;
            }

            table[i + size] = sum;

            // The first pattern can't have any '#' characters anywhere to its left
            // otherwise it wouldn't be the first pattern.
            valid &= pattern[i] != b'#';
        }

        // Count each subsequent spring. The previous patterns take at least the sum of their size
        // and 1 space afterwards so no need to check indices before that.
        let mut start = size + 1;

        for (row, &size) in springs.iter().enumerate().skip(1) {
            // We're using a 1 dimensional vec to implement a two dimensional table.
            // Calculate the starting index of current and previous row for convenience.
            let previous = (row - 1) * pattern.len();
            let current = row * pattern.len();

            // Reset the running sum.
            sum = 0;

            for i in start..start + wiggle {
                // As a minor optimization only check the pattern if the previous row
                // will contribute a non-zero value.
                if pattern[i + size] == b'#' {
                    sum = 0;
                } else if table[previous + i - 1] > 0
                    && pattern[i - 1] != b'#'
                    && broken[i + size] - broken[i] == size
                {
                    sum += table[previous + i - 1];
                }

                table[current + i + size] = sum;
            }

            start += size + 1;
        }

        // The final value of sum (the bottom right of the table) is the number of possible
        // arrangements of the pattern.
        result += sum;
    }

    result
}