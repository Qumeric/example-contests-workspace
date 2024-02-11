//{"name":"E. Tree Constructing","group":"Codeforces - Codeforces Round 494 (Div. 3)","url":"https://codeforces.com/contest/1003/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"6 3 3\n","output":"YES\n3 1\n4 1\n1 2\n5 2\n2 6\n"},{"input":"6 2 3\n","output":"NO\n"},{"input":"10 4 3\n","output":"YES\n2 9\n2 10\n10 3\n3 1\n6 10\n8 2\n4 3\n5 6\n6 7\n"},{"input":"8 5 3\n","output":"YES\n2 5\n7 2\n3 7\n3 1\n1 6\n8 7\n4 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ETreeConstructing"}}}

use std::cmp::min;
use std::collections::VecDeque;

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::graph::edges::bi_edge::BiEdge;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut n = input.read_size();
    let d = input.read_size();
    let k = input.read_size();

    if d >= n {
        out.print_line("NO");
        return;
    }

    let mut last_vertex = 1;
    let mut g = Graph::<BiEdge<()>>::new(n);

    let mut state = VecDeque::new();
    state.push_back((0, 0));
    n -= 1;
    let mut d_added = vec![0; 500_000];
    while n > 0 {
        let (p_depth, cur) = state.pop_front().unwrap();

        let from = if cur == 0 { k } else { k - 1 };

        let mut d_now = p_depth * 2;
        if d_added[p_depth] == 1 {
            d_now += 1;
        } else if d_added[p_depth] > 1 {
            d_now += 2;
        }

        for i in 0..min(from, n) {
            // out.print_line(("PRED", d_now, n, d, cur, last_vertex));
            if d_now + n == d {
                // out.print_line((d_now, n, d, cur, last_vertex));
                g.add_edge(BiEdge::new(cur, last_vertex));
                for _ in 1..n {
                    g.add_edge(BiEdge::new(last_vertex, last_vertex + 1));
                    last_vertex += 1;
                }
                n = 0;
                break;
            } else {
                g.add_edge(BiEdge::new(cur, last_vertex));
                state.push_back((p_depth + 1, last_vertex));
                last_vertex += 1;
                n -= 1;

                if d_added[p_depth] <= 1 && i == 0 {
                    d_now += 1;
                    d_added[p_depth] += 1;
                }
            }
        }
    }

    let real_d = g.find_diameter_path().len();
    if real_d > d + 1 {
        out.print_line("NO");
        return;
    }

    out.print_line("YES");
    for (from, to_vec) in g.into_egdes().enumerate() {
        for to in to_vec {
            if from < to.to() {
                out.print_line((from + 1, to.to() + 1));
            }
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
