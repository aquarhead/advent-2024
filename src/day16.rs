use pathfinding::prelude::dijkstra;
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

pub fn solve(input: &str) -> (i32, i32) {
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

  let (_, p1) = dijkstra(
    &start,
    |r| {
      let (a, b) = r.turn();
      let mut succ = vec![(a, 1000), (b, 1000)];
      let rf = r.forward();
      if tiles.contains(&rf.pos) {
        succ.push((rf, 1));
      }
      succ
    },
    |r| r.pos == end,
  )
  .unwrap();

  (p1, 0)
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
