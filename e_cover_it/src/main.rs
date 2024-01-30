//{"name":"E. Cover it!","group":"Codeforces - Codeforces Round 565 (Div. 3)","url":"https://codeforces.com/contest/1176/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n4 6\n1 2\n1 3\n1 4\n2 3\n2 4\n3 4\n6 8\n2 5\n5 4\n4 3\n4 1\n1 3\n2 3\n2 6\n5 6\n","output":"2\n1 3\n3\n4 3 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECoverIt"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::Shuffle;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let edges = (0..m).map(|_| (input.read1(), input.read1())).collect_vec();
    let mut g = vec![vec![]; n];

    for &(a, b) in &edges {
        g[a].push(b);
        g[b].push(a);
    }

    while true {
        let mut v = (0..n).collect_vec();
        v.shuffle();

        let mut covered = vec![false; n];
        let mut c = 0;
        let mut ans = vec![];
        for i in 0..n {
            if covered[v[i]] {
                continue;
            }
            c += 1;
            covered[v[i]] = true;
            ans.push(v[i] + 1);
            if ans.len() > n / 2 {
                break;
            }
            for &u in g[v[i]].iter() {
                if covered[u] == false {
                    covered[u] = true;
                    c += 1;
                }
            }
        }
        if c == n && ans.len() <= n / 2 {
            out.print_line(ans.len());
            out.print_line(ans);
            break;
        }
    }

    // bigraph n m
    // vertex cover everything with at most half verticies

    // if n is even then just select verticies if they are not covered
    // otherwise intuitively it feels we can sort (from high to low by degree)
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
