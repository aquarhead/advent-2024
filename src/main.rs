#![allow(dead_code)]

mod day01;

fn main() {
  let content = std::fs::read_to_string("inputs/day01.txt").expect("read file");
  let (p1, p2) = day01::solve(&content);

  println!("part1: {}", p1);
  println!("part2: {}", p2);
}
