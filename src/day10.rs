use std::collections::{HashMap, HashSet};

type Pos = (i32, i32);

pub fn solve(input: &str) -> (usize, u32) {
  let mut map = HashMap::new();
  input.lines().enumerate().for_each(|(row, line)| {
    line.trim().char_indices().for_each(|(col, ch)| {
      let p: Pos = (row as i32, col as i32);
      map.insert(p, ch.to_digit(10).unwrap());
    })
  });

  let p1 = map
    .iter()
    .filter_map(|(p, v)| (*v == 0).then(|| find_trails(p, &map).len()))
    .sum();

  let p2 = map
    .iter()
    .filter_map(|(p, v)| (*v == 0).then(|| count_trails(p, &map)))
    .sum();

  (p1, p2)
}

fn find_trails(p: &Pos, map: &HashMap<Pos, u32>) -> HashSet<Pos> {
  match map.get(p) {
    Some(9) => [*p].into(),
    Some(h) => {
      let mut ret = HashSet::new();

      for m in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
        let np = (p.0 + m.0, p.1 + m.1);
        if let Some(v) = map.get(&np) {
          if *v == h + 1 {
            ret.extend(find_trails(&np, map));
          }
        }
      }

      ret
    }
    None => unimplemented!(),
  }
}

fn count_trails(p: &Pos, map: &HashMap<Pos, u32>) -> u32 {
  match map.get(p) {
    Some(9) => 1,
    Some(h) => [(-1, 0), (1, 0), (0, -1), (0, 1)]
      .iter()
      .map(|m| {
        let np = (p.0 + m.0, p.1 + m.1);

        map
          .get(&np)
          .map(|v| if *v == *h + 1 { count_trails(&np, map) } else { 0 })
          .unwrap_or(0)
      })
      .sum(),
    None => unimplemented!(),
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    assert_eq!((36, 81), solve(input.trim()));
  }
}
