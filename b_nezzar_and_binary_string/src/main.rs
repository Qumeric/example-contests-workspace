//{"name":"B. Nezzar and Binary String","group":"Codeforces - Codeforces Round 698 (Div. 1)","url":"https://codeforces.com/problemset/problem/1477/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5 2\n00000\n00111\n1 5\n1 3\n2 1\n00\n01\n1 2\n10 6\n1111111111\n0110001110\n1 10\n5 9\n7 10\n1 7\n3 5\n6 10\n5 2\n10000\n11000\n2 5\n1 3\n","output":"YES\nNO\nYES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BNezzarAndBinaryString"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::specs::AssignSum;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

// TODO I misread the statement
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let s = input.read_str();
    let s = s
        .into_iter()
        .map(|c| if c == b'0' { 0 } else { 1 })
        .collect_vec();
    let f = input.read_str();
    let f = f
        .into_iter()
        .map(|c| if c == b'0' { 0 } else { 1 })
        .collect_vec();

    let mut tree = StaticArq::<AssignSum>::new(&s);

    for i in 0..q {
        let l = input.read1();
        let r = input.read1();

        let len = r - l + 1;
        let sum = tree.query(l, r) as usize;

        if sum * 2 == len {
            out.print_line("BAD SUM");
            out.print_line("NO");
            return;
        } else if sum * 2 < len {
            tree.update(l, r, &0);
        } else {
            tree.update(l, r, &1);
        }
    }

    for i in 0..n {
        if tree.query_point(i) != f[i] {
            out.print_line("NO");
            return;
        }
    }
    out.print_line("YES");
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
