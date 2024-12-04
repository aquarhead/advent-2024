use itertools::Itertools;
use std::collections::HashMap;

type Pos = (i32, i32);

pub fn solve(input: &str) -> (u32, u32) {
  let mut map = HashMap::new();
  input.lines().enumerate().for_each(|(row, line)| {
    line.trim().char_indices().for_each(|(col, ch)| {
      map.insert((row as i32, col as i32), ch);
    });
  });

  let new_pos = |p: Pos, m: Pos, s: i32| -> Pos { ((p.0 + (s * m.0)), (p.1 + (s * m.1))) };

  let p1 = map
    .iter()
    .filter_map(|(pos, ch)| (*ch == 'X').then(|| *pos))
    .cartesian_product([(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, -1)])
    .filter(|(pos, mov)| {
      map.get(&new_pos(*pos, *mov, 1)).map_or(false, |ch| *ch == 'M')
        && map.get(&new_pos(*pos, *mov, 2)).map_or(false, |ch| *ch == 'A')
        && map.get(&new_pos(*pos, *mov, 3)).map_or(false, |ch| *ch == 'S')
    })
    .count() as u32;

  let p2 = map
    .iter()
    .filter_map(|(pos, ch)| (*ch == 'A').then(|| *pos))
    // testing for locations of M, then S should be opposite
    .cartesian_product([
      ((-1, -1), (-1, 1)), // top left, top right
      ((-1, -1), (1, -1)), // top left, bottom left
      ((1, 1), (-1, 1)),   // bottom right, top right
      ((1, 1), (1, -1)),   // bottom right, bottom left
    ])
    .filter(|(pos, mov)| {
      let (m0, m1) = *mov;
      map.get(&new_pos(*pos, m0, 1)).map_or(false, |ch| *ch == 'M')
        && map.get(&new_pos(*pos, m1, 1)).map_or(false, |ch| *ch == 'M')
        && map.get(&new_pos(*pos, m0, -1)).map_or(false, |ch| *ch == 'S')
        && map.get(&new_pos(*pos, m1, -1)).map_or(false, |ch| *ch == 'S')
    })
    .count() as u32;

  (p1, p2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";

    assert_eq!((18, 9), solve(input.trim()));
  }
}
