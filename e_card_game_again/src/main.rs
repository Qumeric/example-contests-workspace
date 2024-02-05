//{"name":"E. Card Game Again","group":"Codeforces - Educational Codeforces Round 24","url":"https://codeforces.com/contest/818/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"3 4\n6 2 8\n","output":"4\n"},{"input":"3 6\n9 1 14\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECardGameAgain"}}}

use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::factorize_map;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let a = input.read_size_vec(n);

    let factor = factorize_map(k as i64);
    let mut trees = BTreeMap::<i64, Vec<usize>>::new();

    for (k, v) in &factor {
        trees.insert(*k, vec![0; n + 1]);
    }
    for i in 0..n {
        let mut cur = a[i];
        for (&k, _) in &factor {
            let mut v = 0;
            while cur % (k as usize) == 0 {
                cur /= k as usize;
                v += 1;
            }
            trees.get_mut(&k).unwrap()[i] = v;
        }
    }

    for k in factor.keys() {
        let cur = trees.get_mut(k).unwrap();
        for i in 1..n {
            cur[i] += cur[i - 1];
        }
    }

    let query = |k, l, r| {
        if l > r {
            return 0;
        }
        if l == 0 {
            return trees.get(k).unwrap()[r];
        }
        trees.get(k).unwrap()[r] - trees.get(k).unwrap()[l - 1]
    };

    let mut ans = 0;
    for i in 0..n {
        let mut l = i;
        let mut r = n;
        while r - l > 1 {
            let m = (l + r) / 2;
            let mut bad = false;
            for (k, v) in &factor {
                if query(k, i, m - 1) < *v {
                    bad = true;
                    break;
                }
            }
            if bad {
                l = m;
            } else {
                r = m;
            }
        }

        let mut bad = false;
        for (k, v) in &factor {
            if query(k, i, r - 1) < *v {
                bad = true;
                break;
            }
        }
        if !bad {
            ans += n - r + 1;
        }
    }
    out.print_line(ans);
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
