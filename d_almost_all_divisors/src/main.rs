//{"name":"D. Almost All Divisors","group":"Codeforces - Codeforces Round 560 (Div. 3)","url":"https://codeforces.com/contest/1165/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n8\n8 2 12 6 4 24 16 3\n1\n2\n","output":"48\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DAlmostAllDivisors"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut d = input.read_size_vec(n);

    d.sort();

    let num = d[0] * d.last().unwrap();

    for &e in &d {
        if num % e != 0 {
            out.print_line(-1);
            return;
        }
    }

    let mut real_divs = vec![];

    let mut i = 2;
    while i * i <= num {
        if num % i == 0 {
            real_divs.push(i);
            real_divs.push(num / i);
        }
        i += 1;
    }

    real_divs.sort();
    real_divs.dedup();

    // out.print_line(&real_divs);

    if real_divs != d {
        out.print_line(-1);
        return;
    }

    out.print_line(num);
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
