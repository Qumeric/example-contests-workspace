//{"name":"C. Medium Design","group":"Codeforces - Codeforces Round 904 (Div. 2)","url":"https://codeforces.com/contest/1884/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"6\n1 3\n2 2\n3 8\n2 4\n3 5\n4 6\n6 3\n1 1\n1 2\n1 3\n2 2\n2 3\n3 3\n7 6\n2 2\n1 6\n1 2\n5 6\n1 5\n4 4\n3 6\n6 27\n6 26\n5 17\n2 3\n20 21\n1 22\n12 24\n4 1000000000\n2 999999999\n3 1000000000\n123456789 987654321\n9274 123456789\n","output":"1\n3\n2\n3\n4\n4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMediumDesign"}}}

use std::cmp::{max, min};

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::specs::AssignSum;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: i64 = input.read();
    let m: i64 = input.read();

    let mut segs: Vec<(i64, i64)> = vec![];
    let mut ops: Vec<(i64, i64, i64, i64)> = vec![];

    for _ in 0..n {
        let l: i64 = input.read();
        let r: i64 = input.read();
        segs.push((l, r));
        ops.push((l, -1, l, r));
        ops.push((r, 1, l, r));
    }

    segs.sort();

    ops.sort();

    let mut cnt = 0;
    let mut maxval = 0;

    for &(pos, op, l, _) in ops.iter() {
        if l == 1 {
            continue;
        }
        cnt -= op;
        if cnt > maxval {
            maxval = cnt;
        }
    }

    let ans_l = maxval;
    // out.print_line("without left");
    // out.print_line(ans_l);

    let mut cnt = 0;
    let mut maxval = 0;

    for (pos, op, _, r) in ops {
        if r == m {
            continue;
        }
        cnt -= op;
        if cnt > maxval {
            maxval = cnt;
        }
    }

    out.print_line(max(ans_l, maxval));

    // out.print_line((maxpos, maxval));
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
}
//END MAIN
