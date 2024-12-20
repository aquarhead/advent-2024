use rayon::prelude::*;

pub fn solve(input: &str) -> (usize, u64) {
  let mut lines = input.lines();
  let towels: Vec<_> = lines.next().unwrap().split(", ").collect();

  let designs: Vec<_> = lines.skip(1).collect();
  let p1 = designs
    .clone()
    .par_iter()
    .filter(|line| {
      let mut dp = vec![0; line.len() + 1];
      dp[0] = 1; // tracking valid _before_ idx
      for idx in 0..line.len() {
        for t in towels.iter() {
          if dp[idx] > 0 && line[idx..].starts_with(t) {
            dp[idx + t.len()] += dp[idx];
          }
        }
      }
      dp[line.len()] > 0
    })
    .count();

  let p2 = designs
    .clone()
    .par_iter()
    .map(|line| {
      let mut dp = vec![0; line.len() + 1];
      dp[0] = 1; // tracking valid _before_ idx
      for idx in 0..line.len() {
        for t in towels.iter() {
          if dp[idx] > 0 && line[idx..].starts_with(t) {
            dp[idx + t.len()] += dp[idx];
          }
        }
      }
      dp[line.len()]
    })
    .sum();

  (p1, p2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    assert_eq!((6, 16), solve(input.trim()));
  }
}
