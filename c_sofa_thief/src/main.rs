//{"name":"C. Sofa Thief","group":"Codeforces - Educational Codeforces Round 24","url":"https://codeforces.com/contest/818/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n3 2\n3 1 3 2\n1 2 2 2\n1 0 0 1\n","output":"1\n"},{"input":"3\n10 10\n1 2 1 1\n5 5 6 5\n6 4 5 4\n2 1 2 0\n","output":"2\n"},{"input":"2\n2 2\n2 1 1 1\n1 2 2 2\n1 0 0 0\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSofaThief"}}}

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let d = input.read_size();
    let n = input.read_size();
    let m = input.read_size();

    let mut dumb_x = vec![];
    let mut dumb_y = vec![];

    let mut sofas = vec![];

    for i in 0..d {
        let x1 = input.read_size();
        let y1 = input.read_size();
        let x2 = input.read_size();
        let y2 = input.read_size();
        dumb_x.push(x1);
        dumb_y.push(y1);
        sofas.push((x1, y1, x2, y2));
    }
    dumb_x.sort();
    dumb_y.sort();

    let cnt_l = input.read_size();
    let cnt_r = input.read_size();
    let cnt_t = input.read_size();
    let cnt_b = input.read_size();

    let check =
        |(x, y): (usize, usize), (x_init, y_init)| (x < x_init, x > x_init, y < y_init, y > y_init);

    for i in 0..d {
        let sofa = &sofas[i];
        let left = dumb_x.lower_bound(&sofa.0);
        let top = dumb_y.lower_bound(&sofa.1);

        if left.abs_diff(cnt_l) < 5 && top.abs_diff(cnt_t) < 5 {
            let mut c_l = cnt_l as i64;
            let mut c_r = cnt_r as i64;
            let mut c_t = cnt_t as i64;
            let mut c_b = cnt_b as i64;
            for j in 0..d {
                if i == j {
                    continue;
                }
                let other = &sofas[j];
                let a = check((other.0, other.1), (sofa.0, sofa.1));
                let b = check((other.2, other.3), (sofa.0, sofa.1));
                let c = check((other.0, other.1), (sofa.2, sofa.3));
                let d = check((other.2, other.3), (sofa.2, sofa.3));
                let result = (
                    a.0 || b.0 || c.0 || d.0,
                    a.1 || b.1 || c.1 || d.1,
                    a.2 || b.2 || c.2 || d.2,
                    a.3 || b.3 || c.3 || d.3,
                );
                if result.0 {
                    c_l -= 1;
                }
                if result.1 {
                    c_r -= 1;
                }
                if result.2 {
                    c_t -= 1;
                }
                if result.3 {
                    c_b -= 1;
                }
            }
            if c_l == 0 && c_r == 0 && c_t == 0 && c_b == 0 {
                out.print_line(i + 1);
                return;
            }
        }
    }
    out.print_line(-1);

    // if sofa the same on line they are conidered both on the top and on the bottom

    //
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
