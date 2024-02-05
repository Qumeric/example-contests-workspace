//{"name":"B. Permutation Game","group":"Codeforces - Educational Codeforces Round 24","url":"https://codeforces.com/contest/818/problem/B","interactive":false,"timeLimit":1000,"tests":[{"input":"4 5\n2 3 1 4 4\n","output":"3 1 2 4\n"},{"input":"3 3\n3 1 2\n","output":"-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BPermutationGame"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let l = input.read_size_vec(m);

    let mut b = vec![0; n];

    let mut used = vec![0; 101];

    for i in 0..(m - 1) {
        let lead = l[i];
        let steps = if l[i + 1] > l[i] {
            l[i + 1] - l[i]
        } else {
            n - l[i] + l[i + 1]
        };
        if b[lead - 1] != steps && b[lead - 1] != 0 {
            out.print_line(-1);
            return;
        }
        b[lead - 1] = steps;
        used[steps] = 1;
        // out.print_line(&b);
    }

    let mut ptr = 1;
    for i in 0..n {
        if b[i] != 0 {
            continue;
        }
        while used[ptr] == 1 {
            ptr += 1;
        }
        b[i] = ptr;
        used[ptr] = 1;
    }
    let mut bb = b.clone();
    bb.sort();
    bb.dedup();
    if bb.len() != n {
        out.print_line(-1);
        return;
    }
    out.print_line(b);

    // 2 3 1 4 4
    // 0 1 0 0
    // 0 1 2 0
    // 3 1 2 4
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
