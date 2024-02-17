//{"name":"A. Возможно, вы знаете этих людей?","group":"Codeforces - VK Cup 2015 - Round 1","url":"https://codeforces.com/contest/524/problem/A","interactive":false,"timeLimit":2000,"tests":[{"input":"5 51\n10 23\n23 42\n39 42\n10 39\n39 58\n","output":"10: 1 42\n23: 1 39\n39: 1 23\n42: 1 10\n58: 2 10 42\n"},{"input":"5 100\n1 2\n1 3\n1 4\n2 3\n2 4\n","output":"1: 0\n2: 0\n3: 1 4\n4: 1 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AVozmozhnoViZnaeteEtikhLyudei"}}}

use std::collections::{BTreeMap, BTreeSet};

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let m = input.read_size();
    let k = input.read_size();

    let mut g = BTreeMap::<usize, Vec<usize>>::new();

    for _ in 0..m {
        let a = input.read_size();
        let b = input.read_size();

        g.entry(a).or_insert_with(Vec::new).push(b);
        g.entry(b).or_insert_with(Vec::new).push(a);
    }

    let keys = g.keys().clone();

    for &h in keys {
        let mut fs = vec![];
        let mut ffs = BTreeMap::<usize, BTreeSet<usize>>::new();
        for (x, y) in &g {
            ffs.entry(*x).or_insert_with(BTreeSet::new);
        }
        for &f in &g[&h] {
            fs.push(f);
            for &ff in &g[&f] {
                ffs.entry(ff).or_insert_with(BTreeSet::new).insert(f);
            }
        }

        let mut ans = vec![];
        for (id, v) in ffs {
            let v = v.len();
            if id == h || fs.contains(&id) {
                continue;
            }
            if 100 * v >= k * fs.len() {
                ans.push(id);
            }
        }

        out.print(format!("{}: {} ", h, ans.len()));
        out.print_line(ans);
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
