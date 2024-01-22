//{"name":"D. Number of Parallelograms","group":"Codeforces - Educational Codeforces Round 11","url":"https://codeforces.com/contest/660/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n0 1\n1 0\n1 1\n2 0\n","output":"1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DNumberOfParallelograms"}}}

use std::collections::BTreeMap;
use std::f32::consts::PI;

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::geometry::geometry_utils::canonize_angle;
use algo_lib::geometry::point::Point;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::real::Real;

type PreCalc = ();

// TODO not quite correct, it's slightly trick
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut pairs = (0..n)
        .map(|_| {
            Point::new(
                Real(input.read_long() as f64),
                Real(input.read_long() as f64),
            )
        })
        .collect_vec();

    pairs.sort();

    let mut map: BTreeMap<Real, usize> = Default::default();
    for i in 0..n {
        out.print_line(&pairs[i]);
        for j in (i + 1)..n {
            if pairs[j].y < pairs[i].y {
                let vec = pairs[i] - pairs[j];
                let a = vec.angle();
                map.entry(a).and_modify(|x| *x += 1).or_insert(1);
            }
        }
    }
    let mut ans = 0;
    for (k, v) in map {
        out.print_line((k, v));

        ans += v * (v - 1) / 2;
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
