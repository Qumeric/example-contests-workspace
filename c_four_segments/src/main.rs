//{"name":"C. Four Segments","group":"Codeforces - Educational Codeforces Round 28","url":"https://codeforces.com/contest/846/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"3\n-1 2 3\n","output":"0 1 3\n"},{"input":"4\n0 0 -1 0\n","output":"0 0 0\n"},{"input":"1\n10000\n","output":"1 1 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFourSegments"}}}

use algo_lib::collections::specs::{ArqSpec, AssignMax};
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

pub enum MaxMax {}
impl ArqSpec for MaxMax {
    type S = (i64, usize, usize);
    type F = (i64, usize, usize);
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a.max(b)
    }
    fn identity() -> Self::S {
        (0, 0, 0)
    }
    fn compose(&f: &Self::F, &g: &Self::F) -> Self::F {
        f.max(g)
    }
    fn apply(&f: &Self::F, &s: &Self::S, _: i64) -> Self::S {
        f.max(s)
    }
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_long_vec(n);

    // half-segment [l; r)

    // split on four parts (could be empty) such that
    // A - B + C - D is maximal

    let mut tree = StaticArq::<MaxMax>::new(&vec![(0, 0, 0); n]);

    for i in 0..n {
        let mut cur = 0;
        for j in i..=n {
            tree.update_point(i, &(cur, i, j));
            if j < n {
                cur += a[j];
            }
        }
    }

    let mut ans_val = i64::MIN;
    let mut ans = (0, 0, 0);

    let mut cur = 0;
    for i in 0..n {
        let q = tree.query(i, n - 1);
        let cur_val = cur + q.0;
        if cur_val > ans_val {
            ans_val = cur_val;
            ans = (i, q.1, q.2);
        }
        cur += a[i];
    }
    out.print_line(ans);
    // out.print_line(ans_val);
    // let mut fin_val = 0;
    // for i in 0..ans.0 {
    //     fin_val += a[i];
    // }
    // for i in ans.0..ans.1 {
    //     fin_val -= a[i];
    // }
    // for i in ans.1..ans.2 {
    //     fin_val += a[i];
    // }
    // for i in ans.2..n {
    //     fin_val -= a[i];
    // }
    // out.print_line(fin_val);
    // for i in 0..n {
    //     out.print_line(tree.query_point(i));
    // }
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
