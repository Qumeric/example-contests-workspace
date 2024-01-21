//{"name":"F. Alex's whims","group":"Codeforces - Codeforces Round 909 (Div. 3)","url":"https://codeforces.com/contest/1899/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3 3\n2\n2\n2\n5 6\n4\n2\n3\n4\n3\n2\n4 9\n2\n3\n3\n2\n2\n2\n3\n2\n2\n","output":"1 2\n2 3\n-1 -1 -1\n-1 -1 -1\n-1 -1 -1\n1 2\n2 3\n3 4\n4 5\n-1 -1 -1\n4 3 2\n5 4 3\n4 2 5\n4 5 2\n5 3 4\n1 2\n2 3\n3 4\n4 3 2\n4 2 3\n-1 -1 -1\n4 3 2\n-1 -1 -1\n-1 -1 -1\n4 2 3\n4 3 2\n-1 -1 -1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FAlexsWhims"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();

    // have tree. Can remove one edge and insert another keeping it tree (if wanted)
    // need to have some leafes at distance d_i after each day

    for i in 1..n {
        out.print_line((i, i + 1));
    }

    // (connected, free)
    let mut palka = (n - 1, n);

    for i in 0..q {
        let d = input.read_size();
        if d == palka.0 {
            out.print_line((-1, -1, -1));
            continue;
        }
        out.print_line((palka.1, palka.0, d));
        palka.0 = d;
    }
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
