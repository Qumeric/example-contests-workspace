//{"name":"E. Masha-forgetful","group":"Codeforces - Codeforces Round 764 (Div. 3)","url":"https://codeforces.com/problemset/problem/1624/E","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n\n4 8\n12340219\n20215601\n56782022\n12300678\n12345678\n\n2 3\n134\n126\n123\n\n1 4\n1210\n1221\n\n4 3\n251\n064\n859\n957\n054\n\n4 7\n7968636\n9486033\n4614224\n5454197\n9482268\n","output":"3\n1 4 1\n5 6 2\n3 4 3\n-1\n2\n1 2 1\n2 3 1\n-1\n3\n1 3 2\n5 6 3\n3 4 1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMashaForgetful"}}}

use std::collections::BTreeMap;

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::string::str::StrReader;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();

    let mut segs = BTreeMap::<String, (usize, usize, usize)>::new();

    let mut numbers = vec![];
    for i in 0..n {
        numbers.push(input.read_str());
        let mut j = 0;
        while j + 1 < m {
            let a = (numbers[i][j] as char).to_string();
            let b = (numbers[i][j + 1] as char).to_string();

            let d = a.clone() + &b;
            segs.entry(d.clone()).or_insert((j+1, j+3, i+1));

            if j + 2 < m {
                let c = (numbers[i][j + 2] as char).to_string();
                let e = d + &c;
                segs.entry(e).or_insert((j+1, j+4, i+1));
            }
            j += 1;
        }
    }

    let last = input.read_str();

    let mut can_cover = vec![None; m+1];
    can_cover[0] = Some((0, 0, 0));

    for i in 2..m {
        if can_cover[i-2] 
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
