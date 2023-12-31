//{"name":"F. Moving Points","group":"Codeforces - Codeforces Round 624 (Div. 3)","url":"https://codeforces.com/contest/1311/problem/F?locale=en","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 3 2\n-100 2 3\n","output":"3\n"},{"input":"5\n2 1 4 3 5\n2 2 2 3 4\n","output":"19\n"},{"input":"2\n2 1\n-3 0\n","output":"0\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FMovingPoints"}}}

use algo_lib::collections::dynamic_arq::DynamicArq;
use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::specs::{ArqSpec, AssignSum, PlusSum};
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::collections::treap::{ImpliedKeyPayload, PureDataPayload, TreapNode};
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::zip;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n: usize = input.read();
    let x: Vec<i64> = input.read_vec(n);
    let mut v: Vec<i64> = input.read_vec(n);

    let mut points = zip!(x.clone(), v.clone()).collect_vec();
    points.sort();

    v.sort();
    v.dedup();
    let mut coords_v = std::collections::HashMap::new();
    for (i, &v) in v.iter().enumerate() {
        coords_v.insert(v, i);
    }
    for point in &mut points {
        point.1 = *coords_v.get(&point.1).unwrap() as i64;
    }

    let mut tree = StaticArq::<PlusSum>::new(&vec![0; n]);
    let mut tree_amount = StaticArq::<PlusSum>::new(&vec![0; n]);

    let mut ans = 0;
    for (x, v) in points.iter() {
        tree.update(v.to(), v.to(), x);
        tree_amount.update(v.to(), v.to(), &1);

        let sum = tree.query(0, *v as usize);
        let cnt = tree_amount.query(0, *v as usize);

        ans += -sum + x * cnt;
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
}
//END MAIN

// 1 2 3 4 5
// 2 2 3 2 4

// left
// 1. 0
// 2. 1
// 3. (3 - 1) + (3 - 2) = 3
// 4. (4 - 1) + (4 - 2) = 5
// 5. (5 - 1) + (5 - 4) + (5 - 3) + (5 - 2) = 1 + 2 + 3 + 4 = 10 sum = 19

// 1     2    3
// -100  3    2
