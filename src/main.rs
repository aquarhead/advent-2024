#![allow(dead_code)]

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

fn main() {
  let content = std::fs::read_to_string("inputs/day16.txt").expect("read file");
  let (p1, p2) = day16::solve(content.trim());

  println!("part1: {}", p1);
  println!("part2: {}", p2);
}
