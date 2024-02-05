//{"name":"D. Multicolored Cars","group":"Codeforces - Educational Codeforces Round 24","url":"https://codeforces.com/contest/818/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4 1\n2 1 4 2\n","output":"2\n"},{"input":"5 2\n2 2 4 5 3\n","output":"-1\n"},{"input":"3 10\n1 2 3\n","output":"4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMulticoloredCars"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size();
    let c = input.read_size_vec(n);

    let mut colors = vec![vec![]; 1_000_001];

    for i in 0..n {
        colors[c[i]].push(i);
    }

    let mut ptrs = vec![0; 1_000_001];

    let mut alive_colors = BTreeSet::<usize>::new();

    for x in c {
        if x != a {
            alive_colors.insert(x);
        }
    }

    for &pos in &colors[a] {
        let mut dead_colors = vec![];
        for &color in &alive_colors {
            let ptr = ptrs[color];
            if ptr >= colors[color].len() || colors[color][ptr] > pos {
                dead_colors.push(color);
            }
            ptrs[color] += 1;
        }
        for color in dead_colors {
            alive_colors.remove(&color);
        }
    }

    if alive_colors.is_empty() {
        out.print_line(-1);
    } else {
        out.print_line(alive_colors.pop_first().unwrap());
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
