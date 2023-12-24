//{"name":"C. Product of Three Numbers","group":"Codeforces - Codeforces Round 615 (Div. 3)","url":"https://codeforces.com/problemset/problem/1294/C","interactive":false,"timeLimit":2000,"tests":[{"input":"5\n64\n32\n97\n2\n12345\n","output":"YES\n2 4 8\nNO\nNO\nNO\nYES\n3 5 823\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CProductOfThreeNumbers"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::{self, Output};
use algo_lib::math::{factorize, factorize_map};

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: i64 = input.read();

    let f = factorize_map(n);

    let keys: Vec<_> = f.keys().collect();

    let mut ans = vec![];

    if keys.len() >= 3 {
        let k1 = *keys[0];
        let k2 = *keys[1];
        ans.push(k1);
        ans.push(k2);
        let mut mul = 1;
        for (k, v) in f {
            if k == k1 || k == k2 {
                for i in 1..v {
                    mul *= k;
                }
            } else {
                for i in 0..v {
                    mul *= k;
                }
            }
        }
        ans.push(mul);
        out.print_line(true);
        out.print_line(ans);
    } else if keys.len() == 2 {
        let k1 = keys[0];
        let v1 = f[k1];
        let k2 = keys[1];
        let v2 = f[k2];

        if v2 == 2 && v1 == 2 {
            out.print_line(true);
            out.print_line((k1, k2, k1 * k2));
            return;
        }
        if v1 >= 3 {
            out.print_line(true);
            out.print_line((k1, k1.pow((v1 - 1) as u32), k2.pow(v2 as u32)))
        } else if v2 >= 3 {
            out.print_line(true);
            out.print_line((k2, k2.pow((v2 - 1) as u32), k1.pow(v1 as u32)))
        } else {
            out.print_line(false);
        }
    } else {
        // one key
        let k = keys[0];
        let v = f[k];

        if v < 6 {
            out.print_line(false);
            return;
        }
        out.print_line(true);
        out.print_line((k, k.pow(2), k.pow((v - 3) as u32)))
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
}
//END MAIN
