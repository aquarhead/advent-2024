use itertools::Itertools;
use rayon::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum DPad {
  Up,
  Down,
  Left,
  Right,
  Activate,
}

use DPad::*;

impl DPad {
  fn all() -> impl IntoIterator<Item = Self> {
    [Up, Down, Left, Right, Activate]
  }
}

const NUM_ACTIVATE: u8 = 10;

type Sequence = Vec<DPad>;
type Possibilities = Vec<Sequence>;

fn num_pad(src: u8, dst: u8) -> Possibilities {
  if src == dst {
    vec![vec![Activate]]
  } else {
    let mut ret = match (src, dst) {
      // 1 step
      (NUM_ACTIVATE, 0) | (3, 2) | (6, 5) | (9, 8) | (2, 1) | (5, 4) | (8, 7) => vec![vec![Left]],
      (0, NUM_ACTIVATE) | (2, 3) | (5, 6) | (8, 9) | (1, 2) | (4, 5) | (7, 8) => vec![vec![Right]],
      (NUM_ACTIVATE, 3) | (3, 6) | (6, 9) | (0, 2) | (2, 5) | (5, 8) | (1, 4) | (4, 7) => vec![vec![Up]],
      (3, NUM_ACTIVATE) | (6, 3) | (9, 6) | (2, 0) | (5, 2) | (8, 5) | (4, 1) | (7, 4) => vec![vec![Down]],
      // 2 steps straight
      (3, 1) | (6, 4) | (9, 7) => vec![vec![Left, Left]],
      (1, 3) | (4, 6) | (7, 9) => vec![vec![Right, Right]],
      (NUM_ACTIVATE, 6) | (3, 9) | (0, 5) | (2, 8) | (1, 7) => vec![vec![Up, Up]],
      (6, NUM_ACTIVATE) | (9, 3) | (5, 0) | (8, 2) | (7, 1) => vec![vec![Down, Down]],
      // 2 steps cross
      (NUM_ACTIVATE, 2) | (3, 5) | (2, 4) | (6, 8) | (5, 7) => vec![vec![Left, Up], vec![Up, Left]],
      (2, NUM_ACTIVATE) | (5, 3) | (4, 2) | (8, 6) | (7, 5) => {
        vec![vec![Right, Down], vec![Down, Right]]
      }
      (0, 3) | (1, 5) | (2, 6) | (4, 8) | (5, 9) => vec![vec![Right, Up], vec![Up, Right]],
      (3, 0) | (5, 1) | (6, 2) | (8, 4) | (9, 5) => vec![vec![Left, Down], vec![Down, Left]],
      // special
      (0, 1) => vec![vec![Up, Left]],
      (1, 0) => vec![vec![Right, Down]],
      // 3 steps
      (NUM_ACTIVATE, 9) | (0, 8) => vec![vec![Up, Up, Up]],
      (9, NUM_ACTIVATE) | (8, 0) => vec![vec![Down, Down, Down]],
      (NUM_ACTIVATE, 1) => vec![vec![Up, Left, Left], vec![Left, Up, Left]],
      (1, NUM_ACTIVATE) => vec![vec![Right, Right, Down], vec![Right, Down, Right]],
      (3, 4) | (6, 7) => vec![
        //
        vec![Up, Left, Left],
        vec![Left, Up, Left],
        vec![Left, Left, Up],
      ],
      (4, 3) | (7, 6) => vec![
        vec![Down, Right, Right],
        vec![Right, Down, Right],
        vec![Right, Right, Down],
      ],
      (1, 6) | (4, 9) => vec![
        //
        vec![Up, Right, Right],
        vec![Right, Up, Right],
        vec![Right, Right, Up],
      ],
      (6, 1) | (9, 4) => vec![
        //
        vec![Down, Left, Left],
        vec![Left, Down, Left],
        vec![Left, Left, Down],
      ],
      (NUM_ACTIVATE, 5) | (2, 7) | (3, 8) => vec![
        //
        vec![Left, Up, Up],
        vec![Up, Left, Up],
        vec![Up, Up, Left],
      ],
      (5, NUM_ACTIVATE) | (7, 2) | (8, 3) => vec![
        vec![Right, Down, Down],
        vec![Down, Right, Down],
        vec![Down, Down, Right],
      ],
      (1, 8) | (2, 9) | (0, 6) => vec![
        //
        vec![Right, Up, Up],
        vec![Up, Right, Up],
        vec![Up, Up, Right],
      ],
      (8, 1) | (9, 2) | (6, 0) => vec![
        //
        vec![Left, Down, Down],
        vec![Down, Left, Down],
        vec![Down, Down, Left],
      ],
      (0, 4) => vec![vec![Up, Up, Left], vec![Up, Left, Up]],
      (4, 0) => vec![vec![Right, Down, Down], vec![Down, Right, Down]],

      // 4 steps
      (NUM_ACTIVATE, 4) => vec![
        vec![Left, Up, Up, Left],
        vec![Left, Up, Left, Up],
        vec![Up, Left, Left, Up],
        vec![Up, Left, Up, Left],
        vec![Up, Up, Left, Left],
      ],
      (4, NUM_ACTIVATE) => vec![
        vec![Right, Down, Down, Right],
        vec![Right, Down, Right, Down],
        vec![Down, Right, Right, Down],
        vec![Down, Right, Down, Right],
        vec![Right, Right, Down, Down],
      ],
      (3, 7) => vec![
        vec![Left, Left, Up, Up],
        vec![Left, Up, Up, Left],
        vec![Left, Up, Left, Up],
        vec![Up, Left, Left, Up],
        vec![Up, Left, Up, Left],
        vec![Up, Up, Left, Left],
      ],
      (7, 3) => vec![
        vec![Right, Right, Down, Down],
        vec![Right, Down, Down, Right],
        vec![Right, Down, Right, Down],
        vec![Down, Right, Right, Down],
        vec![Down, Right, Down, Right],
        vec![Down, Down, Right, Right],
      ],
      (1, 9) => vec![
        vec![Right, Right, Up, Up],
        vec![Right, Up, Up, Right],
        vec![Right, Up, Right, Up],
        vec![Up, Right, Right, Up],
        vec![Up, Right, Up, Right],
        vec![Up, Up, Right, Right],
      ],
      (9, 1) => vec![
        vec![Left, Left, Down, Down],
        vec![Left, Down, Down, Left],
        vec![Left, Down, Left, Down],
        vec![Down, Left, Left, Down],
        vec![Down, Left, Down, Left],
        vec![Down, Down, Left, Left],
      ],
      (NUM_ACTIVATE, 8) => vec![
        vec![Left, Up, Up, Up],
        vec![Up, Left, Up, Up],
        vec![Up, Up, Left, Up],
        vec![Up, Up, Up, Left],
      ],
      (8, NUM_ACTIVATE) => vec![
        vec![Right, Down, Down, Down],
        vec![Down, Right, Down, Down],
        vec![Down, Down, Right, Down],
        vec![Down, Down, Down, Right],
      ],
      (0, 7) => vec![
        //
        vec![Up, Left, Up, Up],
        vec![Up, Up, Left, Up],
        vec![Up, Up, Up, Left],
      ],
      (7, 0) => vec![
        //
        vec![Right, Down, Down, Down],
        vec![Down, Right, Down, Down],
        vec![Down, Down, Right, Down],
      ],
      (0, 9) => vec![
        vec![Right, Up, Up, Up],
        vec![Up, Right, Up, Up],
        vec![Up, Up, Right, Up],
        vec![Up, Up, Up, Right],
      ],
      (9, 0) => vec![
        vec![Left, Down, Down, Down],
        vec![Down, Left, Down, Down],
        vec![Down, Down, Left, Down],
        vec![Down, Down, Down, Left],
      ],

      // 5 steps
      (NUM_ACTIVATE, 7) => vec![
        vec![Left, Up, Left, Up, Up],
        vec![Left, Up, Up, Left, Up],
        vec![Left, Up, Up, Up, Left],
        vec![Up, Left, Left, Up, Up],
        vec![Up, Left, Up, Left, Up],
        vec![Up, Left, Up, Up, Left],
        vec![Up, Up, Left, Left, Up],
        vec![Up, Up, Left, Up, Left],
        vec![Up, Up, Up, Left, Left],
      ],
      (7, NUM_ACTIVATE) => vec![
        vec![Right, Down, Right, Down, Down],
        vec![Right, Down, Down, Right, Down],
        vec![Right, Down, Down, Down, Right],
        vec![Down, Right, Right, Down, Down],
        vec![Down, Right, Down, Right, Down],
        vec![Down, Right, Down, Down, Right],
        vec![Down, Down, Right, Right, Down],
        vec![Down, Down, Right, Down, Right],
        vec![Right, Right, Down, Down, Down],
      ],

      (a, b) => panic!("invalid ({}, {}) on num pad", a, b),
    };
    ret.iter_mut().for_each(|p| p.push(Activate));
    ret
  }
}

fn dpad(src: DPad, dst: DPad) -> Vec<Vec<DPad>> {
  if src == dst {
    vec![vec![Activate]]
  } else {
    let mut ret = match (src, dst) {
      // 1 step
      (Up, Down) | (Activate, Right) => vec![vec![Down]],
      (Down, Up) | (Right, Activate) => vec![vec![Up]],
      (Activate, Up) | (Right, Down) | (Down, Left) => vec![vec![Left]],
      (Left, Down) | (Down, Right) | (Up, Activate) => vec![vec![Right]],
      // 2 step
      (Left, Right) => vec![vec![Right, Right]],
      (Right, Left) => vec![vec![Left, Left]],
      (Activate, Down) => vec![vec![Left, Down], vec![Down, Left]],
      (Down, Activate) => vec![vec![Right, Up], vec![Up, Right]],
      (Up, Left) => vec![vec![Down, Left]],
      (Left, Up) => vec![vec![Right, Up]],
      (Up, Right) => vec![vec![Right, Down], vec![Down, Right]],
      (Right, Up) => vec![vec![Left, Up], vec![Up, Left]],
      // 3 step
      (Activate, Left) => vec![vec![Left, Down, Left], vec![Down, Left, Left]],
      (Left, Activate) => vec![vec![Right, Up, Right], vec![Right, Right, Up]],
      (a, b) => panic!("invalid ({:?}, {:?}) on dpad", a, b),
    };
    ret.iter_mut().for_each(|p| p.push(Activate));
    ret
  }
}

pub fn solve(input: &str) -> (usize, usize) {
  let p1 = input
    .lines()
    .map(|line| {
      let code: usize = line[0..(line.len() - 1)].parse().unwrap();
      let mut np_seq = vec![NUM_ACTIVATE];
      line.chars().for_each(|ch| match ch {
        'A' => np_seq.push(NUM_ACTIVATE),
        n => np_seq.push(n as u8 - '0' as u8),
      });

      let dp1: Possibilities = np_seq
        .into_iter()
        .tuple_windows()
        .map(|(i, j)| num_pad(i, j))
        .multi_cartesian_product()
        .map(|vs| vs.concat())
        .collect();

      let dp1_ml = dp1.iter().map(|s| s.len()).min().unwrap();

      dbg!(line);
      for p in dp1.iter() {
        if p.len() == dp1_ml {
          dbg!(p);
        }
      }

      let dp2: Possibilities = dp1
        .into_iter()
        .filter(|s| s.len() == dp1_ml)
        .flat_map(|s| {
          let mut s2 = s.clone();
          s2.insert(0, Activate);
          s2.iter()
            .tuple_windows()
            .map(|(i, j)| dpad(*i, *j))
            .multi_cartesian_product()
            .map(|vs| vs.concat())
            .collect::<Possibilities>()
        })
        .unique()
        .collect();

      let dp2_ml = dp2.iter().map(|s| s.len()).min().unwrap();

      let human: Possibilities = dp2
        .into_iter()
        .filter(|s| s.len() == dp2_ml)
        .flat_map(|s| {
          let mut s2 = s.clone();
          s2.insert(0, Activate);
          s2.iter()
            .tuple_windows()
            .map(|(i, j)| dpad(*i, *j))
            .multi_cartesian_product()
            .map(|vs| vs.concat())
            .collect::<Possibilities>()
        })
        .unique()
        .collect();

      let human_ml = human.iter().map(|s| s.len()).min().unwrap();

      code * human_ml
    })
    .sum();

  // let lines: Vec<_> = input.lines().collect();
  // let p2 = lines
  //   .into_iter()
  //   .map(|line| {
  //     let code: usize = line[0..(line.len() - 1)].parse().unwrap();
  //     let mut np_seq = vec![NUM_ACTIVATE];
  //     line.chars().for_each(|ch| match ch {
  //       'A' => np_seq.push(NUM_ACTIVATE),
  //       n => np_seq.push(n as u8 - '0' as u8),
  //     });

  //     let mut dp: Possibilities = np_seq
  //       .into_iter()
  //       .tuple_windows()
  //       .map(|(i, j)| num_pad(i, j))
  //       .multi_cartesian_product()
  //       .map(|vs| vs.concat())
  //       .collect();

  //     let mut min_len = dp.iter().map(|s| s.len()).min().unwrap();

  //     for n in 0..25 {
  //       dbg!(n);
  //       dp = dp
  //         .into_iter()
  //         .filter(|s| s.len() == min_len)
  //         .flat_map(|s| {
  //           let mut s2 = s.clone();
  //           s2.insert(0, Activate);
  //           s2.iter()
  //             .tuple_windows()
  //             .map(|(i, j)| dpad(*i, *j))
  //             .multi_cartesian_product()
  //             .map(|vs| vs.concat())
  //             .collect::<Possibilities>()
  //         })
  //         .collect();

  //       min_len = dp.iter().map(|s| s.len()).min().unwrap();
  //     }

  //     code * min_len
  //   })
  //   .sum();

  (p1, 0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_impl() {
    for i in 0..=NUM_ACTIVATE {
      for j in 0..=NUM_ACTIVATE {
        num_pad(i, j);
      }
    }

    for i in DPad::all() {
      for j in DPad::all() {
        dpad(i, j);
      }
    }
  }

  #[test]
  fn test_example() {
    let input = "
029A
980A
179A
456A
379A
";

    assert_eq!((126384, 0), solve(input.trim()));
  }
}
