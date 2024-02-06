//{"name":"G. MinOr Tree","group":"Codeforces - Codeforces Round 764 (Div. 3)","url":"https://codeforces.com/problemset/problem/1624/G","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n\n3 3\n1 2 1\n2 3 2\n1 3 2\n\n5 7\n4 2 7\n2 5 8\n3 4 2\n3 2 1\n2 4 2\n4 1 2\n1 2 2\n\n3 4\n1 2 1\n2 3 2\n1 3 3\n3 1 4\n","output":"2\n10\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GMinOrTree"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut bitmask = (1 << 30) - 1;

    let edges = (0..m)
        .map(|_| (input.read1(), input.read1(), input.read_size()))
        .collect_vec();

    let mut check = |max| {
        let mut dsu = DSU::new(n);
        for &(a, b, w) in &edges {
            if w & max == w {
                dsu.join(a, b);
            }
        }
        // out.print_line((max, "Size", dsu.size(0)));
        dsu.size(0) == n
    };

    for bit in (0..30).rev() {
        let bit = 1 << bit;
        bitmask ^= bit;
        if !check(bitmask) {
            bitmask ^= bit;
        }
    }

    out.print_line(bitmask);
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
