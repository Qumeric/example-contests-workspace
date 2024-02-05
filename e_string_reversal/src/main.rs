//{"name":"E. String Reversal","group":"Codeforces - Educational Codeforces Round 96 (Rated for Div. 2)","url":"https://codeforces.com/problemset/problem/1430/E","interactive":false,"timeLimit":2000,"tests":[{"input":"5\naaaza\n","output":"2\n"},{"input":"6\ncbaabc\n","output":"0\n"},{"input":"9\nicpcsguru\n","output":"30\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EStringReversal"}}}

use algo_lib::collections::specs::PlusSum;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let s = input.read_str();
    let mut r = s.clone();
    r.reverse();

    let mut pos = vec![vec![]; 26];

    for i in 0..n {
        let char = s[i] - b'a';
        pos[char as usize].push(n - i - 1);
    }

    for i in 0..26 {
        pos[i].sort();
    }

    //let mut v = vec![9, 6, 7, 8, 5, 4, 1, 2, 3];
    let mut v = vec![0; n];
    let mut ptrs = vec![0; 26];

    for i in 0..n {
        let cur = (s[i] - b'a') as usize;
        v[i] = pos[cur][ptrs[cur]] + 1;
        ptrs[cur] += 1;
    }

    // out.print_line(&v);

    let mut tree = StaticArq::<PlusSum>::new(&vec![0; n]);

    let mut inversions = 0;
    for i in 0..v.len() {
        inversions += tree.query((v[i] - 1) as usize, v.len() - 1);
        tree.update_point((v[i] - 1) as usize, &1);
    }
    out.print_line(inversions);
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
