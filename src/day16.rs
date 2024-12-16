use pathfinding::directed::dijkstra;
use std::collections::HashSet;

type Pos = (i32, i32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
  North,
  South,
  East,
  West,
}

use Direction::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Reindeer {
  pos: Pos,
  dir: Direction,
}

impl Reindeer {
  fn forward(&self) -> Self {
    let mut ret = self.clone();
    match self.dir {
      North => ret.pos.0 -= 1,
      South => ret.pos.0 += 1,
      East => ret.pos.1 += 1,
      West => ret.pos.1 -= 1,
    }
    ret
  }

  fn backward(&self) -> Self {
    let mut ret = self.clone();
    match self.dir {
      North => ret.pos.0 += 1,
      South => ret.pos.0 -= 1,
      East => ret.pos.1 -= 1,
      West => ret.pos.1 += 1,
    }
    ret
  }

  fn turn(&self) -> (Self, Self) {
    let mut a = self.clone();
    let mut b = self.clone();
    match self.dir {
      North | South => {
        a.dir = East;
        b.dir = West;
      }
      East | West => {
        a.dir = North;
        b.dir = South;
      }
    }
    (a, b)
  }
}

pub fn solve(input: &str) -> (i32, usize) {
  let mut tiles = HashSet::new();
  let mut start = (0, 0);
  let mut end = (0, 0);
  input.lines().enumerate().for_each(|(row, line)| {
    line.char_indices().for_each(|(col, ch)| {
      let p: Pos = (row as i32, col as i32);
      if ch != '#' {
        tiles.insert(p);
      }
      if ch == 'S' {
        start = p;
      }
      if ch == 'E' {
        end = p;
      }
    })
  });

  let start = Reindeer { pos: start, dir: East };

  let da = dijkstra::dijkstra_all(&start, |r| {
    let (a, b) = r.turn();
    let mut succ = vec![(a, 1000), (b, 1000)];
    let rf = r.forward();
    if tiles.contains(&rf.pos) {
      succ.push((rf, 1));
    }
    succ
  });

  let p1 = da
    .iter()
    .filter_map(|(r, (_, d))| (r.pos == end).then(|| d))
    .min()
    .unwrap();

  let mut p2 = HashSet::new();
  let mut search: HashSet<_> = da
    .iter()
    .filter_map(|(r, (_, d))| (r.pos == end && d == p1).then(|| (*r, *d)))
    .collect();

  while !search.is_empty() {
    p2.extend(search.clone());

    let mut new_search = HashSet::new();
    for s in search {
      let br = s.0.backward();
      if let Some((_, d)) = da.get(&br) {
        if *d == s.1 - 1 {
          new_search.insert((br, *d));
        }
      }

      let (a, b) = s.0.turn();
      if let Some((_, d)) = da.get(&a) {
        if *d == s.1 - 1000 {
          new_search.insert((a, *d));
        }
      }

      if let Some((_, d)) = da.get(&b) {
        if *d == s.1 - 1000 {
          new_search.insert((b, *d));
        }
      }
    }

    search = new_search;
  }



  (*p1, p2.into_iter().map(|(r, _)| r.pos).collect::<HashSet<_>>().len())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    assert_eq!((7036, 45), solve(input.trim()));
  }

  #[test]
  fn test2() {
    let input = "
#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    assert_eq!((11048, 64), solve(input.trim()));
  }
}
