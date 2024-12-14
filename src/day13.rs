type Pair = (i64, i64);

pub fn solve(input: &str) -> (i64, i64) {
  input
    .split("\n\n")
    .map(|machine| {
      let mut lines = machine.lines();
      let mut parse = |pre: usize, sp: &str| -> Pair {
        let (xs, ys) = lines.next().unwrap()[pre..].split_once(sp).unwrap();
        (xs.parse().unwrap(), ys.parse().unwrap())
      };

      let a = parse(12, ", Y+");
      let b = parse(12, ", Y+");
      let prize = parse(9, ", Y=");

      (
        win(a, b, prize),
        win(a, b, (prize.0 + 10000000000000, prize.1 + 10000000000000)),
      )
    })
    .fold((0, 0), |acc, (p1, p2)| {
      (p1.map_or(acc.0, |r| acc.0 + r), p2.map_or(acc.1, |r| acc.1 + r))
    })
}

fn win(a: Pair, b: Pair, prize: Pair) -> Option<i64> {
  if (prize.0 * b.1 - prize.1 * b.0) % (a.0 * b.1 - a.1 * b.0) == 0 {
    let i = (prize.0 * b.1 - prize.1 * b.0) / (a.0 * b.1 - a.1 * b.0);
    let j = (prize.0 - a.0 * i) / b.0;
    Some(i * 3 + j)
  } else {
    None
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    assert_eq!(480, solve(input.trim()).0);
  }
}
