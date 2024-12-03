pub fn solve(input: &str) -> (u32, u32) {
  (p1(input), p2(input))
}

fn p1(input: &str) -> u32 {
  let mut p1 = 0;
  for (mut idx, _) in input.match_indices("mul(") {
    idx += 4;
    let mut left = 0;
    for _ in 0..3 {
      if let Some(c) = input.chars().nth(idx) {
        if c.is_ascii_digit() {
          left *= 10;
          left += c.to_digit(10).unwrap();
          idx += 1;
        }
      }
    }

    if let Some(c) = input.chars().nth(idx) {
      if c != ',' {
        continue;
      }
    } else {
      continue;
    }

    idx += 1;

    let mut right = 0;
    for _ in 0..3 {
      if let Some(c) = input.chars().nth(idx) {
        if c.is_ascii_digit() {
          right *= 10;
          right += c.to_digit(10).unwrap();
          idx += 1;
        }
      }
    }

    if let Some(c) = input.chars().nth(idx) {
      if c != ')' {
        continue;
      }
    } else {
      continue;
    }

    p1 += left * right;
  }

  p1
}

fn p2(input: &str) -> u32 {
  let mut ops = Vec::new();
  input.match_indices("mul(").for_each(|m| ops.push(m));
  input.match_indices("do()").for_each(|m| ops.push(m));
  input.match_indices("don't()").for_each(|m| ops.push(m));
  ops.sort();

  let mut p2 = 0;
  let mut enabled = true;
  for (mut idx, op) in ops {
    if op == "do()" {
      enabled = true;
      continue;
    }

    if op == "don't()" {
      enabled = false;
      continue;
    }

    if !enabled {
      continue;
    }

    idx += 4;
    let mut left = 0;
    for _ in 0..3 {
      if let Some(c) = input.chars().nth(idx) {
        if c.is_ascii_digit() {
          left *= 10;
          left += c.to_digit(10).unwrap();
          idx += 1;
        }
      }
    }

    if let Some(c) = input.chars().nth(idx) {
      if c != ',' {
        continue;
      }
    } else {
      continue;
    }

    idx += 1;

    let mut right = 0;
    for _ in 0..3 {
      if let Some(c) = input.chars().nth(idx) {
        if c.is_ascii_digit() {
          right *= 10;
          right += c.to_digit(10).unwrap();
          idx += 1;
        }
      }
    }

    if let Some(c) = input.chars().nth(idx) {
      if c != ')' {
        continue;
      }
    } else {
      continue;
    }

    p2 += left * right;
  }

  p2
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    assert_eq!((161, 48), solve(input.trim()));
  }
}
