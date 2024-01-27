//{"name":"E. Space Harbour","group":"Codeforces - Codeforces Round 921 (Div. 2)","url":"https://codeforces.com/contest/1925/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"8 3 4\n1 3 8\n3 24 10\n2 2 5\n1 5 15\n2 5 5\n2 7 8\n","output":"171\n0\n15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ESpaceHarbour"}}}

use algo_lib::collections::specs::{ArqSpec, PlusSum};
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::collections::treap_map::TreapSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let q = input.read_size();
    let x = input.read_size_vec(m);
    let v = input.read_size_vec(m);

    let mut harbours = vec![];

    let mut tree = StaticArq::<PlusSum>::new(&vec![0; n + 1]);
    let mut set = TreapSet::new();
    for i in 0..n {
        harbours.push((x[i], v[i]));
        set.insert((x[i], v[i]));
    }
    harbours.sort();
    let mut ptr = 0;
    for i in 1..n {
        if harbours[ptr + 1].0 < i {
            ptr += i;
        }
        if harbours[ptr].0 == i {
            continue;
        }
        let val = harbours[ptr].1 * (harbours[ptr + 1].0 - i);
        tree.update_point(i, &(val as i64));
    }

    for i in 0..q {
        let qu = input.read_size();
        if qu == 1 {
            let x = input.read_size();
            let v = input.read_size();
            let prev_harbour = set.floor(&(x, v)).unwrap();
            let prev_pos = prev_harbour.0;
            let prev_val = prev_harbour.1;

            out.print_line(("for v", v, "prev", prev_pos));

            set.insert((x, v));
        } else {
            let l = input.read_size();
            let r = input.read_size();
            let val = tree.query(l, r);
            out.print_line(val);
        }
    }

    // when we add harbour between A and B
    // val for A:
    // * -= V[PREV] * (X[B] - X[new])
    // * =  V[PREV] * (X[new] - X[A])
    // val for new = V[A] * (X[B] - X[new])

    // so total change is (V[A] * (X[B] - X[new])) - V[PREV] * (X[B] - X[new])
    // (V[A] - V[PREV]) * (X[B] - X[new])
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
