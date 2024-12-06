use itertools::Itertools;
use rayon::prelude::*;
use std::collections::HashSet;

type Pos = (i32, i32);

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
  Up,
  Right,
  Down,
  Left,
}

impl Direction {
  fn turn_right(&mut self) {
    use Direction::*;
    *self = match self {
      Up => Right,
      Right => Down,
      Down => Left,
      Left => Up,
    }
  }

  fn forward(&self, p: &Pos) -> Pos {
    use Direction::*;
    match self {
      Up => (p.0 - 1, p.1),
      Right => (p.0, p.1 + 1),
      Down => (p.0 + 1, p.1),
      Left => (p.0, p.1 - 1),
    }
  }
}

impl From<char> for Direction {
  fn from(value: char) -> Self {
    match value {
      '^' => Self::Up,
      '>' => Self::Right,
      'v' => Self::Down,
      '<' => Self::Left,
      _ => unimplemented!(),
    }
  }
}

pub fn solve(input: &str) -> (usize, usize) {
  let mut obstructions = HashSet::new();
  let mut start_pos: Pos = (0, 0);
  let mut start_dir = Direction::Up;

  input.lines().enumerate().for_each(|(row, line)| {
    line.chars().enumerate().for_each(|(col, ch)| {
      let p: Pos = (row as i32, col as i32);
      if ch == '#' {
        obstructions.insert(p);
      } else if ch != '.' {
        start_pos = p;
        start_dir = ch.into();
      }
    })
  });

  let row_max = input.lines().enumerate().last().unwrap().0 as i32;
  let col_max = input.lines().next().unwrap().chars().enumerate().last().unwrap().0 as i32;

  let inside = |p: Pos| -> bool { p.0 >= 0 && p.0 <= row_max && p.1 >= 0 && p.1 <= col_max };

  let p1 = {
    let mut pos = start_pos;
    let mut dir = start_dir;
    let mut visited = HashSet::new();

    while inside(pos) {
      visited.insert(pos.clone());

      let np = dir.forward(&pos);

      if obstructions.contains(&np) {
        dir.turn_right();
      } else {
        pos = np;
      }
    }

    visited.len()
  };

  let p2 = {
    let possible: Vec<_> = (0..=row_max)
      .cartesian_product(0..=col_max)
      .filter(|p| !(obstructions.contains(p) || *p == start_pos))
      .collect();

    possible
      .into_par_iter()
      .filter(|new_ob_pos| {
        let mut no = obstructions.clone();
        no.insert(*new_ob_pos);

        let mut pos = start_pos;
        let mut dir = start_dir;
        let mut visited = HashSet::new();

        let mut has_loop = false;

        while inside(pos) {
          if visited.contains(&(pos, dir)) {
            has_loop = true;
            break;
          }
          visited.insert((pos.clone(), dir));

          let np = dir.forward(&pos);

          if no.contains(&np) {
            dir.turn_right();
          } else {
            pos = np;
          }
        }

        has_loop
      })
      .count()
  };

  (p1, p2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";

    assert_eq!((41, 6), solve(input.trim()));
  }
}
