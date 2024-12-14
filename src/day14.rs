use itertools::Itertools;
use std::cmp::Ordering::*;

type TwoD = (i32, i32);

pub fn solve(input: &str) -> (usize, usize) {
  #[cfg(test)]
  let bound = (11, 7);
  #[cfg(not(test))]
  let bound = (101, 103);

  dbg!(bound);

  let p1 = input
    .lines()
    .filter_map(|line| {
      let (ps, vs) = line.split_once(" v=").unwrap();
      let (px, py) = ps[2..].split_once(',').unwrap();
      let (vx, vy) = vs.split_once(',').unwrap();

      let p: TwoD = (px.parse().unwrap(), py.parse().unwrap());
      let v: TwoD = (vx.parse().unwrap(), vy.parse().unwrap());

      let fp = (
        (p.0 + v.0 * 100).rem_euclid(bound.0),
        (p.1 + v.1 * 100).rem_euclid(bound.1),
      );

      match (fp.0.cmp(&(bound.0 / 2)), fp.1.cmp(&(bound.1 / 2))) {
        (Equal, _) | (_, Equal) => None,
        (Less, Less) => Some(1),
        (Less, Greater) => Some(2),
        (Greater, Less) => Some(3),
        (Greater, Greater) => Some(4),
      }
    })
    .counts()
    .values()
    .product();

  (p1, 0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    assert_eq!((12, 0), solve(input.trim()));
  }
}
