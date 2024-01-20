//{"name":"E. Connected Components?","group":"Codeforces - Educational Codeforces Round 37 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/920/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5 5\n1 2\n3 4\n3 2\n4 2\n2 5\n","output":"2\n1 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EConnectedComponents"}}}

use std::collections::BTreeMap;

use algo_lib::collections::dsu::DSU;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut dsu = DSU::new(n);

    let mut g: Vec<Vec<_>> = vec![vec![]; n];

    for i in 0..m {
        let v = input.read1();
        let u = input.read1();

        g[v].push(u);
        g[u].push(v);
    }

    for i in 0..n {
        g[i].sort();
    }

    let mut s = vec![];
    for i in 0..n {
        s.push((g[i].len(), i));
    }

    s.sort();
    s.reverse();

    for h in s.iter().take(1000) {
        let mut ptr = 0;
        for to in 0..n {
            if ptr >= g[h.1].len() || to < g[h.1][ptr] {
                if h.1 != to {
                    // out.print_line(("JOIN", h.1, to));
                    dsu.join(h.1, to);
                }
            } else {
                ptr += 1;
            }
        }
    }
    if s.len() > 1000 {
        let x = s[1000].1;
        for h in s.iter().skip(1000) {
            dsu.join(x, h.1);
        }
    }

    let mut ans: BTreeMap<usize, usize> = Default::default();

    for i in 0..n {
        ans.entry(dsu.get(i))
            .and_modify(|x| {
                *x += 1;
            })
            .or_insert(1);
    }

    out.print_line(ans.len());
    let mut anw = ans.values().collect_vec();
    anw.sort();
    out.print_line(anw);
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
