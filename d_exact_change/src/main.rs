//{"name":"D. Exact Change","group":"Codeforces - Educational Codeforces Round 119 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1620/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1\n1337\n3\n10 8 10\n5\n1 2 3 4 5\n3\n7 77 777\n","output":"446\n4\n3\n260\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DExactChange"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut ans = 1_000_000_000;
    for ones in 0..4 {
        for twos in 0..4 {
            let mut cur_ans = 0;
            for &e in &a {
                let mut elem_ans = 1_000_000_000;
                for cur_ones in 0..=ones {
                    for cur_twos in 0..=twos {
                        if e >= cur_ones + cur_twos * 2 {
                            let left = e - cur_ones - cur_twos * 2;
                            if left % 3 == 0 {
                                elem_ans.minim(ones + twos + left / 3);
                            }
                        }
                    }
                }
                cur_ans.maxim(elem_ans);
            }
            ans.minim(cur_ans);
        }
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
