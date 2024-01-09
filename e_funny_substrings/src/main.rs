//{"name":"E. Funny Substrings","group":"Codeforces - Codeforces Round 725 (Div. 3)","url":"https://codeforces.com/problemset/problem/1538/E","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n6\na := h\nb := aha\nc = a + b\nc = c + c\ne = c + c\nd = a + c\n15\nx := haha\nx = x + x\nx = x + x\nx = x + x\nx = x + x\nx = x + x\nx = x + x\nx = x + x\nx = x + x\nx = x + x\nx = x + x\nx = x + x\nx = x + x\nx = x + x\nx = x + x\n1\nhaha := hah\n5\nhaahh := aaaha\nahhhh = haahh + haahh\nhaahh = haahh + haahh\nahhhh = ahhhh + haahh\nahhaa = haahh + ahhhh\n","output":"3\n32767\n0\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EFunnySubstrings"}}}

use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

#[derive(Clone, Debug, Copy)]
pub enum S {
    Full(String),
    Compressed { prefix: String, suffix: String },
}

#[derive(Clone, Debug, Copy)]
pub struct Val {
    val: usize,
    s: S,
}

// TODO: this is not done, need a way to merge suff and pref
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let mut memory = BTreeMap::<String, Val>::new();

    let mut last = 0;

    for i in 0..n {
        let str = input.read_line().to_string();
        let parts: Vec<&str> = str.split_whitespace().collect();

        match parts.clone().as_slice() {
            [var, ":=", value] => {
                let count_ha = value.matches("haha").count();
                let s = if value.len() < 10 {
                    S::Full(value.to_string())
                } else {
                    S::Compressed {
                        prefix: value[0..5],
                        suffix: value[value.len() - 5..value.len()],
                    }
                };

                memory.insert(var.to_string(), Val { val: count_ha, s });
            }

            [result, "=", operand1, "+", operand2] => {
                let val1 = memory.get(&operand1.to_string()).unwrap();
                let val2 = memory.get(&operand2.to_string()).unwrap();
                let mut val = val1.val + val2.val;
                if val1.ends_with_h && val2.statrs_with_a {
                    val += 1;
                }
                last = val;
                memory.insert(
                    result.to_string(),
                    Val {
                        val,
                        ends_with_h: val2.ends_with_h,
                        statrs_with_a: val1.statrs_with_a,
                    },
                );
            }

            _ => unimplemented!("wtf bro"),
        }
    }
    out.print_line(last);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = ();

    #[allow(dead_code)]
    enum TestType {
        Single,
        MultiNumber,
        MultiEof,
    }
    let test_type = TestType::MultiNumber;
    match test_type {
        TestType::Single => solve(&mut input, &mut output, 1, &pre_calc),
        TestType::MultiNumber => {
            let t = input.read();
            for i in 0usize..t {
                solve(&mut input, &mut output, i + 1, &pre_calc);
            }
        }
        TestType::MultiEof => {
            let mut i = 1;
            while input.peek().is_some() {
                solve(&mut input, &mut output, i, &pre_calc);
                i += 1;
            }
        }
    }
    output.flush();
    input.skip_whitespace();
    input.peek().is_none()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
