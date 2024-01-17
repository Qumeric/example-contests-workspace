//{"name":"D. Prefixes and Suffixes","group":"Codeforces - Codeforces Round 246 (Div. 2)","url":"https://codeforces.com/problemset/problem/432/D","interactive":false,"timeLimit":1000,"tests":[{"input":"ABACABA\n","output":"3\n1 4\n3 2\n7 1\n"},{"input":"AAA\n","output":"3\n1 3\n2 2\n3 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DPrefixesAndSuffixes"}}}

use algo_lib::collections::specs::{AssignSum, PlusSum};
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;
use algo_lib::string::string_algorithms::prefix_function::PrefixFunction;
use algo_lib::string::string_algorithms::z_algorithm::ZAlgorithm;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let s = input.read_str();
    let n = s.len();

    let pf = s.prefix_function();
    let zf = s.z_algorithm();
    out.print_line(&pf);
    out.print_line(&zf);

    let mut st = StaticArq::<PlusSum>::new(&vec![0; n]);

    for i in 0..n {
        if pf[i] > 0 {
            st.update(0, pf[i] - 1, &1);
        }
    }

    for i in 0..n {
        if i + zf[i] == n {
            out.print_line((zf[i], st.query_point(zf[i] - 1)));
        }
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
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
