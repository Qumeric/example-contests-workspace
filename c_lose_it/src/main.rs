//{"name":"C. Lose it!","group":"Codeforces - Codeforces Round 565 (Div. 3)","url":"https://codeforces.com/contest/1176/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n4 8 15 16 23\n","output":"5\n"},{"input":"12\n4 8 4 15 16 8 23 15 16 42 23 42\n","output":"0\n"},{"input":"15\n4 8 4 8 15 16 8 16 23 15 16 4 42 23 42\n","output":"3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLoseIt"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);

    // k div by 6 and possible to split into k/6 subsequences 4, 8, 15, 16, 23, 42

    let mut pos = vec![];

    for i in 0..n {
        let v = match a[i] {
            4 => 0,
            8 => 1,
            15 => 2,
            16 => 3,
            23 => 4,
            42 => 5,
            _ => panic!(),
        };
        pos.push(v);
    }

    let mut cnts = vec![0; 6];
    let mut del = 0;

    for x in pos {
        if x == 0 {
            cnts[0] += 1;
        } else {
            if cnts[x - 1] > 0 {
                cnts[x - 1] -= 1;
                cnts[x] += 1;
            } else {
                del += 1;
            }
        }
    }
    for i in 0..5 {
        del += cnts[i] * (i + 1);
    }
    out.print_line(del);
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
