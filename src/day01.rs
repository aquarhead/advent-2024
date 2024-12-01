pub fn solve(input: &str) -> (u64, u64) {
  (part1(input), part2(input))
}

fn part1(input: &str) -> u64 {
  let mut left = Vec::new();
  let mut right = Vec::new();
  for l in input.trim().lines() {
    let mut s = l.trim().split("   ");
    left.push(s.next().unwrap().parse::<u64>().unwrap());
    right.push(s.next().unwrap().parse::<u64>().unwrap());
  }

  left.sort();
  right.sort();

  left
    .into_iter()
    .zip(right.into_iter())
    .map(|(x, y)| x.abs_diff(y))
    .sum()
}

fn part2(input: &str) -> u64 {
  0
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

    assert_eq!((11, 0), solve(input));
  }
}
