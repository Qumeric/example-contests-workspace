//{"name":"D. Fill The Bag","group":"Codeforces - Educational Codeforces Round 82 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1303/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n10 3\n1 32 1\n23 4\n16 1 4 1\n20 5\n2 1 16 1 8\n","output":"2\n-1\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DFillTheBag"}}}

use core::num;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut numbers = vec![0; 35];
    let a = input.read_size_vec(m);
    for i in &a {
        let x = i.trailing_zeros() as usize;
        numbers[x] += 1usize;
    }

    if a.into_iter().sum::<usize>() < n {
        out.print_line(-1);
        return;
    }

    // 10 is 0101
    // out.print_line(&numbers);

    // 20 is 10111

    // out.print_line(("n", n));
    let mut ans = 0;
    for i in 0..33 {
        if (n & (1 << i)) > 0 {
            // out.print_line(("bit match", i));
            if numbers[i] > 0 {
                numbers[i] -= 1;
            } else {
                let mut found = false;
                for j in i + 1..33 {
                    if numbers[j] > 0 {
                        numbers[j] -= 1;
                        for k in i + 1..j {
                            numbers[k] += 1;
                        }
                        found = true;
                        ans += j - i;
                        break;
                    }
                }
                if !found {
                    out.print_line(-1);
                    return;
                }
                // do stuff
            }
        }
        numbers[i + 1] += numbers[i] / 2;
        numbers[i] = 0;
        // out.print_line(&numbers);
    }
    out.print_line(ans);
    // out.print_line("");
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
