use pathfinding::prelude::dijkstra;
use std::collections::HashSet;

type Pos = (i32, i32);

pub fn solve(input: &str) -> (usize, &str) {
  #[cfg(test)]
  let t = 12;
  #[cfg(not(test))]
  let t = 1024;

  let corrupted: HashSet<Pos> = input
    .lines()
    .take(t)
    .map(|line| {
      let (x, y) = line.split_once(',').unwrap();
      (x.parse().unwrap(), y.parse().unwrap())
    })
    .collect();

  #[cfg(test)]
  let bound = 6;
  #[cfg(not(test))]
  let bound = 70;

  let p1 = dijk(bound, &corrupted).unwrap();

  let p2 = {
    let mut corrupted = HashSet::new();
    input
      .lines()
      .find(|line| {
        let (x, y) = line.split_once(',').unwrap();
        let p: Pos = (x.parse().unwrap(), y.parse().unwrap());
        corrupted.insert(p);
        dijk(bound, &corrupted).is_none()
      })
      .unwrap()
  };

  (p1, p2)
}

fn dijk(bound: i32, corrupted: &HashSet<Pos>) -> Option<usize> {
  let start: Pos = (0, 0);
  dijkstra(
    &start,
    |p| {
      [(-1, 0), (1, 0), (0, -1), (0, 1)]
        .iter()
        .filter_map(|m| {
          let np = (p.0 + m.0, p.1 + m.1);
          if np.0 >= 0 && np.0 <= bound && np.1 >= 0 && np.1 <= bound && !corrupted.contains(&np) {
            Some((np, 1))
          } else {
            None
          }
        })
        .collect::<Vec<_>>()
    },
    |p| p.0 == bound && p.1 == bound,
  )
  .map(|(_, steps)| steps)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    assert_eq!((22, "6,1"), solve(input.trim()));
  }
}
