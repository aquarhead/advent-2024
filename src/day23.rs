use itertools::Itertools;
use pathfinding::undirected::connected_components::connected_components;
use std::collections::HashSet;

pub fn solve(input: &str) -> (usize, String) {
  let conns: HashSet<_> = input.lines().map(|line| line.split_once('-').unwrap()).collect();
  let nodes: Vec<_> = conns.iter().flat_map(|(a, b)| [*a, *b]).unique().collect();

  let p1 = nodes
    .iter()
    .combinations(3)
    .filter(|g| {
      let a = *g[0];
      let b = *g[1];
      let c = *g[2];
      (conns.contains(&(a, b)) || conns.contains(&(b, a)))
        && (conns.contains(&(a, c)) || conns.contains(&(c, a)))
        && (conns.contains(&(b, c)) || conns.contains(&(c, b)))
        && (a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
    })
    .count();

  let mut p2 = connected_components(&nodes, |n| {
    conns
      .iter()
      .filter_map(|(a, b)| {
        if a == n {
          Some(b)
        } else if b == n {
          Some(a)
        } else {
          None
        }
      })
      .cloned()
      .collect::<Vec<_>>()
  })
  .into_iter()
  .map(|party| {
    party
      .iter()
      .powerset()
      .reduce(|best, cur| {
        if cur.len() <= best.len() {
          best
        } else if cur
          .iter()
          .combinations(2)
          .all(|p| conns.contains(&(p[0], p[1])) || conns.contains(&(p[1], p[0])))
        {
          cur
        } else {
          best
        }
      })
      .unwrap()
      .into_iter()
      .map(|x| x.to_string())
      .collect::<Vec<_>>()
  })
  .max_by_key(|x| x.len())
  .unwrap();

  p2.sort();

  (p1, p2.into_iter().join(","))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    assert_eq!((7, "co,de,ka,ta".to_string()), solve(input.trim()));
  }
}
