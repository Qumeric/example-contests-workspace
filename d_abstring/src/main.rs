//{"name":"D. AB-string","group":"Codeforces - Educational Codeforces Round 74 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1238/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5\nAABBB\n","output":"6\n"},{"input":"3\nAAA\n","output":"3\n"},{"input":"7\nAAABABB\n","output":"15\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DABString"}}}

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;
use algo_lib::string::string_algorithms::palindromes::Palindromes;

type PreCalc = ();

// fuck i was solving kinda wrong thing, missed that letters are A and B only......
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();

    let odd = s.odd_palindromes();
    let even = s.even_palindromes();

    out.print_line(&odd);
    out.print_line(&even);

    let mut v = vec![];
    for i in 0..n {
        if odd[i] > 1 {
            v.push((i + 1 - odd[i], i + odd[i] - 1));
        }
        if even[i] > 0 {
            v.push((i - even[i], i + even[i] - 1));
        }
    }

    v.sort();

    let mut final_v = vec![];
    final_v.push(v[0]);

    for i in 1..v.len() {
        let mut cur = final_v.pop().unwrap();
        if v[i].0 <= cur.1 {
            cur.1.maxim(v[i].1);
            final_v.push(cur);
        } else {
            final_v.push(cur);
            final_v.push(v[i]);
        }
    }
    out.print_line(&final_v);

    let mut ans = 0;
    for (a, b) in final_v {
        let len = b + 1 - a;
        ans += len * (len + 1) / 2 - len;
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
