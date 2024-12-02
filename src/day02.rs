use itertools::Itertools;
use rayon::prelude::*;

pub fn solve(input: &str) -> (usize, usize) {
  let p1 = input
    .lines()
    .filter(|line| {
      let l: Vec<_> = line
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

      safe_report(&l)
    })
    .count();

  let p2 = input
    .lines()
    .collect::<Vec<_>>()
    .into_par_iter()
    .filter(|line| {
      let l: Vec<_> = line
        .trim()
        .split_ascii_whitespace()
        .map(|x| x.parse::<u32>().unwrap())
        .collect();

      if safe_report(&l) {
        return true;
      }

      for i in 0..l.len() {
        let mut l1 = l.clone();
        l1.remove(i);
        if safe_report(&l1) {
          return true;
        }
      }

      false
    })
    .count();

  (p1, p2)
}

fn safe_report(level: &Vec<u32>) -> bool {
  level.iter().tuple_windows().map(|(x, y)| x > y).all_equal()
    && level
      .iter()
      .tuple_windows()
      .all(|(x, y)| x.abs_diff(*y) >= 1 && x.abs_diff(*y) <= 3)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

    assert_eq!((2, 4), solve(input.trim()));
  }
}
