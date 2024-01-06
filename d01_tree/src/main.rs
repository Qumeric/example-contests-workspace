//{"name":"D. 01 Tree","group":"Codeforces - Hello 2024","url":"https://codeforces.com/contest/1919/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n5\n2 1 0 1 1\n5\n1 0 2 1 3\n","output":"YES\nNO\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D01Tree"}}}

use std::collections::BTreeSet;

use algo_lib::collections::iter_ext::collect::IterCollect;
use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::num_traits::primitive::Primitive;

type PreCalc = ();

// TODO this is not correct
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_u64_vec(n);

    let mut i = 0;
    while i < n {
        // inclusive
        let r = i + a[i] as usize;
        if r >= n.to() {
            out.print_line("NO");
            return;
        }

        let mut elems: Vec<u64> = Default::default();
        for j in (i + 1)..=r {
            elems.push(a[j]);
        }

        elems.reverse();

        // out.print_line(("for", i, "-", a[i], "elems", &elems));

        for (j, e) in elems.enumerate() {
            let e = *e as usize;
            if e != j && e != j + 1 {
                out.print_line("NO");
                return;
            }
        }

        i = (r + 1) as usize;
    }
    out.print_line("YES");
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
}
//END MAIN
