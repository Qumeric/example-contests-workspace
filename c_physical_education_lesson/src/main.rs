//{"name":"C. Physical Education Lesson","group":"Codeforces - Codeforces Round 924 (Div. 2)","url":"https://codeforces.com/contest/1928/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n10 2\n3 1\n76 4\n100 99\n1000000000 500000000\n","output":"4\n1\n9\n0\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPhysicalEducationLesson"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size() - 1;
    let x = input.read_size() - 1;

    // 2k - 2 is div of this
    let mult1 = n - x;
    let mult2 = x + n;

    let mut divs = vec![];

    // 10 2
    // 8 -- 2, 4, 8 -- 2, 3, 5
    // 10 -- 5, 10 -- 6

    let mut i = 1;
    while i * i <= mult1 {
        if mult1 % i == 0 {
            divs.push(i);
            divs.push(mult1 / i);
        }
        i += 1;
    }
    i = 1;
    while i * i <= mult2 {
        if mult2 % i == 0 {
            divs.push(i);
            divs.push(mult2 / i);
        }
        i += 1;
    }

    divs.sort();
    divs.dedup();

    // out.print_line(&divs);

    let mut ans = 0;
    for i in divs {
        if i % 2 == 1 {
            continue;
        }
        let k = i / 2;
        // out.print_line(k);
        if k >= 1 && k >= x {
            ans += 1;
        }
        // 2k  = i + 2
    }
    out.print_line(ans);

    // this is sort of like % (2k and then in almost palin
    // 4: 0 1 2 3 2 1 0

    // x - n == 0 mod 2k
    // test 9 1 -- 8 div -- 2, 4, 8 which is (2k-2) so 2, 5
    // but also 8 -

    // 2 0 -- 2
    // 75 3 -- 72
    // 99 98 -- 1
    // 1000000000  500000000 -- 5000000000
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
