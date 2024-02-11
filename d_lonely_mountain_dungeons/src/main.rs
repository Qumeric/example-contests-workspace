//{"name":"D. Lonely Mountain Dungeons","group":"Codeforces - Codeforces Round 924 (Div. 2)","url":"https://codeforces.com/contest/1928/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n3 1 0\n1 2 3\n3 5 10\n2 5 3\n4 3 3\n3 2 1 2\n4 1 0\n4 1 4 2\n4 1 10\n4 1 4 2\n","output":"4\n40\n9\n13\n0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DLonelyMountainDungeons"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn c2(n: usize) -> usize {
    if n <= 1 {
        return 0;
    }
    n * (n - 1) / 2
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let b = input.read_size();
    let x = input.read_size();
    let c = input.read_size_vec(n);

    // 2 same in different squads
    // k squads -(k - 1) * x
    // n races

    let calc = |k, out: &mut Output| {
        if k == 1 {
            return 0;
        }
        let penalty = (k - 1) * x;

        let mut cur_ans = 0;
        for &t in &c {
            let small_box_size = t / k;
            let big_box_size = small_box_size + 1;
            let big_boxes = t % k;
            let small_boxes = k - big_boxes;
            let max_value = c2(t);
            let small_penalty = small_boxes * c2(small_box_size);
            let big_penalty = big_boxes * c2(big_box_size);
            let value = max_value - small_penalty - big_penalty;
            let boost = value * b;
            cur_ans += boost;
        }
        // out.print_line((k, cur_ans, penalty));

        if cur_ans >= penalty {
            cur_ans - penalty
        } else {
            0
        }
    };

    let mut left = 1;
    let mut right = 200_200; // Max creatures of kin:w
    while right - left > 2 {
        let m1 = left + (right - left) / 3;
        let m2 = right - (right - left) / 3;
        let f1 = calc(m1, out);
        let f2 = calc(m2, out);
        if f1 < f2 {
            left = m1;
        } else {
            right = m2;
        }
    }
    let mut max_value = 0;
    for k in left..=right {
        max_value = max_value.max(calc(k, out));
    }
    out.print_line(max_value);

    // // test
    // for k in 2..10 {
    //     let ans = calc(k, out);
    //     if ans >= 0 {
    //         out.print_line(ans);
    //     }
    //     out.print_line("----");
    // }
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
