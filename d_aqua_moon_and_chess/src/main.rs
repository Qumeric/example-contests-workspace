//{"name":"D. AquaMoon and Chess","group":"Codeforces - Codeforces Round 732 (Div. 2)","url":"https://codeforces.com/problemset/problem/1546/D","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n4\n0110\n6\n011011\n5\n01010\n20\n10001111110110111000\n20\n00110110100110111101\n20\n11101111011000100010\n","output":"3\n6\n1\n1287\n1287\n715\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DAquaMoonAndChess"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModIntF;
use algo_lib::numbers::mod_utils::Combinations;
use algo_lib::string::str::StrReader;

type PreCalc = Combinations<ModIntF>;

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, c: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let mut vec: Vec<usize> = vec![];
    vec.push((s[0] - b'0') as usize);
    for i in 1..n {
        let elem = vec.pop().unwrap() as usize;
        if s[i] == b'1' {
            if elem == 0 {
                vec.push(elem);
                vec.push(1);
            } else {
                vec.push(elem + 1);
            }
        } else {
            if elem != 1 {
                vec.push(elem);
            }
            vec.push(0);
        }
    }
    if vec.len() > 0 && vec.last().unwrap() == &1 {
        vec.pop();
    }
    let mut num: usize = 0;
    let mut from: usize = 0;
    for i in 0..(vec.len()) {
        vec[i] = vec[i] / 2;
        num += vec[i] as usize;
        if vec[i] == 0 {
            from += 1;
        }
    }
    // out.print_line(&vec);
    from += num;

    // out.print_line((from, num));
    let ans = c.c(from, num);
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let pre_calc = Combinations::<ModIntF>::new(100_001);

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
