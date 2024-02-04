//{"name":"C. Friends and Gifts","group":"Codeforces - Codeforces Round 611 (Div. 3)","url":"https://codeforces.com/contest/1283/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n5 0 0 2 4\n","output":"5 3 1 2 4\n"},{"input":"7\n7 0 0 1 4 0 6\n","output":"7 3 2 1 4 5 6\n"},{"input":"7\n7 4 0 3 0 5 1\n","output":"7 4 2 3 6 5 1\n"},{"input":"5\n2 1 0 0 0\n","output":"2 1 4 5 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFriendsAndGifts"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::random::Shuffle;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let f = input.read_size_vec(n);

    let given = BTreeSet::from_iter(f.iter());

    let mut not_given = vec![];

    for i in 1..=n {
        if !given.contains(&i) {
            not_given.push(i);
        }
    }

    while true {
        not_given.shuffle();
        let mut ff = f.clone();
        let mut ptr = 0;
        for i in 0..n {
            if ff[i] == 0 {
                ff[i] = not_given[ptr];
                ptr += 1;
            }
        }
        let mut good = true;
        for i in 0..n {
            if ff[i] == i + 1 {
                good = false;
                break;
            }
        }
        if good {
            out.print_line(ff);
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
