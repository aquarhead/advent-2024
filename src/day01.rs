use itertools::Itertools;

pub fn solve(input: &str) -> (u64, u64) {
  let mut left = Vec::new();
  let mut right = Vec::new();
  for l in input.trim().lines() {
    let mut s = l.trim().split("   ");
    left.push(s.next().unwrap().parse::<u64>().unwrap());
    right.push(s.next().unwrap().parse::<u64>().unwrap());
  }

  left.sort();
  let mut r1 = right.clone();
  r1.sort();

  let p1 = left
    .clone()
    .into_iter()
    .zip(r1.into_iter())
    .map(|(x, y)| x.abs_diff(y))
    .sum();

  let r2 = right.into_iter().counts();
  let p2 = left
    .into_iter()
    .map(|x| x * r2.get(&x).map(|y| *y as u64).unwrap_or(0))
    .sum();

  (p1, p2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
3   4
4   3
2   5
1   3
3   9
3   3
";

    assert_eq!((11, 31), solve(input));
  }
}
