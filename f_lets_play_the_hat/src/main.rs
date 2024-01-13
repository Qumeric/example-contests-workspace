//{"name":"F. Let's Play the Hat?","group":"Codeforces - Codeforces Round 762 (Div. 3)","url":"https://codeforces.com/problemset/problem/1619/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 2 2\n8 3 1\n2 1 3\n","output":"3 1 2 3\n2 4 5\n3 4 5 2\n2 1 3\n\n2 6 2\n3 3 5 1\n3 4 7 8\n\n2 2 1\n2 2 1\n2 2 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FLetsPlayTheHat"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();

    let big_tables = n % m;
    let small_tables = m - big_tables;

    let mut player = 0;
    let mut next_player = 0;

    for i in 0..k {
        player = next_player;
        let big_sz = (n + m - 1) / m;
        for table in 0..big_tables {
            out.print(big_sz);
            out.print(" ");
            for _ in 0..big_sz {
                out.print(player + 1);
                out.print(" ");
                player = (player + 1) % n;
            }
            out.print_line("");
        }
        next_player = player;
        let small_sz = n / m;
        for table in 0..small_tables {
            out.print(small_sz);
            out.print(" ");
            for _ in 0..small_sz {
                out.print(player + 1);
                out.print(" ");
                player = (player + 1) % n;
            }
            out.print_line("");
        }
    }
    out.print_line("");
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
