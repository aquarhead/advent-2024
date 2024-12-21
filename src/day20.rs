use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);

pub fn solve(input: &str) -> (usize, usize) {
  let mut track = HashSet::new();
  let mut start: Pos = (0, 0);
  let mut end: Pos = (0, 0);
  input.lines().enumerate().for_each(|(row, line)| {
    line.char_indices().for_each(|(col, ch)| {
      //
      let p: Pos = (row as i32, col as i32);
      if ch != '#' {
        track.insert(p);
      }
      if ch == 'S' {
        start = p;
      } else if ch == 'E' {
        end = p;
      }
    });
  });

  let mut time = HashMap::from([(start, 0)]);
  track.remove(&start);
  let mut t: u32 = 0;
  let mut p = start;
  while p != end {
    t += 1;
    for m in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
      let np: Pos = (p.0 + m.0, p.1 + m.1);
      if track.remove(&np) {
        time.insert(np, t);
        p = np;
        break;
      }
    }
  }

  let p1 = time
    .iter()
    .combinations(2)
    .filter(|v| {
      let (p1, t1) = v[0];
      let (p2, t2) = v[1];

      (p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1) == 2) && (t1.abs_diff(*t2) >= 102)
    })
    .count();

  let p2 = time
    .iter()
    .combinations(2)
    .filter(|v| {
      let (p1, t1) = v[0];
      let (p2, t2) = v[1];
      let path = p1.0.abs_diff(p2.0) + p1.1.abs_diff(p2.1);

      (path <= 20) && (t1.abs_diff(*t2) >= (100 + path))
    })
    .count();

  (p1, p2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    assert_eq!((0, 0), solve(input.trim()));
  }
}
