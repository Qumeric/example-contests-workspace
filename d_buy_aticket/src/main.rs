//{"name":"D. Buy a Ticket","group":"Codeforces - Educational Codeforces Round 38 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/938/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\n1 2 4\n2 3 7\n6 20 1 25\n","output":"6 14 1 25\n"},{"input":"3 3\n1 2 1\n2 3 1\n1 3 1\n30 10 20\n","output":"12 10 12\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBuyATicket"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::graph::distances::Distances;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut edges = (0..m)
        .map(|_| (input.read_size(), input.read_size(), input.read_size() * 2))
        .collect_vec();

    for i in 1..=n {
        edges.push((0, i, input.read_size()));
    }

    let g = Graph::from_weighted_biedges(n + 1, &edges);

    let ans = g.distances_from(0);

    for i in 1..=n {
        out.print_line(ans[i].unwrap().0);
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
