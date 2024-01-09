//{"name":"E. Masha-forgetful","group":"Codeforces - Codeforces Round 764 (Div. 3)","url":"https://codeforces.com/problemset/problem/1624/E","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n\n4 8\n12340219\n20215601\n56782022\n12300678\n12345678\n\n2 3\n134\n126\n123\n\n1 4\n1210\n1221\n\n4 3\n251\n064\n859\n957\n054\n\n4 7\n7968636\n9486033\n4614224\n5454197\n9482268\n","output":"3\n1 4 1\n5 6 2\n3 4 3\n-1\n2\n1 2 1\n2 3 1\n-1\n3\n1 3 2\n5 6 3\n3 4 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMashaForgetful"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    
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
