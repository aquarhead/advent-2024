use std::collections::VecDeque;

#[derive(Debug, Clone, Copy)]
enum DiskMap {
  File(usize, usize),
  Free(usize),
}

impl DiskMap {
  fn is_file(&self) -> bool {
    match self {
      File(_, _) => true,
      Free(_) => false,
    }
  }
}

use DiskMap::*;

pub fn solve(input: &str) -> (usize, usize) {
  let disk: VecDeque<_> = input
    .char_indices()
    .map(|(idx, ch)| {
      let n = ch.to_digit(10).unwrap() as usize;
      if idx % 2 == 0 {
        File(idx / 2, n)
      } else {
        Free(n)
      }
    })
    .collect();

  (p1(disk.clone()), p2(disk))
}

fn p1(mut disk: VecDeque<DiskMap>) -> usize {
  let mut ret = 0;

  for idx in 0.. {
    while let Some(m) = disk.pop_back() {
      if m.is_file() {
        disk.push_back(m);
        break;
      }
    }

    while let Some(m) = disk.pop_front() {
      match m {
        Free(0) => continue,
        _ => {
          disk.push_front(m);
          break;
        }
      }
    }

    match disk.pop_front() {
      Some(File(fid, fsize)) => {
        ret += idx * fid;
        if fsize > 1 {
          disk.push_front(File(fid, fsize - 1));
        }
      }
      Some(Free(free_size)) => {
        if let Some(File(fid, fsize)) = disk.pop_back() {
          let mut front_size = fsize;

          if free_size < fsize {
            front_size = free_size;
            disk.push_back(File(fid, fsize - free_size));
          }

          if free_size > fsize {
            disk.push_front(Free(free_size - fsize));
          }

          ret += idx * fid;
          if front_size > 1 {
            disk.push_front(File(fid, front_size - 1));
          }
        } else {
          unimplemented!()
        }
      }
      None => break,
    }
  }

  ret
}

fn p2(mut disk: VecDeque<DiskMap>) -> usize {
  let mut right = VecDeque::new();

  while let Some(File(fid, fsize)) = disk.pop_back() {
    let found_free_slot = disk.iter().position(|f| {
      if let Free(free_size) = f {
        *free_size >= fsize
      } else {
        false
      }
    });

    if let Some(idx) = found_free_slot {
      if let Free(free_size) = disk.remove(idx).unwrap() {
        disk.insert(idx, File(fid, fsize));
        if free_size > fsize {
          disk.insert(idx + 1, Free(free_size - fsize));
        }
        right.push_front(Free(fsize));
      } else {
        unimplemented!()
      }
    } else {
      right.push_front(File(fid, fsize));
    }

    while let Some(m) = disk.pop_back() {
      if m.is_file() {
        disk.push_back(m);
        break;
      } else {
        right.push_front(m);
      }
    }
  }

  disk = right;

  let mut ret = 0;

  for idx in 0.. {
    while let Some(m) = disk.pop_front() {
      match m {
        Free(0) => continue,
        _ => {
          disk.push_front(m);
          break;
        }
      }
    }

    match disk.pop_front() {
      Some(File(fid, fsize)) => {
        ret += idx * fid;
        if fsize > 1 {
          disk.push_front(File(fid, fsize - 1));
        }
      }
      Some(Free(free_size)) => {
        if free_size > 1 {
          disk.push_front(Free(free_size - 1));
        }
      }
      None => break,
    }
  }

  ret
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "2333133121414131402";

    assert_eq!((1928, 2858), solve(input.trim()));
  }
}
