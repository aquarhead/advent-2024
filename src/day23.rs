use itertools::Itertools;
use std::collections::HashSet;

pub fn solve(input: &str) -> (usize, String) {
  let conns: HashSet<_> = input.lines().map(|line| line.split_once('-').unwrap()).collect();
  let nodes: Vec<_> = conns
    .iter()
    .flat_map(|(a, b)| [a.to_string(), b.to_string()])
    .unique()
    .collect();

  let mut lans: Vec<_> = nodes
    .iter()
    .cloned()
    .combinations(3)
    .filter(|g| {
      g.iter()
        .combinations(2)
        .all(|c| conns.contains(&(c[0], c[1])) || conns.contains(&(c[1], c[0])))
    })
    .collect();

  let p1 = lans.iter().filter(|g| g.iter().any(|c| c.starts_with('t'))).count();

  while lans.len() > 1 {
    lans = lans
      .into_iter()
      .flat_map(|lan| {
        let mut ret = Vec::new();
        for n in nodes.iter() {
          if !lan.contains(n) && lan.iter().all(|x| conns.contains(&(n, x)) || conns.contains(&(x, n))) {
            let mut nl = lan.clone();
            nl.push(n.clone());
            nl.sort();
            ret.push(nl);
          }
        }
        ret
      })
      .unique()
      .collect();
  }

  let p2 = lans.get_mut(0).unwrap();

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
