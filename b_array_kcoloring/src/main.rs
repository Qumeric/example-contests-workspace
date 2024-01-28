//{"name":"B. Array K-Coloring","group":"Codeforces - Codeforces Round 531 (Div. 3)","url":"https://codeforces.com/contest/1102/problem/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2\n1 2 2 3\n","output":"YES\n1 1 2 2\n"},{"input":"5 2\n3 2 1 2 3\n","output":"YES\n2 1 1 2 1\n"},{"input":"5 2\n2 1 1 2 1\n","output":"NO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BArrayKColoring"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    if n < k {
        out.print_line("NO");
        return;
    }

    let a = input.read_size_vec(n);

    let mut vec = vec![vec![]; 5001];

    for i in 0..n {
        vec[a[i]].push(i);
    }

    let mut ans = vec![0; n];

    let mut col = 0;
    for i in 0..=5000 {
        if vec[i].len() > k {
            out.print_line("NO");
            return;
        }
        for &j in vec[i].iter() {
            ans[j] = col + 1;
            col = (col + 1) % k;
        }
    }

    out.print_line("YES");
    out.print_line(ans);
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
