//{"name":"C. Ice Cave","group":"Codeforces - Codeforces Round 301 (Div. 2)","url":"https://codeforces.com/problemset/problem/540/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4 6\nX...XX\n...XX.\n.X..X.\n......\n1 6\n2 2\n","output":"YES\n"},{"input":"5 4\n.X..\n...X\nX.X.\n....\n.XX.\n5 3\n1 1\n","output":"NO\n"},{"input":"4 7\n..X.XX.\n.XX..X.\nX...X..\nX......\n2 2\n1 6\n","output":"YES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CIceCave"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::graph::edges::edge_trait::EdgeTrait;
use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::RecursiveFunction4;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut f = (0..n).map(|_| input.read_str()).collect_vec();

    let r1 = input.read1();
    let c1 = input.read1();
    let r2 = input.read1();
    let c2 = input.read1();

    f[r1][c1] = b'.';

    let is_end_cracked = f[r2][c2] == b'X';
    f[r2][c2] = b'.';

    let mut edges = vec![];

    let vertex = |r, c| r * 1000 + c;

    let edge = |r1, c1, r2, c2| (vertex(r1, c1), vertex(r2, c2));

    for i in 0..n {
        for j in 0..m {
            if f[i][j] == b'X' {
                continue;
            }

            if i > 0 && f[i - 1][j] == b'.' {
                edges.push(edge(i - 1, j, i, j));
            }
            if j > 0 && f[i][j - 1] == b'.' {
                edges.push(edge(i, j - 1, i, j));
            }
        }
    }

    edges.sort();
    edges.dedup();

    let g = Graph::from_biedges(1000 * 1000, &edges);

    let start = vertex(r1, c1);
    let end = vertex(r2, c2);

    if start == end {
        out.print_line(g.edges[vertex(r1, c1)].len() > 0);
        return;
    }

    let path = g.find_path_between(start, end);

    if path.is_empty() {
        out.print_line(false);
        return;
    }

    // out.print_line("PATH FOUND");
    // for e in &g.edges[end] {
    //     out.print_line(e.to());
    // }

    out.print_line(is_end_cracked || g.edges[end].len() >= 2);
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
