use rayon::prelude::*;
use std::collections::HashSet;

pub fn solve(input: &str) -> (u64, u64) {
  let equations: Vec<_> = input
    .lines()
    .map(|line| {
      let mut p = line.split(": ");
      (
        p.next().unwrap().parse::<u64>().unwrap(),
        p.next()
          .unwrap()
          .split_ascii_whitespace()
          .map(|x| x.parse::<u64>().unwrap())
          .collect::<Vec<_>>(),
      )
    })
    .collect();

  let p1 = equations.par_iter().filter_map(|(tv, nums)| valid_p1(*tv, nums)).sum();

  let p2 = equations.par_iter().filter_map(|(tv, nums)| valid_p2(*tv, nums)).sum();

  (p1, p2)
}

fn valid_p1(tv: u64, nums: &Vec<u64>) -> Option<u64> {
  let mut vs = HashSet::new();

  for n in nums.iter().copied() {
    if vs.is_empty() {
      vs.insert(n);
    } else {
      let mut nvs = HashSet::new();

      for n0 in vs.iter().copied() {
        nvs.insert(n0 + n);
        nvs.insert(n0 * n);
        nvs.insert(format!("{}{}", n0, n).parse().unwrap());
      }
      vs = nvs;
    }
  }

  vs.contains(&tv).then(|| tv)
}

fn valid_p2(tv: u64, nums: &Vec<u64>) -> Option<u64> {
  let mut vs = HashSet::new();

  for n in nums.iter().copied() {
    if vs.is_empty() {
      vs.insert(n);
    } else {
      let mut nvs = HashSet::new();

      for n0 in vs.iter().copied() {
        nvs.insert(n0 + n);
        nvs.insert(n0 * n);
        nvs.insert(format!("{}{}", n0, n).parse().unwrap());
      }
      vs = nvs;
    }
  }

  vs.contains(&tv).then(|| tv)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";

    assert_eq!((3749, 11387), solve(input.trim()));
  }
}
