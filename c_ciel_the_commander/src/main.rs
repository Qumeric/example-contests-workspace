//{"name":"C. Ciel the Commander","group":"Codeforces - Codeforces Round 190 (Div. 1)","url":"https://codeforces.com/problemset/problem/321/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1 2\n1 3\n1 4\n","output":"A B B B\n"},{"input":"10\n1 2\n2 3\n3 4\n4 5\n5 6\n6 7\n7 8\n8 9\n9 10\n","output":"D C B A D C B D C D\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CCielTheCommander"}}}

use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// TODO: this is incorrect. Counterexample is bamboo. But isn't it centroid decomposition though?
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read();

    let edges: Vec<_> = (1..n).map(|_| (input.read1(), input.read1())).collect();
    let tree = Graph::from_biedges(n, &edges);

    let diameter = tree.find_diameter_path();
    let d_vert = diameter[0];

    let mut ans = vec![0; n];

    let mut ls: Vec<u8> = vec![];

    let mut bottom = b'A';
    while bottom <= b'Z' {
        let mut top = b'Z';
        while top >= bottom {
            ls.push(top);
            top -= 1;
        }
        bottom += 1;
    }

    // out.print_line(ls.len());
    out.print_line(d_vert);

    // 1 Y
    // 2 X
    // 3 X
    // 4 Z

    ans[d_vert] = 1;

    let mut impossible = false;
    tree.dfs(d_vert, |p, v| {
        if ans[v] == 0 {
            let p = p.unwrap();
            if ans[p] == ls.len() {
                impossible = true;
                return;
            }
            ans[v] = ans[p] + 1
        }
    });

    if impossible {
        out.print_line("Impossible");
    } else {
        out.print_iter(ans.into_iter().map(|c| ls[351 - c] as char));
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
