use std::collections::HashSet;

pub fn solve(input: &str) -> (i32, i32) {
  (p1(input), p2(input))
}

fn p1(input: &str) -> i32 {
  let mut walls = HashSet::new();
  let mut boxes = HashSet::new();
  let mut robot = (0, 0);

  let (map, moves) = input.split_once("\n\n").unwrap();
  map.lines().enumerate().for_each(|(row, line)| {
    line.char_indices().for_each(|(col, ch)| {
      let p = (row as i32, col as i32);
      match ch {
        '#' => {
          walls.insert(p);
        }
        'O' => {
          boxes.insert(p);
        }
        '@' => {
          robot = p;
        }
        _ => {}
      }
    })
  });

  for line in moves.lines() {
    for m in line.chars() {
      let mov = match m {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => unimplemented!(),
      };

      let mut carrying_box = false;
      let mut np = robot;
      loop {
        np = (np.0 + mov.0, np.1 + mov.1);
        if walls.contains(&np) {
          // not move
          break;
        } else {
          if boxes.contains(&np) {
            carrying_box = true;
          } else {
            // moved
            robot = (robot.0 + mov.0, robot.1 + mov.1);

            if carrying_box {
              if !boxes.remove(&robot) {
                panic!("wat");
              }
              boxes.insert(np);
            }

            break;
          }
        }
      }
    }
  }

  boxes.iter().map(|(r, c)| r * 100 + c).sum()
}

fn p2(input: &str) -> i32 {
  let mut walls = HashSet::new();
  let mut boxes = HashSet::new();
  let mut robot = (0, 0);

  let (map, moves) = input.split_once("\n\n").unwrap();
  map.trim().lines().enumerate().for_each(|(row, line)| {
    line.char_indices().for_each(|(col, ch)| {
      let p = (row as i32, 2 * col as i32);
      let p2 = (row as i32, 2 * col as i32 + 1);
      match ch {
        '#' => {
          walls.insert(p);
          walls.insert(p2);
        }
        'O' => {
          boxes.insert(p);
        }
        '@' => {
          robot = p;
        }
        _ => {}
      }
    })
  });

  for line in moves.lines() {
    for m in line.chars() {
      let mov = match m {
        '^' => (-1, 0),
        'v' => (1, 0),
        '<' => (0, -1),
        '>' => (0, 1),
        _ => unimplemented!(),
      };

      let mut search = HashSet::from([robot]);
      let mut carrying = HashSet::new();

      loop {
        search = search.iter().map(|s| (s.0 + mov.0, s.1 + mov.1)).collect();

        if search.iter().any(|s| walls.contains(s)) {
          // not move
          break;
        } else {
          let more_box = search.iter().fold(HashSet::new(), |mut acc, s| {
            if boxes.contains(&s) {
              if !carrying.contains(s) {
                acc.insert(*s);
              }
            }

            let left = (s.0, s.1 - 1);
            if boxes.contains(&left) {
              if !carrying.contains(&left) {
                acc.insert(left);
              }
            }

            acc
          });
          if more_box.is_empty() {
            // moved
            robot = (robot.0 + mov.0, robot.1 + mov.1);

            for b in carrying.iter() {
              if !boxes.remove(b) {
                panic!("where's the box?!");
              }
            }
            for b in carrying {
              boxes.insert((b.0 + mov.0, b.1 + mov.1));
            }

            break;
          } else {
            search = more_box.iter().flat_map(|s| [*s, (s.0, s.1 + 1)]).collect();
            carrying = carrying.union(&more_box).copied().collect();
          }
        }
      }
    }
  }

  boxes.iter().map(|(r, c)| r * 100 + c).sum()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    assert_eq!((10092, 9021), solve(input.trim()));
  }

  #[test]
  fn test2() {
    let input = "
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    assert_eq!(2028, solve(input.trim()).0);
  }

  #[test]
  fn test3() {
    let input = "
#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^
";

    solve(input.trim());
  }
}
