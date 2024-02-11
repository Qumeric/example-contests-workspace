//{"name":"D. Coins and Queries","group":"Codeforces - Codeforces Round 494 (Div. 3)","url":"https://codeforces.com/contest/1003/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n2 4 8 2 4\n8\n5\n14\n10\n","output":"1\n-1\n3\n2\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DCoinsAndQueries"}}}

use std::cmp::min;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let q = input.read_size();
    let a = input.read_size_vec(n);
    let mut coins = vec![0; 35];
    for e in a {
        let mut power = 0;
        while (1 << power) <= e {
            if e & (1 << power) != 0 {
                coins[power] += 1;
                break;
            }
            power += 1;
        }
    }
    // out.print_line(&coins);
    

    for _ in 0..q {
        let mut b = input.read_long();

        let mut ans = 0;
        for i in (0..35).rev() {
            let i = i as i64;
            let sz = 1i64 << i;
            let can_take = min(coins[i as usize], b / sz);
            b -= can_take * sz;
            ans += can_take;
        }
        if b == 0 {
            out.print_line(ans);
        } else {
            out.print_line(-1);
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
