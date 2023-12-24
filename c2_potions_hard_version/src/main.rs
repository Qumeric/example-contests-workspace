//{"name":"C2. Potions (Hard Version)","group":"Codeforces - Codeforces Round 723 (Div. 2)","url":"https://codeforces.com/problemset/problem/1526/C2","interactive":false,"timeLimit":1000,"tests":[{"input":"6\n4 -4 1 -3 1 -3\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C2PotionsHardVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

// TODO: not finished
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let a: Vec<i64> = input.read_vec(n);

    let mut split_vecs: Vec<Vec<i64>> = Vec::new();
    let mut current_vec: Vec<i64> = Vec::new();
    for &item in &a {
        if current_vec.is_empty() || (item >= 0) == (current_vec[0] >= 0) {
            current_vec.push(item);
        } else {
            split_vecs.push(current_vec);
            current_vec = vec![item];
        }
    }
    if !current_vec.is_empty() {
        split_vecs.push(current_vec);
    }

    split_vecs = split_vecs
        .into_iter()
        .map(|mut v| {
            if v[0] >= 0 {
                vec![v.iter().sum()]
            } else {
                v.sort();
                v.reverse();
                for i in 1..v.len() {
                    v[i] += v[i - 1];
                }
                v
            }
        })
        .collect();

    for vec in split_vecs {
        out.print_line(vec);
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
}
//END MAIN
