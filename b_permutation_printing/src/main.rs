//{"name":"B. Permutation Printing","group":"Codeforces - think-cell Round 1","url":"https://codeforces.com/contest/1930/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4\n3\n","output":"4 1 2 3\n1 2 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPermutationPrinting"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut ans = vec![];
    let mut odd = n - 1 + n % 2;
    let mut even = 2;
    let mut is_odd = true;

    for i in 0..n {
        if is_odd {
            ans.push(odd);
            if odd >= 2 {
                odd -= 2;
            }
        } else {
            ans.push(even);
            even += 2;
        }
        is_odd = !is_odd;
    }

    // for i in 0..(n - 1) {
    //     for j in 0..(n - 1) {
    //         if i == j {
    //             continue;
    //         }
    //         if ans[i] % ans[j] == 0 && ans[i + 1] % ans[j + 1] == 0 {
    //             out.print_line(("BAD", i, j));
    //         }
    //     }
    // }

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
