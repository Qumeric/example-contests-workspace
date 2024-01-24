//{"name":"B. Johnny and Grandmaster","group":"Codeforces - Codeforces Round 647 (Div. 1) - Thanks, Algo Muse!","url":"https://codeforces.com/problemset/problem/1361/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 2\n2 3 4 4 3\n3 1\n2 10 1000\n4 5\n0 1 1 100\n1 8\n89\n","output":"4\n1\n146981438\n747093407\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BJohnnyAndGrandmaster"}}}

use std::collections::BTreeMap;

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::{ModInt7, ModInt9};
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

// Not quite right, see solution
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let p = input.read_size();

    let k = input.read_int_vec(n);
    if p == 1 {
        out.print_line(n % 2);
        return;
    }

    let mut map: BTreeMap<i32, usize> = Default::default();
    for i in k {
        map.entry(i).and_modify(|x| *x += 1).or_insert(1);
    }
    let mut set = vec![];
    for (k, v) in map {
        if v % 2 == 1 {
            set.push(k);
        }
    }
    set.sort();
    // out.print_line(&set);
    if set.is_empty() {
        out.print_line(0);
        return;
    }
    let p = ModInt7::new(p as i32);
    let mut result = p.power(set.pop().unwrap());
    for e in set {
        result -= p.power(e);
    }
    out.print_line(result);
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
