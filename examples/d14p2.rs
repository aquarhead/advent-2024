type TwoD = (i32, i32);

fn main() {
  let input = std::fs::read_to_string("inputs/day14.txt").expect("read file");

  let bound = (101, 103);

  let mut robots: Vec<_> = input
    .lines()
    .map(|line| {
      let (ps, vs) = line.split_once(" v=").unwrap();
      let (px, py) = ps[2..].split_once(',').unwrap();
      let (vx, vy) = vs.split_once(',').unwrap();

      let p: TwoD = (px.parse().unwrap(), py.parse().unwrap());
      let v: TwoD = (vx.parse().unwrap(), vy.parse().unwrap());

      (p, v)
    })
    .collect();

  for step in 1.. {
    println!("{}", step);
    println!("----");
    robots.iter_mut().for_each(|(p, v)| {
      (*p).0 = (p.0 + v.0).rem_euclid(bound.0);
      (*p).1 = (p.1 + v.1).rem_euclid(bound.1);
    });

    for y in 0..bound.1 {
      for x in 0..bound.0 {
        if robots.iter().find(|(p, _)| p.0 == x && p.1 == y).is_some() {
          print!("x");
        } else {
          print!(" ");
        }
      }
      println!();
    }

    println!("----");

    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();
  }
}
