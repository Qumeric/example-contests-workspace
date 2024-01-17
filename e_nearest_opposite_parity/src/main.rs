//{"name":"E. Nearest Opposite Parity","group":"Codeforces - Codeforces Round 605 (Div. 3)","url":"https://codeforces.com/problemset/problem/1272/E","interactive":false,"timeLimit":2000,"tests":[{"input":"10\n4 5 7 6 7 5 4 4 6 4\n","output":"1 1 1 2 -1 1 1 3 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ENearestOppositeParity"}}}

use std::collections::VecDeque;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    let mut evens = VecDeque::new();
    let mut odds = VecDeque::new();
    for i in 0..n {
        if a[i] % 2 == 0 {
            evens.push_back((i, 0i64));
        } else {
            odds.push_back((i, 0i64));
        }
    }

    let mut edges = vec![vec![]; n];

    for i in 0..n {
        let x = i as i64 - a[i];
        let y = i as i64 + a[i];

        if x >= 0 {
            edges[x as usize].push(i);
        }
        if y < n as i64 {
            edges[y as usize].push(i);
        }
    }

    let mut ans = vec![-1; n];
    let mut dist1 = vec![-1; n];
    let mut dist2 = vec![-1; n];

    while let Some((i, d)) = evens.pop_front() {
        if dist1[i] != -1 {
            continue;
        }
        dist1[i] = d;

        for &to in edges[i].iter() {
            evens.push_back((to, d + 1));
        }
    }

    while let Some((i, d)) = odds.pop_front() {
        if dist2[i] != -1 {
            continue;
        }
        dist2[i] = d;

        for &to in edges[i].iter() {
            odds.push_back((to, d + 1));
        }
    }

    // out.print_line(&dist1);
    // out.print_line(&dist2);

    for i in 0..n {
        if a[i] % 2 == 0 {
            ans[i] = dist2[i];
        } else {
            ans[i] = dist1[i];
        }
    }

    out.print_line(ans);

    /*
    It's just multisource bfs.
    Start from all even and find distance to each odd.
    Same for starting with odd. Not much edges (2n - 2) so it's fine.


     */
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
