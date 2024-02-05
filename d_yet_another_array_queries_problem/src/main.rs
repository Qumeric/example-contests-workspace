//{"name":"D. Yet Another Array Queries Problem","group":"Codeforces - Educational Codeforces Round 29","url":"https://codeforces.com/contest/863/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6 3 5\n1 2 3 4 5 6\n2 1 3\n2 3 6\n1 1 6\n2 2 1 5 3\n","output":"3 3 1 5 2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DYetAnotherArrayQueriesProblem"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let qq = input.read_size();
    let m = input.read_size();
    let a = input.read_long_vec(n);
    let q = (0..qq)
        .map(|_| (input.read_size(), input.read1(), input.read1()))
        .collect_vec();
    let mut b = input.read_size_vec(m).map(|x| x - 1).collect_vec();

    for &(t, l, r) in q.rev() {
        let mut new_b = vec![];
        for e in b {
            if e < l || e > r {
                new_b.push(e);
                continue;
            }
            // Cycling to the left cause order is reversed
            if t == 1 {
                if e == l {
                    new_b.push(r);
                } else {
                    new_b.push(e - 1);
                }
            } else {
                new_b.push(l + r - e);
            }
        }
        b = new_b;
    }
    out.print_line(b.map(|&x| a[x]).collect_vec());
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
