//{"name":"F. Level Generation","group":"Codeforces - Educational Codeforces Round 24","url":"https://codeforces.com/contest/818/problem/F","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n3\n4\n6\n","output":"2\n3\n6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FLevelGeneration"}}}

use std::cmp::max;

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    // bigraph with n verticies.
    // at least half are bridges
    // maximise edges.

    // isn't it just clique with leaves?
    // apparently not?

    if n == 1 {
        out.print_line(0);
        return;
    }

    // maybe just need pack a knapsack? have (edges, verticies) pairs

    let edges = |x: usize| x * (x - 1) / 2;

    let mut clique = n / 2;
    let mut bridges = (n + 1) / 2;

    while edges(clique) > bridges {
        clique -= 1;
        bridges += 1;
    }

    let mut old_edges = edges(clique);
    // out.print_line(("OLD EDGES, BRIDGES", old_edges, bridges));
    for i in 0..300 {
        if old_edges + edges(2) + 2 <= bridges {
            let mut new_clique = 2;
            bridges -= 2;
            while bridges > 0 && old_edges + edges(new_clique + 1) <= bridges - 1 {
                new_clique += 1;
                bridges -= 1;
            }
            old_edges += edges(new_clique) + new_clique;
        }
    }

    let mut ans0 = old_edges + bridges;

    if n <= 10 {
        out.print_line(ans0);
        return;
    }

    // many small cliques is better than few large ones...
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
