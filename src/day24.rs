enum Input {
  Wire(String),
  Value(bool),
}

pub fn solve(input: &str) -> (usize, usize) {
  let (init, gates_str) = input.split_once("\n\n").unwrap();
  let mut wires: Vec<_> = init
    .lines()
    .map(|line| {
      let (wire, value) = line.split_once(": ").unwrap();
      (wire.to_string(), value == "1")
    })
    .collect();

  let mut gates: Vec<_> = gates_str
    .lines()
    .map(|line| {
      //
      let (input, output) = line.split_once(" -> ").unwrap();
      let mut input = input.split_ascii_whitespace();
      let a = Input::Wire(input.next().unwrap().to_string());
      let op = match input.next().unwrap() {
        "AND" => |a: bool, b: bool| -> bool { a & b },
        "OR" => |a: bool, b: bool| -> bool { a || b },
        "XOR" => |a: bool, b: bool| -> bool { a ^ b },
        _ => panic!("invalid gate"),
      };
      let b = Input::Wire(input.next().unwrap().to_string());
      (a, b, op, output.to_string())
    })
    .collect();

  let mut zwires = Vec::new();

  while let Some((w, v)) = wires.pop() {
    gates.iter_mut().for_each(|(a, b, op, out)| {
      let mut got_input = false;
      if let Input::Wire(x) = a {
        if *x == w {
          *a = Input::Value(v);
        }
        got_input = true;
      }
      if let Input::Wire(x) = b {
        if *x == w {
          *b = Input::Value(v);
        }
        got_input = true
      }
      if got_input {
        if let Input::Value(x) = a {
          if let Input::Value(y) = b {
            let o = op(*x, *y);
            wires.push((out.clone(), o));
            if out.starts_with('z') {
              zwires.push((out.clone(), o));
            }
          }
        }
      }
    });
  }

  zwires.sort();
  let p1 = zwires.into_iter().rev().fold(0, |acc, (_, v)| (acc << 1) + v as usize);

  (p1, 0)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_example() {
    let input = "
x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

    assert_eq!((4, 0), solve(input.trim()));
  }

  #[test]
  fn test2() {
    let input = "
x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

    assert_eq!((2024, 0), solve(input.trim()));
  }
}
