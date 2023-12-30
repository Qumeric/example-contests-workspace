//{"name":"D. Tree Tag","group":"Codeforces - Codeforces Round 668 (Div. 2)","url":"https://codeforces.com/problemset/problem/1405/D","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n4 3 2 1 2\n1 2\n1 3\n1 4\n6 6 1 2 5\n1 2\n6 5\n2 3\n3 4\n4 5\n9 3 9 2 5\n1 2\n1 6\n1 9\n1 3\n9 5\n7 9\n4 8\n4 3\n11 8 11 3 3\n1 2\n11 9\n4 9\n6 5\n2 10\n3 2\n5 9\n8 3\n7 4\n7 10\n","output":"Alice\nBob\nAlice\nAlice\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTreeTag"}}}

use std::cmp::max;

use algo_lib::graph::graph::Graph;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::zip;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let a: usize = input.read1();
    let b: usize = input.read1();
    let da: usize = input.read();
    let db: usize = input.read();

    let edges: Vec<(usize, usize)> = (1..n).map(|_| (input.read1(), input.read1())).collect();

    if db <= da * 2 {
        out.print_line("Alice");
        return;
    }

    let tree = Graph::from_biedges(n, &edges);

    let ab_path = tree.find_path_between(a, b);

    if ab_path.len() <= da + 1 {
        out.print_line("Alice");
        return;
    }
    let a = ab_path[da];

    let mut distances = vec![0; n];
    let mut is_in_subtree = vec![false; n];

    tree.dfs(b, |parent, v| {
        if parent.is_none() {
            return;
        }
        let parent = parent.unwrap();

        distances[v] = distances[parent] + 1;
        is_in_subtree[v] = is_in_subtree[parent] || v == a;
    });

    let diameter = tree.find_diameter_path();

    let diam_a = *diameter.first().unwrap();
    let diam_b = *diameter.last().unwrap();

    // This is actually redundant because we can prove that diameter is always accessible if
    // it's long enough (diameter.len() - 1 > da * 2). 
    // But it works too.
    let longest_accessible = if !is_in_subtree[diam_a] && !is_in_subtree[diam_b] {
        let mut distances_a = vec![0; n];
        tree.dfs(diam_a, |parent, v| {
            if parent.is_none() {
                return;
            }
            let parent = parent.unwrap();

            distances_a[v] = distances_a[parent] + 1;
        });
        let mut distances_b = vec![0; n];
        tree.dfs(diam_b, |parent, v| {
            if parent.is_none() {
                return;
            }
            let parent = parent.unwrap();

            distances_b[v] = distances_b[parent] + 1;
        });

        zip!(distances_a, distances_b)
            .enumerate()
            .map(|(i, (a, b))| if is_in_subtree[i] { max(a, b) } else { 1 })
            .max()
            .unwrap()
            - 1
    } else {
        diameter.len() - 1
    };

    if longest_accessible <= da * 2 {
        out.print_line("Alice");
    } else {
        out.print_line("Bob");
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
