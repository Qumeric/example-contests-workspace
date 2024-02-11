//{"name":"D. Anti-Sudoku","group":"Codeforces - Codeforces Round 634 (Div. 3)","url":"https://codeforces.com/contest/1335/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"1\n154873296\n386592714\n729641835\n863725149\n975314628\n412968357\n631457982\n598236471\n247189563\n","output":"154873396\n336592714\n729645835\n863725145\n979314628\n412958357\n631457992\n998236471\n247789563\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DAntiSudoku"}}}

use std::vec;

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::Shuffle;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut s = (0..9)
        .map(|_| {
            input
                .read_str()
                .into_iter()
                .map(|x| (x - b'0') as usize)
                .collect_vec()
        })
        .collect_vec();

    let order = vec![
        (0, 0),
        (3, 1),
        (6, 2),
        (1, 3),
        (4, 4),
        (7, 5),
        (2, 6),
        (5, 7),
        (8, 8),
    ];
    // let squares = order.map(|&(i, j)| s[i][j]).collect_vec();

    for &(i, j) in &order {
        s[i][j] += 1;
        if s[i][j] == 10 {
            s[i][j] = 1;
        }
    }
    for row in s {
        for e in row {
            out.print(e);
        }
        out.print_line("");
    }
    out.print_line("");
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
