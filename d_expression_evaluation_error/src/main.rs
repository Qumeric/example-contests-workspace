//{"name":"D. Expression Evaluation Error","group":"Codeforces - Codeforces Round 742 (Div. 2)","url":"https://codeforces.com/problemset/problem/1567/D","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n97 2\n17 1\n111 4\n100 2\n10 9\n999999 3\n","output":"70 27\n17\n3 4 100 4\n10 90\n1 1 2 1 1 1 1 1 1\n999900 90 9\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DExpressionEvaluationError"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};

type PreCalc = ();

fn calc(mut v: Vec<usize>) -> usize {
    let mut max = 1_000_000_000; // 1e9
    let mut tw = 2357947691; // 11^9

    let mut ans = 0;
    while max > 0 {
        for i in 0..v.len() {
            while v[i] >= max {
                v[i] -= max;
                ans += tw;
            }
        }
        max /= 10;
        tw /= 11;
    }

    ans
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_size();
    let n = input.read_size();

    let mut ans = 0;
    let mut ans_vec = vec![];
    for ones in 0..n {
        let mut vec = vec![];
        let mut solve = RecursiveFunction2::new(|f, s, n| {
            if s == 0 {
                return;
            }
            if n == 0 {
                vec.push(s);
                return;
            }
            let mut max = 1_000_000_000;
            while max > s {
                max /= 10;
            }
            vec.push(max);
            f.call(s - max, n - 1);
        });
        solve.call(s - ones, n - ones);

        // let mut vec = vec.clone();

        while vec.len() > n {
            let last = vec.pop().unwrap();
            let x = vec.len() - 1;
            vec[x] += last;
        }

        let mut rem = ones;
        while vec.len() < n && rem > 0 {
            vec.push(1);
            rem -= 1;
        }
        vec[0] += rem;
        if vec.len() != n {
            continue;
        }
        let val = calc(vec.clone());
        if val > ans {
            ans = val;
            ans_vec = vec;
        }
    }
    out.print_line(ans_vec);
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
