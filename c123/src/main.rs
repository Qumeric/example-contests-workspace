//{"name":"C. 1-2-3","group":"Codeforces - Educational Codeforces Round 29","url":"https://codeforces.com/contest/863/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"10 2 1\n1 1 1\n1 1 1\n1 1 1\n2 2 2\n2 2 2\n2 2 2\n","output":"1 9\n"},{"input":"8 1 1\n2 2 1\n3 3 1\n3 1 3\n1 1 1\n2 1 1\n1 2 3\n","output":"5 2\n"},{"input":"5 1 1\n1 2 2\n2 2 2\n2 2 2\n1 2 2\n2 2 2\n2 2 2\n","output":"0 0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C123"}}}

use std::cmp::min;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut k = input.read_size();
    let a = input.read_size();
    let b = input.read_size();

    let mut games = vec![(a, b)];

    let mut alice = vec![];
    for i in 0..3 {
        alice.push([input.read_size(), input.read_size(), input.read_size()]);
    }
    let mut bob = vec![];
    for i in 0..3 {
        bob.push([input.read_size(), input.read_size(), input.read_size()]);
    }

    let mut pos = 0;
    while true {
        let (prev_a, prev_b) = *games.last().unwrap();

        let new_a = alice[prev_a - 1][prev_b - 1];
        let new_b = bob[prev_a - 1][prev_b - 1];

        if games.contains(&(new_a, new_b)) {
            pos = games.iter().position(|&x| x == (new_a, new_b)).unwrap();
            break;
        } else {
            games.push((new_a, new_b));
        }
    }

    let mut start_score = (0, 0);
    let mut cycle_score = (0, 0);

    let play = |score: &mut (usize, usize), (a, b)| match (a, b) {
        (1, 1) | (2, 2) | (3, 3) => {}
        (1, 2) | (2, 3) | (3, 1) => score.1 += 1,
        (2, 1) | (3, 2) | (1, 3) => score.0 += 1,
        _ => panic!(),
    };

    for i in 0..min(k, pos) {
        k -= 1;
        play(&mut start_score, games[i]);
    }

    let cycle_len = games.len() - pos;
    for i in pos..games.len() {
        play(&mut cycle_score, games[i]);
    }

    let full_cycles = k / cycle_len;
    let rem = k % cycle_len;

    cycle_score.0 *= full_cycles;
    cycle_score.1 *= full_cycles;

    for i in pos..pos + rem {
        play(&mut cycle_score, games[i]);
    }

    let final_score = (start_score.0 + cycle_score.0, start_score.1 + cycle_score.1);

    // for g in &games {
    //     out.print_line(g);
    // }
    // out.print_line(("POS", pos));
    // out.print_line(("CYCLE", cycle_score));
    // out.print_line(("START", start_score));

    out.print_line(final_score);
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
