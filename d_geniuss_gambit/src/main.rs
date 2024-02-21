//{"name":"D. Genius's Gambit","group":"Codeforces - Codeforces Round 704 (Div. 2)","url":"https://codeforces.com/problemset/problem/1492/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4 2 3\n","output":"Yes\n101000\n100001\n"},{"input":"3 2 1\n","output":"Yes\n10100\n10010\n"},{"input":"3 2 5\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DGeniussGambit"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let mut a = input.read_size();
    let mut b = input.read_size();
    let mut k = input.read_size();

    if a == 0 {
        if k == 0 {
            out.print_line("Yes");
            for i in 0..b {
                out.print(1);
            }
            out.print_line("");
            for i in 0..b {
                out.print(1);
            }
            out.print_line("");
        } else {
            out.print_line("No");
        }
        return;
    }

    if k == 0 {
        let mut ans = vec![];
        while b > 0 {
            ans.push(1);
            b -= 1;
        }
        while a > 0 {
            ans.push(0);
            a -= 1;
        }
        out.print_line("Yes");
        for x in &ans {
            out.print(x);
        }
        out.print_line("");
        for x in &ans {
            out.print(x);
        }
        out.print_line("");
        return;
    }

    let mut x = vec![];
    let mut y = vec![];

    x.push(0);
    y.push(1);
    k -= 1;
    while k > 0 {
        if a > 1 {
            x.push(0);
            y.push(0);
            a -= 1;
        } else if b > 1 {
            x.push(1);
            y.push(1);
            b -= 1;
        } else {
            out.print_line("No");
            return;
        }
        k -= 1;
    }
    a -= 1;
    b -= 1;
    x.push(1);
    y.push(0);

    if b == 0 {
        out.print_line("No");
        return;
    }

    while a > 0 {
        y.push(0);
        x.push(0);
        a -= 1;
    }

    while b > 0 {
        y.push(1);
        x.push(1);
        b -= 1;
    }

    out.print_line("Yes");
    for e in x.iter().rev() {
        out.print(e);
    }
    out.print_line("");
    for e in y.iter().rev() {
        out.print(e);
    }
    out.print_line("");
    return;
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
