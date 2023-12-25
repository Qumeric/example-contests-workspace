//{"name":"Static Range Minimum Queries","group":"CSES - CSES Problem Set","url":"https://cses.fi/problemset/task/1647","interactive":false,"timeLimit":1000,"tests":[{"input":"8 4\n3 2 4 5 1 1 5 3\n2 4\n5 6\n1 8\n3 3\n","output":"2\n1\n1\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"StaticRangeMinimumQueries"}}}

use algo_lib::collections::specs::AssignMin;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let q: usize = input.read();
    let x: Vec<i64> = input.read_vec(n);

    let mut rsq = StaticArq::<AssignMin>::new(&x);

    for _ in 0..q {
        let l: usize = input.read();
        let r: usize = input.read();
        out.print_line(rsq.query(l - 1, r - 1));
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
}
//END MAIN
