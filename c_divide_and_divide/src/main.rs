//{"name":"C - Divide and Divide","group":"AtCoder - KAJIMA CORPORATION CONTEST 2024（AtCoder Beginner Contest 340）","url":"https://atcoder.jp/contests/abc340/tasks/abc340_c","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n","output":"5\n"},{"input":"340\n","output":"2888\n"},{"input":"100000000000000000\n","output":"5655884811924144128\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDivideAndDivide"}}}

use std::collections::BTreeMap;
use std::mem::needs_drop;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction, RecursiveFunction0};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let mut cur = BTreeMap::new();
    cur.insert(n, 1);

    let mut result = 0;
    let mut rec = RecursiveFunction::new(|f, map: BTreeMap<usize, usize>| {
        let mut new_map = BTreeMap::new();
        let mut did_anything = false;
        for (&k, &v) in &map {
            if k <= 1 {
                continue;
            }
            did_anything = true;
            result += k * v;
            let k1 = k / 2;
            let k2 = (k + 1) / 2;
            *new_map.entry(k1).or_insert(0) += v;
            *new_map.entry(k2).or_insert(0) += v;
        }
        if did_anything {
            f.call(new_map);
        }
    });

    rec.call(cur);

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
    let test_type = TestType::Single;
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
