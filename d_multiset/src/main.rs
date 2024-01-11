//{"name":"D. Multiset","group":"Codeforces - Educational Codeforces Round 87 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1354/D","interactive":false,"timeLimit":1500,"tests":[{"input":"5 5\n1 2 3 4 5\n-1 -1 -1 -1 -1\n","output":"0\n"},{"input":"5 4\n1 2 3 4 5\n-5 -1 -3 -1\n","output":"3\n"},{"input":"6 2\n1 1 1 2 3 4\n5 6\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMultiset"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_int_vec(n + q);

    let mut b = vec![0; 1001];

    for &q in a.iter() {
        if q > 0 {
            b[(q / 1000) as usize] += 1;
        } else {
            let mut q = (-q) as usize;
            for i in 0..=1000 {
                if q <= b[i] {
                    b[i] -= 1;
                    break;
                }
                q -= b[i];
            }
        }
    }
    let mut bucket = 1001;
    for i in 0..=1000 {
        if b[i] > 0 {
            bucket = i;
            break;
        }
    }
    if bucket == 1001 {
        out.print_line(0);
        return;
    }

    let mut smaller = 0;
    let mut relevant = vec![0; 1000];
    for &q in a.iter() {
        if q > 0 {
            let ord = (q / 1000) as usize;
            if ord < bucket {
                smaller += 1;
            } else if ord == bucket {
                relevant[(q % 1000) as usize] += 1;
            }
        } else {
            let mut q = -q;
            if q <= smaller {
                smaller -= 1;
                continue;
            }
            q -= smaller;
            for i in 0..1000 {
                if q <= relevant[i] {
                    relevant[i] -= 1;
                    break;
                }
                q -= relevant[i];
            }
        }
    }
    for i in 0..1000 {
        if relevant[i] > 0 {
            out.print_line(bucket * 1000 + i);
            return;
        }
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
