use std::collections::HashMap;

pub fn solve(input: &str) -> (u64, u64) {
  let mut memo = HashMap::new();

  let p1 = input
    .split_ascii_whitespace()
    .map(|n| blink(n.parse().unwrap(), 25, &mut memo))
    .sum();

  let p2 = input
    .split_ascii_whitespace()
    .map(|n| blink(n.parse().unwrap(), 75, &mut memo))
    .sum();

  (p1, p2)
}

fn blink(n: u64, left: u8, memo: &mut HashMap<(u64, u8), u64>) -> u64 {
  if let Some(r) = memo.get(&(n, left)) {
    return *r;
  }

  let r = if left == 0 {
    1
  } else if n == 0 {
    blink(1, left - 1, memo)
  } else {
    let ns = n.to_string();
    if ns.len() % 2 == 0 {
      let (a, b) = ns.split_at(ns.len() / 2);
      blink(a.parse().unwrap(), left - 1, memo) + blink(b.parse().unwrap(), left - 1, memo)
    } else {
      blink(n * 2024, left - 1, memo)
    }
  };

  memo.insert((n, left), r);

  r
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "125 17";

    assert_eq!(55312, solve(input.trim()).0);
  }
}
