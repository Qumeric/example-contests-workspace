//{"name":"B. Mike and Feet","group":"Codeforces - Codeforces Round 305 (Div. 1)","url":"https://codeforces.com/contest/547/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n1 2 3 4 5 4 3 2 1 6\n","output":"6 4 4 3 3 2 2 1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BMikeAndFeet"}}}

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    let mut aa = a
        .clone()
        .into_iter()
        .enumerate()
        .map(|(i, e)| (e, i))
        .collect_vec();

    aa.sort();
    aa.reverse();

    let mut used = vec![false; n];

    let mut dsu = DSU::new(n);

    let mut longest = 0;
    for (e, i) in aa {
        used[i] = true;
        if i > 0 && used[i - 1] {
            dsu.join(i, i - 1);
        }
        if i < n - 1 && used[i + 1] {
            dsu.join(i, i + 1);
        }
        let len = dsu.size(i);
        while longest < len {
            longest += 1;
            out.print(&e);
            out.print(" ");
        }
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
