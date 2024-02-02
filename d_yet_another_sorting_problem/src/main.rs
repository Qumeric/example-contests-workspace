//{"name":"D. Yet Another Sorting Problem","group":"Codeforces - Codeforces Round 759 (Div. 2, based on Technocup 2022 Elimination Round 3)","url":"https://codeforces.com/problemset/problem/1591/D","interactive":false,"timeLimit":2000,"tests":[{"input":"7\n1\n1\n2\n2 2\n2\n2 1\n3\n1 2 3\n3\n2 1 3\n3\n3 1 2\n4\n2 1 4 3\n","output":"YES\nYES\nNO\nYES\nNO\nYES\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DYetAnotherSortingProblem"}}}

use algo_lib::collections::specs::AssignSum;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut a_clone = a.clone();
    a_clone.sort();
    a_clone.dedup();
    if a_clone.len() != a.len() {
        out.print_line(true);
        return;
    }

    let mut inversions = 0;
    let mut seg_tree = StaticArq::<AssignSum>::new(&vec![0; n + 1]);

    for &value in &a {
        inversions += seg_tree.query(value + 1, n);
        seg_tree.update_point(value, &1);
    }

    out.print_line(inversions % 2 == 0);
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
