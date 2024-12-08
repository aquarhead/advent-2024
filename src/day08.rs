use itertools::Itertools;
use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);

struct Resonant {
  p: Pos,
  row_step: i32,
  col_step: i32,
  row_max: i32,
  col_max: i32,
}

impl Resonant {
  fn create(base: Pos, other: Pos, row_max: i32, col_max: i32) -> Self {
    Self {
      p: base,
      row_step: base.0 - other.0,
      col_step: base.1 - other.1,
      row_max,
      col_max,
    }
  }
}

impl Iterator for Resonant {
  type Item = Pos;

  fn next(&mut self) -> Option<Self::Item> {
    (self.p.0 >= 0 && self.p.0 <= self.row_max && self.p.1 >= 0 && self.p.1 <= self.col_max).then(|| {
      let ret = self.p;
      self.p.0 += self.row_step;
      self.p.1 += self.col_step;
      ret
    })
  }
}

pub fn solve(input: &str) -> (usize, usize) {
  let mut map: HashMap<char, Vec<_>> = HashMap::new();
  input.lines().enumerate().for_each(|(row, line)| {
    line.trim().char_indices().for_each(|(col, ch)| {
      if ch != '.' {
        map.entry(ch).or_default().push((row as i32, col as i32));
      }
    });
  });

  let row_max = input.lines().enumerate().last().unwrap().0 as i32;
  let col_max = input.lines().next().unwrap().chars().enumerate().last().unwrap().0 as i32;

  let mut p1 = HashSet::new();

  for ants in map.values() {
    ants
      .iter()
      .copied()
      .combinations(2)
      .flat_map(|aa| {
        [
          Resonant::create(aa[0], aa[1], row_max, col_max).nth(1),
          Resonant::create(aa[1], aa[0], row_max, col_max).nth(1),
        ]
      })
      .filter(Option::is_some)
      .for_each(|p| {
        p1.insert(p);
      });
  }

  let mut p2 = HashSet::new();
  for ants in map.values() {
    ants
      .iter()
      .copied()
      .combinations(2)
      .flat_map(|aa| {
        [
          Resonant::create(aa[0], aa[1], row_max, col_max),
          Resonant::create(aa[1], aa[0], row_max, col_max),
        ]
      })
      .flatten()
      .for_each(|p| {
        p2.insert(p);
      });
  }

  (p1.len(), p2.len())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    assert_eq!((14, 34), solve(input.trim()));
  }
}
