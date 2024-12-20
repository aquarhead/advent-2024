use itertools::Itertools;
use std::collections::HashMap;
use std::ops::BitXor;

#[derive(Debug, Clone, Default)]
struct Computer {
  reg_a: u64,
  reg_b: u64,
  reg_c: u64,
  ip: usize,
  output: Vec<u8>,
}

impl Computer {
  fn run_program(&mut self, prog: &Vec<u8>) {
    while let Some(opcode) = prog.get(self.ip) {
      let operand = prog.get(self.ip + 1).unwrap();

      self.step(*opcode, *operand);
    }
  }

  fn recur(&mut self, prog: &Vec<u8>) -> bool {
    while let Some(opcode) = prog.get(self.ip) {
      let operand = prog.get(self.ip + 1).unwrap();
      self.step(*opcode, *operand);

      if self.output.len() > prog.len() || self.output.iter().zip(prog.iter()).any(|(x, y)| x != y) {
        return false;
      }
    }

    self.output.len() == prog.len() && self.output.iter().zip(prog.iter()).all(|(x, y)| x == y)
  }

  fn step(&mut self, opcode: u8, operand: u8) {
    match opcode {
      0 => {
        // adv
        self.reg_a = self.reg_a / 2_u64.pow(self.combo(operand) as u32);
      }
      1 => {
        // bxl
        self.reg_b = self.reg_b.bitxor(operand as u64);
      }
      2 => {
        // bst
        self.reg_b = self.combo(operand) % 8;
      }
      3 => {
        // jnz
        if self.reg_a != 0 {
          self.ip = operand as usize;
          return;
        }
      }
      4 => {
        // bxc
        self.reg_b = self.reg_b.bitxor(self.reg_c);
      }
      5 => {
        // out

        self.output.push((self.combo(operand) % 8) as u8);
      }
      6 => {
        // bdv
        self.reg_b = self.reg_a / 2_u64.pow(self.combo(operand) as u32);
      }
      7 => {
        // cdv
        self.reg_c = self.reg_a / 2_u64.pow(self.combo(operand) as u32);
      }
      _ => panic!("bad opcode"),
    }

    self.ip += 2;
  }

  fn combo(&self, operand: u8) -> u64 {
    match operand {
      0..=3 => operand as u64,
      4 => self.reg_a,
      5 => self.reg_b,
      6 => self.reg_c,
      _ => panic!("bad operand"),
    }
  }
}

pub fn solve(input: &str) -> (String, u64) {
  let mut lines = input.lines();
  let reg_a = lines.next().unwrap()[12..].parse().unwrap();
  let reg_b = lines.next().unwrap()[12..].parse().unwrap();
  let reg_c = lines.next().unwrap()[12..].parse().unwrap();
  lines.next();
  let program = lines.next().unwrap()[9..]
    .split(',')
    .map(|x| x.parse().unwrap())
    .collect();

  let p1 = {
    let mut comp = Computer {
      reg_a,
      reg_b,
      reg_c,
      ip: 0,
      output: Vec::new(),
    };
    comp.run_program(&program);
    comp.output.into_iter().join(",")
  };

  let p2 = {
    let vec_to_input = |v: &Vec<u8>| -> u64 {
      let j: Vec<_> = v.iter().map(|o| format!("{:03b}", o)).collect();

      u64::from_str_radix(&j.join(""), 2).unwrap()
    };

    let mut search: Vec<Vec<u8>> = vec![Vec::new()];
    for ml in 1..=program.len() {
      let mut new_search = Vec::new();
      for s in search {
        for n in 0..=7 {
          let mut t = s.clone();
          t.push(n);
          let mut comp = Computer {
            reg_a: vec_to_input(&t),
            ..Default::default()
          };
          comp.run_program(&program);
          if comp.output.len() == ml
            && program
              .iter()
              .rev()
              .take(ml)
              .rev()
              .zip(comp.output)
              .all(|(x, y)| *x == y)
          {
            new_search.push(t);
          }
        }
      }
      search = new_search
    }

    search.iter().map(|s| vec_to_input(s)).min().unwrap()
  };

  (p1, p2)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let mut comp1 = Computer {
      reg_a: 729,
      reg_b: 0,
      reg_c: 0,
      ip: 0,
      output: Vec::new(),
    };
    comp1.run_program(&vec![0, 1, 5, 4, 3, 0]);
    assert_eq!(vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0], comp1.output);
  }

  #[test]
  fn test2() {
    let input = "
Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    assert_eq!(117440, solve(input.trim()).1);
  }
}
