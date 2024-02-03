//{"name":"D1. RGB Substring (easy version)","group":"Codeforces - Codeforces Round 575 (Div. 3)","url":"https://codeforces.com/contest/1196/problem/D1","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5 2\nBGGGG\n5 3\nRBRGR\n5 5\nBBBRR\n","output":"1\n0\n3\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D1RGBSubstringEasyVersion"}}}

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::collections::specs::AssignSum;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();
    let s = input.read_str();

    let s = s
        .into_iter()
        .map(|x| match x {
            b'R' => 0,
            b'G' => 1,
            b'B' => 2,
            _ => unimplemented!(),
        })
        .collect_vec();

    let mut kinds = vec![vec![0; n]; 3];

    for i in 0..3 {
        let mut val = i;
        for j in 0..n {
            if s[j] != val {
                kinds[i][j] = 1;
            }
            val = (val + 1) % 3;
        }
    }
    let mut a = StaticArq::<AssignSum>::new(&kinds[0]);
    let mut b = StaticArq::<AssignSum>::new(&kinds[1]);
    let mut c = StaticArq::<AssignSum>::new(&kinds[2]);

    let mut ans = 1_000_000;

    for i in 0..n {
        let last = i + k - 1;
        if last >= n {
            continue;
        }
        ans.minim(a.query(i, last));
        ans.minim(b.query(i, last));
        ans.minim(c.query(i, last));
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
