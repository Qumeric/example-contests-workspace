//{"name":"C. Three Bags","group":"Codeforces - Codeforces Round 695 (Div. 2)","url":"https://codeforces.com/problemset/problem/1467/C","interactive":false,"timeLimit":1000,"tests":[{"input":"2 4 1\n1 2\n6 3 4 5\n5\n","output":"20\n"},{"input":"3 2 2\n7 5 4\n2 9\n7 1\n","output":"29\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CThreeBags"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let a = input.read();
    let b = input.read();
    let c = input.read();
    let x = input.read_long_vec(a);
    let y = input.read_long_vec(b);
    let z = input.read_long_vec(c);

    let xx = *x.iter().min().unwrap();
    let yy = *y.iter().min().unwrap();
    let zz = *z.iter().min().unwrap();

    let xs = x.into_iter().sum::<i64>();
    let ys = y.into_iter().sum::<i64>();
    let zs = z.into_iter().sum::<i64>();

    let mut v = vec![xx, yy, zz];
    v.sort();

    let mut vv = vec![xs, ys, zs];
    vv.sort();

    let ans: i64 = xs + ys + zs - vv[0].min((v[0] + v[1])) * 2; // sum - 6

    out.print_line(ans);

    // 7 5 4
    // 2 9
    // 7 1

    // 4
    // (2 - 7 - 5 - 7) = -17
    // (1 - 9) = -8

    // 29
    // .
    // .

    // TODO: in some cases we can do better... apparently two mins could be in the same line

    // Case when do better (20 instead of 18),
    // 1 2
    // 6 3 4 5
    // 5

    // -10 2
    // 3 4
    // 5
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
