//{"name":"D2. Domino (hard version)","group":"Codeforces - Codeforces Round 734 (Div. 3)","url":"https://codeforces.com/problemset/problem/1551/D2","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n4 4 2\n2 3 0\n3 2 3\n1 2 0\n2 4 2\n5 2 2\n2 17 16\n2 1 1\n","output":"YES\naccx\naegx\nbega\nbdda\nYES\naha\naha\nYES\nzz\naa\nzz\nNO\nYES\naaza\nbbza\nNO\nYES\nbbaabbaabbaabbaay\nddccddccddccddccy\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D2DominoHardVersion"}}}

use algo_lib::collections::md_arr::arr2d::Arr2d;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn get(i: usize, j: usize) -> usize {
    1 + (i % 5) * 5 + (j % 5)
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut hor = input.read_size();
    let mut vert = (n * m) / 2 - hor;

    let mut ans = Arr2d::new(n, m, 0);

    if n % 2 == 1 {
        let mut j = 0;
        while j < m && hor > 0 {
            let c = get(0, j);
            ans[0][j] = c;
            ans[0][j + 1] = c;
            j += 2;
            hor -= 1;
        }
    }
    if m % 2 == 1 {
        let mut i = 0;
        while i < n && vert > 0 {
            let c = get(i, 0);
            ans[i][0] = c;
            ans[i + 1][0] = c;
            i += 2;
            vert -= 1;
        }
    }

    let mut i = n % 2;

    while i < n {
        let mut j = m % 2;
        while j < m {
            let c = get(i, j);
            if hor >= 2 {
                ans[i][j] = c;
                ans[i][j + 1] = c;
                let c2 = get(i + 1, j);
                ans[i + 1][j] = c2;
                ans[i + 1][j + 1] = c2;
                hor -= 2;
            } else if vert >= 2 {
                ans[i][j] = c;
                ans[i + 1][j] = c;
                let c2 = get(i, j + 1);
                ans[i][j + 1] = c2;
                ans[i + 1][j + 1] = c2;
                vert -= 2;
            }
            j += 2;
        }

        i += 2;
    }

    for i in 0..n {
        for j in 0..m {
            if ans[i][j] == 0 {
                out.print_line("NO");
                return;
            }
        }
    }
    out.print_line("YES");
    for i in 0..n {
        for j in 0..m {
            let l = ans[i][j];
            let char_l = (b'a' + l as u8) as char;
            out.print(char_l.to_string());
        }
        out.print_line("");
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
