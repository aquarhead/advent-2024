use rayon::prelude::*;
use std::cmp::Ordering;
use std::collections::HashMap;

struct Rule {
  front: u32,
  back: u32,
}

impl Rule {
  fn from_str(rs: &str) -> Self {
    let mut p = rs.split('|');
    Self {
      front: p.next().unwrap().parse().unwrap(),
      back: p.next().unwrap().parse().unwrap(),
    }
  }

  fn get_pos(&self, pos: &HashMap<u32, usize>) -> Option<(usize, usize)> {
    match (pos.get(&self.front), pos.get(&self.back)) {
      (Some(a), Some(b)) => Some((*a, *b)),
      _ => None,
    }
  }

  fn valid(&self, pos: &HashMap<u32, usize>) -> bool {
    if let Some((a, b)) = self.get_pos(pos) {
      a < b
    } else {
      true
    }
  }

  fn order(&self, a: u32, b: u32) -> Option<Ordering> {
    if a == self.front && b == self.back {
      Some(Ordering::Less)
    } else if a == self.back && b == self.front {
      Some(Ordering::Greater)
    } else {
      None
    }
  }
}

pub fn solve(input: &str) -> (u32, u32) {
  let mut parts = input.split("\n\n");
  let rules: Vec<_> = parts.next().unwrap().lines().map(Rule::from_str).collect();
  let updates: Vec<_> = parts
    .next()
    .unwrap()
    .lines()
    .map(|line| line.split(',').map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>())
    .collect();

  let p1 = updates
    .par_iter()
    .filter(|update| {
      let page_pos = update.iter().enumerate().fold(HashMap::new(), |mut acc, (idx, page)| {
        acc.insert(*page, idx);
        acc
      });

      rules.iter().all(|r| r.valid(&page_pos))
    })
    .map(|u| u[(u.len() - 1) / 2])
    .sum();

  let p2 = updates
    .par_iter()
    .filter(|update| {
      let page_pos = update.iter().enumerate().fold(HashMap::new(), |mut acc, (idx, page)| {
        acc.insert(*page, idx);
        acc
      });
      rules.iter().any(|r| !r.valid(&page_pos))
    })
    .map(|u| {
      let mut update = u.clone();
      update.sort_by(|a, b| {
        if let Some(o) = rules.iter().map(|r| r.order(*a, *b)).skip_while(|o| o.is_none()).next() {
          o.unwrap()
        } else {
          Ordering::Equal
        }
      });
      update[(update.len() - 1) / 2]
    })
    .sum();

  let p2_alt = updates
    .par_iter()
    .filter_map(|u| {
      let mut update = u.clone();
      let mut page_pos = update.iter().enumerate().fold(HashMap::new(), |mut acc, (idx, page)| {
        acc.insert(*page, idx);
        acc
      });

      let mut invalid = false;

      'outer: loop {
        for r in rules.iter() {
          if !r.valid(&page_pos) {
            let (a, b) = r.get_pos(&page_pos).unwrap();
            update.swap(a, b);
            page_pos = update.iter().enumerate().fold(HashMap::new(), |mut acc, (idx, page)| {
              acc.insert(*page, idx);
              acc
            });

            invalid = true;
            continue 'outer;
          }
        }
        break;
      }

      invalid.then(|| update[(update.len() - 1) / 2])
    })
    .sum();

  assert_eq!(p2, p2_alt);

  (p1, p2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    assert_eq!((143, 123), solve(input.trim()));
  }
}
