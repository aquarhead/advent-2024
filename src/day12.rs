use std::collections::HashMap;

type Pos = (i32, i32);
type FenceKey = (i32, i32);
type Area = u64;
type Perimeter = u64;

pub fn solve(input: &str) -> (u64, u64) {
  let mut map: HashMap<Pos, (char, FenceKey)> = HashMap::new();
  let mut fences: HashMap<FenceKey, (Area, Perimeter)> = HashMap::new();

  input.lines().enumerate().for_each(|(row, line)| {
    line.char_indices().for_each(|(col, ch)| {
      let pos = (row as i32, col as i32);

      match (
        map.get(&(pos.0 - 1, pos.1)).copied(),
        map.get(&(pos.0, pos.1 - 1)).copied(),
      ) {
        (Some((up, fk1)), Some((left, fk2))) if left == ch && up == ch => {
          let f2 = fences.remove(&fk2).unwrap();
          fences
            .entry(fk1)
            .and_modify(|e| {
              (*e).0 += f2.0 + 1;
              (*e).1 += f2.1;
            })
            .or_insert((f2.0 + 1, f2.1));

          map.iter_mut().for_each(|(_, fk)| {
            if fk.1 == fk2 {
              fk.1 = fk1;
            }
          });

          map.insert(pos, (ch, fk1));
        }
        (Some((c2, fk)), _) | (_, Some((c2, fk))) if c2 == ch => {
          let fence = fences.get_mut(&fk).unwrap();
          fence.0 += 1;
          fence.1 += 2;
          map.insert(pos, (ch, fk));
        }
        (_, _) => {
          let fk = pos;
          fences.insert(fk, (1, 4));
          map.insert(pos, (ch, fk));
        }
      }
    });
  });

  let row_max = input.lines().enumerate().last().unwrap().0 as i32;
  let col_max = input.lines().next().unwrap().chars().enumerate().last().unwrap().0 as i32;

  let p1 = fences.values().map(|(a, p)| a * p).sum();

  let mut p2 = fences.clone();
  p2.iter_mut().for_each(|(_, e)| (*e).1 = 0);

  let diff = |p: &Pos, ch: char| -> bool { map.get(&p).map_or(true, |x| x.0 != ch) };

  for row in 0..=row_max {
    // top
    (0..=col_max).fold(false, |counted, col| {
      let cur = map.get(&(row, col)).copied().unwrap();
      match (diff(&(row - 1, col), cur.0), diff(&(row, col - 1), cur.0)) {
        (true, true) => {
          p2.get_mut(&cur.1).unwrap().1 += 1;
          true
        }
        (true, false) => {
          if !counted {
            p2.get_mut(&cur.1).unwrap().1 += 1;
          }
          true
        }
        (false, _) => false,
      }
    });

    // down
    (0..=col_max).fold(false, |counted, col| {
      let cur = map.get(&(row, col)).copied().unwrap();
      match (diff(&(row + 1, col), cur.0), diff(&(row, col - 1), cur.0)) {
        (true, true) => {
          p2.get_mut(&cur.1).unwrap().1 += 1;
          true
        }
        (true, false) => {
          if !counted {
            p2.get_mut(&cur.1).unwrap().1 += 1;
          }
          true
        }
        (false, _) => false,
      }
    });
  }

  for col in 0..=col_max {
    // left
    (0..=row_max).fold(false, |counted, row| {
      let cur = map.get(&(row, col)).copied().unwrap();
      match (diff(&(row, col - 1), cur.0), diff(&(row - 1, col), cur.0)) {
        (true, true) => {
          p2.get_mut(&cur.1).unwrap().1 += 1;
          true
        }
        (true, false) => {
          if !counted {
            p2.get_mut(&cur.1).unwrap().1 += 1;
          }
          true
        }
        (false, _) => false,
      }
    });

    // right
    (0..=row_max).fold(false, |counted, row| {
      let cur = map.get(&(row, col)).copied().unwrap();

      match (diff(&(row, col + 1), cur.0), diff(&(row - 1, col), cur.0)) {
        (true, true) => {
          p2.get_mut(&cur.1).unwrap().1 += 1;
          true
        }
        (true, false) => {
          if !counted {
            p2.get_mut(&cur.1).unwrap().1 += 1;
          }
          true
        }
        (false, _) => false,
      }
    });
  }

  (p1, p2.values().map(|(a, p)| a * p).sum())
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
AAAA
BBCD
BBCC
EEEC
";

    assert_eq!((140, 80), solve(input.trim()));
  }

  #[test]
  fn test_example2() {
    let input = "
OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

    assert_eq!(772, solve(input.trim()).0);
  }

  #[test]
  fn test_example3() {
    let input = "
RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    assert_eq!(1930, solve(input.trim()).0);
  }

  #[test]
  fn test4() {
    let input = "
EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
    assert_eq!(236, solve(input.trim()).1);
  }

  #[test]
  fn test5() {
    let input = "
AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
    assert_eq!(368, solve(input.trim()).1);
  }
}
