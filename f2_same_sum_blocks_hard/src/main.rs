//{"name":"F2. Same Sum Blocks (Hard)","group":"Codeforces - Codeforces Round 547 (Div. 3)","url":"https://codeforces.com/problemset/problem/1141/F2","interactive":false,"timeLimit":3000,"tests":[{"input":"7\n4 1 2 2 1 5 3\n","output":"3\n7 7\n2 3\n4 5\n"},{"input":"11\n-5 -4 -3 -2 -1 0 1 2 3 4 5\n","output":"2\n3 4\n1 1\n"},{"input":"4\n1 1 1 1\n","output":"4\n4 4\n1 1\n2 2\n3 3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"F2SameSumBlocksHard"}}}

use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_int_vec(n);

    let mut map = BTreeMap::new();

    for i in 0..n {
        let mut s = 0;
        for j in i..n {
            s += a[j];
            map.entry(s)
                .and_modify(|v: &mut Vec<(usize, usize)>| v.push((j, i)))
                .or_insert(vec![(j, i)]);
        }
    }

    let mut ans = vec![];
    for (k, v) in map.iter_mut() {
        v.sort();

        let mut cans = vec![];
        let mut last = -1;

        for e in v {
            if e.1 as i32 > last {
                last = e.0 as i32;
                cans.push((e.1, e.0));
            }
        }
        if cans.len() > ans.len() {
            ans = cans;
        }
    }
    out.print_line(ans.len());
    for (a, b) in ans {
        out.print_line((a + 1, b + 1));
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
