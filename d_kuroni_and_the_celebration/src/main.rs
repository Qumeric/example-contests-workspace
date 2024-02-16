//{"name":"D. Kuroni and the Celebration","group":"Codeforces - Ozon Tech Challenge 2020 (Div.1 + Div.2, Rated, T-shirts + prizes!)","url":"https://codeforces.com/contest/1305/problem/D","interactive":true,"timeLimit":1000,"tests":[{"input":"6\n1 4\n4 2\n5 3\n6 3\n2 3\n\n3\n\n4\n\n4\n","output":"\n\n\n\n\n? 5 6\n\n? 3 1\n\n? 1 2\n\n! 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DKuroniAndTheCelebration"}}}

use std::collections::BTreeSet;
use std::io::Write;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();

    let mut g = vec![vec![]; n];
    for i in 1..n {
        let a = input.read1();
        let b = input.read1();
        g[a].push(b);
        g[b].push(a);
    }

    let mut deg = vec![0i64; n];
    for i in 0..n {
        deg[i] = g[i].len() as i64;
    }

    let mut q = BTreeSet::<usize>::new();

    for i in 0..n {
        if deg[i] == 1 {
            q.insert(i);
        }
    }

    while q.len() > 1 {
        let a = q.pop_first().unwrap();
        let b = q.pop_first().unwrap();

        for &u in &g[a] {
            deg[u] -= 1;
            if deg[u] == 1 {
                q.insert(u);
            }
        }
        for &u in &g[b] {
            deg[u] -= 1;
            if deg[u] == 1 {
                q.insert(u);
            }
        }
        out.print_line(("?", a + 1, b + 1));
        out.flush();
        let res = input.read_long();
        if res == -1 {
            return;
        }
        let res = (res - 1) as usize;
        if res == a || res == b {
            out.print_line(("!", res + 1));
            out.flush();
            return;
        }
    }
    out.print_line(("!", q.pop_first().unwrap() + 1));
    out.flush();
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
