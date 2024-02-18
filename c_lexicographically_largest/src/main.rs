//{"name":"C. Lexicographically Largest","group":"Codeforces - think-cell Round 1","url":"https://codeforces.com/contest/1930/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n2\n2 1\n5\n1 100 1000 1000000 1000000000\n3\n6 4 8\n","output":"3 2\n1000000005 1000004 1003 102 2\n11 7 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CLexicographicallyLargest"}}}
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use std::collections::BTreeMap;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut a = input.read_long_vec(n);

    for i in 0..n {
        a[i] += 1 + i as i64;
    }

    let mut cnt = BTreeMap::new();

    let mut result = vec![];
    for i in 0..n {
        let same_before = cnt.get(&a[i]).unwrap_or(&0);

        result.push(a[i] - same_before);

        cnt.entry(&a[i]).and_modify(|x| *x += 1).or_insert(1);
    }
    result.sort();
    result.dedup();
    result.reverse();

    out.print_line(result);
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
