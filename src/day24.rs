enum Input {
  Wire(String),
  Value(bool),
}

type Gates = Vec<(String, String, String, String)>;

pub fn solve(input: &str) -> (usize, String) {
  let (init, gates_str) = input.split_once("\n\n").unwrap();
  let mut wires: Vec<_> = init
    .lines()
    .map(|line| {
      let (wire, value) = line.split_once(": ").unwrap();
      (wire.to_string(), value == "1")
    })
    .collect();

  let p1 = {
    let mut gates: Vec<_> = gates_str
      .lines()
      .map(|line| {
        let (input, output) = line.split_once(" -> ").unwrap();
        let mut input = input.split_ascii_whitespace();
        let a = Input::Wire(input.next().unwrap().to_string());
        let op = input.next().unwrap().to_string();
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
              let o = match op.as_str() {
                "AND" => *x & *y,
                "OR" => *x || *y,
                "XOR" => *x ^ *y,
                _ => panic!("invalid gate"),
              };
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
    zwires.into_iter().rev().fold(0, |acc, (_, v)| (acc << 1) + v as usize)
  };

  let p2 = {
    let mut gates: Vec<_> = gates_str
      .lines()
      .map(|line| {
        let (input, output) = line.split_once(" -> ").unwrap();
        let mut input = input.split_ascii_whitespace();
        let a = input.next().unwrap().to_string();
        let op = input.next().unwrap().to_string();
        let b = input.next().unwrap().to_string();
        (a, b, op, output.to_string())
      })
      .collect();

    let find = |g: &Gates, a: &str, b: &str, op0: &str| -> Option<String> {
      g.iter()
        .find(|(x, y, op, _)| op == op0 && ((x == a && y == b) || (y == a && x == b)))
        .map(|g| g.3.clone())
    };
    let find1 = |g: &Gates, a: &str, output: &str, op: &str| -> Option<String> {
      g.iter().find_map(|(x, y, op0, output0)| {
        if op == op0 && output == output0 {
          if x == a {
            return Some(y.clone());
          }
          if y == a {
            return Some(x.clone());
          }
        }
        None
      })
    };
    let find2 = |g: &Gates, a: &str, op: &str| -> Option<_> {
      g.iter().find_map(|(x, y, op0, output0)| {
        if op == op0 {
          if x == a {
            return Some((y.clone(), output0.clone()));
          }
          if y == a {
            return Some((x.clone(), output0.clone()));
          }
        }
        None
      })
    };

    let mut should_swap = Vec::new();
    let mut swap = |gates: &mut Gates, o1: String, o2: String| {
      should_swap.push(o1.clone());
      should_swap.push(o2.clone());
      gates.iter_mut().for_each(|(_, _, _, o)| {
        if o == &o1 {
          *o = o2.clone();
        } else if o == &o2 {
          *o = o1.clone();
        }
      });
    };

    {
      let z0 = find(&gates, "x00", "y00", "XOR").unwrap();
      if z0 != "z00" {
        swap(&mut gates, z0, "z00".to_string());
      }
    }

    let mut carry = find(&gates, "x00", "y00", "AND").unwrap();
    for n in 1..(init.lines().count() / 2) {
      let x = format!("x{:02}", n);
      let y = format!("y{:02}", n);
      let z = format!("z{:02}", n);
      let mut bit_xor = find(&gates, &x, &y, "XOR").unwrap();

      // bit_xor ^ carry = z_n
      if let Some(z_n) = find(&gates, &bit_xor, &carry, "XOR") {
        if z_n != z {
          swap(&mut gates, z.clone(), z_n.clone());
        }
      } else {
        // check bit_xor
        if let Some(bit_xor_check) = find1(&gates, &carry, &z, "XOR") {
          if bit_xor != bit_xor_check {
            swap(&mut gates, bit_xor.clone(), bit_xor_check.clone());
            bit_xor = bit_xor_check.clone();
          }
        }

        // check carry
        if let Some(carry_check) = find1(&gates, &bit_xor, &z, "XOR") {
          if carry != carry_check {
            swap(&mut gates, carry.clone(), carry_check.clone());
            carry = carry_check.clone();
          }
        }
      }

      // (bit_xor & carry) || bit_and = carry_next
      let mut bit_and = find(&gates, &x, &y, "AND").unwrap();
      let carry_next_left = find(&gates, &bit_xor, &carry, "AND").unwrap();
      if let Some(carry_next) = find(&gates, &carry_next_left, &bit_and, "OR") {
        carry = carry_next;
      } else {
        // check bit_and
        if let Some((bit_and_check, carry_next)) = find2(&gates, &carry_next_left, "OR") {
          if bit_and_check != bit_and {
            swap(&mut gates, bit_and.clone(), bit_and_check.clone());
            bit_and = bit_and_check.clone();
            carry = carry_next;
          }
        }

        // check left
        if let Some((cnl_check, carry_next)) = find2(&gates, &bit_and, "OR") {
          if cnl_check != carry_next_left {
            swap(&mut gates, carry_next_left.clone(), cnl_check.clone());
            carry = carry_next;
          }
        }
      }
    }

    // the last bit should be carry

    should_swap.sort();
    should_swap.join(",")
  };

  (p1, p2)
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

    assert_eq!(4, solve(input.trim()).0);
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

    assert_eq!(2024, solve(input.trim()).0);
  }
}
