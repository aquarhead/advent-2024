use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::iter::successors;

const PRUNE_MASK: isize = 16777216 - 1;

pub fn solve(input: &str) -> (isize, isize) {
  let secrets: Vec<isize> = input.lines().map(|line| line.parse().unwrap()).collect();
  let p1 = secrets
    .clone()
    .into_par_iter()
    .map(|secret| {
      successors(Some(secret), |s| {
        let mut s = *s;
        s ^= s << 6;
        s &= PRUNE_MASK;
        s ^= s >> 5;
        s &= PRUNE_MASK;
        s ^= s << 11;
        s &= PRUNE_MASK;
        Some(s)
      })
      .nth(2000)
      .unwrap()
    })
    .sum();

  let all_bananas: Vec<_> = secrets
    .into_par_iter()
    .map(|secret| {
      let mut bananas = HashMap::new();
      successors(Some(secret), |s| {
        let mut s = *s;
        s ^= s << 6;
        s &= PRUNE_MASK;
        s ^= s >> 5;
        s &= PRUNE_MASK;
        s ^= s << 11;
        s &= PRUNE_MASK;
        Some(s)
      })
      .take(2001)
      .map(|r| r % 10)
      .tuple_windows::<(_, _, _, _, _)>()
      .for_each(|prices| {
        let change = (
          prices.1 - prices.0,
          prices.2 - prices.1,
          prices.3 - prices.2,
          prices.4 - prices.3,
        );
        if !bananas.contains_key(&change) {
          bananas.insert(change, prices.4);
        }
      });
      bananas
    })
    .collect();

  let mut all_changes = HashSet::new();
  all_bananas.iter().flat_map(|b| b.keys()).for_each(|c| {
    all_changes.insert(*c);
  });

  let p2 = all_changes
    .par_iter()
    .map(|c| all_bananas.iter().map(|b| b.get(c).unwrap_or(&0)).sum())
    .max()
    .unwrap();

  //
  (p1, p2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
1
10
100
2024
";

    assert_eq!(37327623, solve(input.trim()).0);
  }

  #[test]
  fn test2() {
    let input = "
1
2
3
2024
";

    assert_eq!(23, solve(input.trim()).1);
  }
}
