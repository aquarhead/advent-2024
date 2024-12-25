use itertools::Itertools;

pub fn solve(input: &str) -> (usize, usize) {
  let mut locks = Vec::new();
  let mut keys = Vec::new();
  let mut space = 0;
  for key_or_lock in input.split("\n\n") {
    let mut is_lock = true;
    let mut idx = 0;
    key_or_lock.lines().enumerate().for_each(|(row, line)| {
      line.char_indices().for_each(|(col, ch)| {
        if row == 0 && col == 0 {
          if ch == '.' {
            is_lock = false;
            keys.push(vec![0; line.len()]);
          } else {
            locks.push(vec![0; line.len()]);
          }
        }

        if is_lock && ch == '#' {
          idx = locks.len() - 1;
          let l = locks.get_mut(idx).unwrap();
          let ll = l.get_mut(col).unwrap();
          *ll = row as isize;
        }

        if !is_lock && ch == '.' {
          idx = keys.len() - 1;
          let l = keys.get_mut(idx).unwrap();
          let ll = l.get_mut(col).unwrap();
          *ll = row as isize;
        }
      });
    });
    space = key_or_lock.lines().count() as isize;
  }

  let p1 = keys
    .into_iter()
    .cartesian_product(locks.into_iter())
    .filter(|(k, l)| (0..5).all(|n| l[n] + space - k[n] <= space))
    .count();

  (p1, 0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    assert_eq!(3, solve(input.trim()).0);
  }
}
