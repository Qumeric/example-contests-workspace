//{"name":"G. Unusual Entertainment","group":"Codeforces - Codeforces Round 909 (Div. 3)","url":"https://codeforces.com/contest/1899/problem/G","interactive":false,"timeLimit":3000,"tests":[{"input":"3\n3 5\n1 2\n2 3\n1 2 3\n1 2 2\n1 2 3\n2 3 1\n1 2 3\n2 3 3\n10 10\n2 6\n2 7\n2 4\n1 7\n2 8\n10 6\n8 5\n9 4\n3 4\n10 2 5 9 1 7 6 4 3 8\n8 9 8\n7 8 1\n7 10 6\n4 8 9\n5 5 10\n7 10 1\n9 9 2\n9 10 6\n6 6 2\n10 10 6\n1 1\n1\n1 1 1\n","output":"YES\nNO\nYES\nNO\nYES\n\nNO\nYES\nYES\nYES\nNO\nYES\nYES\nNO\nNO\nNO\n\nYES\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"GUnusualEntertainment"}}}

use algo_lib::collections::treap_map::TreapSet;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();

    let mut edges = vec![];
    for i in 1..n {
        let u = input.read1();
        let v = input.read1();

        let edge = (u, v);
        edges.push(edge);
    }

    let mut p = vec![];
    for i in 0..n {
        p.push(input.read1());
    }

    let mut tree_vals = vec![0; n];

    for i in 0..n {
        tree_vals[p[i]] = i + 1;
    }

    let mut queries = vec![vec![]; n];

    for i in 0..q {
        let l = input.read_size();
        let r = input.read_size();
        let x = input.read1();

        queries[x].push((l, r, i));
    }

    let mut sets = vec![];
    for i in 0..n {
        sets.push(TreapSet::new());
    }

    let mut g: Vec<Vec<usize>> = vec![vec![]; n];
    for (a, b) in edges {
        g[a].push(b);
        g[b].push(a);
    }

    let mut ans = vec![false; q];

    let mut dfs = RecursiveFunction2::new(|f, v: usize, p| {
        for &edge in g[v].iter() {
            if Some(edge) != p {
                f.call(edge, Some(v));
            }
        }

        if g[v].is_empty() || (g[v].len() == 1 && p.is_some()) {
            sets[v].insert(tree_vals[v]);
        } else {
            let mut big_child: Option<usize> = None;
            for &edge in g[v].iter() {
                if Some(edge) != p {
                    if let Some(bc) = big_child {
                        if sets[edge].len() > sets[bc].len() {
                            big_child = Some(edge);
                        }
                    } else {
                        big_child = Some(edge);
                    }
                }
            }
            let mut cur_set = std::mem::take(&mut sets[big_child.unwrap()]);
            cur_set.insert(tree_vals[v]);
            for &edge in g[v].iter() {
                if Some(edge) == big_child {
                    continue;
                }
                for &item in sets[edge].iter() {
                    cur_set.insert(item);
                }
            }
            sets[v] = cur_set;
        }

        for &(l, r, i) in queries[v].iter() {
            let pos = sets[v].less_or_eq(&r);
            if pos == 0 {
                continue;
            }
            let prev = sets[v].get_at(pos - 1);

            if let Some(&prev) = prev {
                ans[i] = prev >= l && prev <= r;
            }
        }
    });

    dfs.call(0, None);

    for i in ans {
        out.print_line(i);
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
