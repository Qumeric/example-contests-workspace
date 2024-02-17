//{"name":"D. Ehab and the Expected XOR Problem","group":"Codeforces - Codeforces Round 563 (Div. 2)","url":"https://codeforces.com/problemset/problem/1174/D","interactive":false,"timeLimit":1000,"tests":[{"input":"3 5\n","output":"3\n6 1 3\n"},{"input":"2 4\n","output":"3\n1 3 1\n"},{"input":"1 1\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DEhabAndTheExpectedXORProblem"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let x = input.read_size();

    let mut arr = vec![];
    if x >= (1 << n) {
        for i in (1..=n).rev() {
            for j in 0..i {
                arr.push(1 << j);
            }
        }
    } else {
        for i in (1..=(n - 1)).rev() {
            for j in 0..i {
                arr.push(1 << j);
            }
        }
        let bit = 1 << x.trailing_zeros();

        for i in 0..arr.len() {
            let small_part = arr[i] % bit;
            let big_part = arr[i] - small_part;
            arr[i] = small_part + big_part * 2;
        }
    }

    out.print_line(arr.len());
    out.print_line(&arr);
    // for i in 0..arr.len() {
    //     arr[i] ^= x;
    // }
    // 6
    // 1 2 4 1 2 1
    // 1 3 7 6 4 5 2 6 7 5 4 4 5 7 6 1 3 2 2 3 1
    // 21

    // let mut res = vec![];
    // for i in 0..arr.len() {
    //     let mut cur = 0;
    //     for j in i..arr.len() {
    //         cur ^= arr[j];
    //         res.push(cur);
    //     }
    // }
    // out.print_line(&res);
    // out.print_line(res.len());
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
