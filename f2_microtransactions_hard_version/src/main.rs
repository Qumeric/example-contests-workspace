//{"name":"F2. Microtransactions (hard version)","group":"Codeforces - Codeforces Round 560 (Div. 3)","url":"https://codeforces.com/contest/1165/problem/F2","interactive":false,"timeLimit":3000,"tests":[{"input":"5 6\n1 2 0 2 0\n2 4\n3 3\n1 5\n1 2\n1 5\n2 3\n","output":"8\n"},{"input":"5 3\n4 2 1 3 2\n3 5\n4 2\n2 5\n","output":"20\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F2MicrotransactionsHardVersion"}}}

use std::cmp::min;

use algo_lib::collections::slice_ext::bounds::Bounds;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let mut k = input.read_size_vec(n);
    k.insert(0, 0);

    let mut sales_days = vec![vec![]; 500_000];
    let mut sales_ks = vec![vec![]; n + 1];

    for _ in 0..m {
        let d = input.read_size();
        let k = input.read_size();
        sales_days[d].push(k);
        sales_ks[k].push(d);
    }
    for i in 1..=200_000 {
        sales_days[i].sort();
    }
    for i in 1..=n {
        sales_ks[i].sort();
    }

    let mut l = 0;
    let mut r = 500_000;

    while r - l > 1 {
        let m = (l + r) / 2;
        let mut need = k.clone();
        let mut money = 0;
        let mut bought_sale = 0;

        for d in 1..=m {
            money += 1;
            for &k in &sales_days[d] {
                let next = sales_ks[k].upper_bound(&d);
                let to_buy = if next == sales_ks[k].len() || sales_ks[k][next] > m {
                    min(money, need[k])
                } else {
                    0
                };
                bought_sale += to_buy;
                need[k] -= to_buy;
                money -= to_buy;
            }
        }
        let money_left = m - bought_sale;
        let need_money = need.into_iter().sum::<usize>() * 2;
        if money_left >= need_money {
            r = m;
        } else {
            l = m;
        }
    }
    out.print_line(r);
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
