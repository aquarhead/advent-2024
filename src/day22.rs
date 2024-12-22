use itertools::Itertools;
use rayon::prelude::*;
use std::collections::{HashMap, HashSet};

const PRUNE_MASK: usize = 16777216 - 1;

pub fn solve(input: &str) -> (usize, usize) {
  let secrets: Vec<usize> = input.lines().map(|line| line.parse().unwrap()).collect();
  let p1 = secrets
    .into_par_iter()
    .map(|secret| {
      std::iter::repeat_n((), 2000).fold(secret, |mut s, _| {
        s ^= s << 6;
        s &= PRUNE_MASK;
        s ^= s >> 5;
        s &= PRUNE_MASK;
        s ^= s << 11;
        s &= PRUNE_MASK;

        s
      })
    })
    .sum();

  (p1, 0)
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
