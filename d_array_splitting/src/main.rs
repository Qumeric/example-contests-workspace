//{"name":"D. Array Splitting","group":"Codeforces - Educational Codeforces Round 66 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1175/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5 2\n-1 -2 5 -4 8\n","output":"15\n"},{"input":"7 6\n-3 0 -1 -2 -2 -4 -1\n","output":"-45\n"},{"input":"4 1\n3 -1 6 0\n","output":"8\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DArraySplitting"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let mut a = input.read_long_vec(n);
    a.reverse();
    let mut s = 0;
    let mut set = vec![];
    for e in a.iter().cloned().take(n - 1) {
        s += e;
        set.push(s);
    }
    set.sort();
    set.reverse();
    // out.print_line(&set);
    let mut ans: i64 = a.clone().into_iter().sum();

    for i in 0..(k - 1) {
        ans += set[i];
    }

    out.print_line(ans);
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
