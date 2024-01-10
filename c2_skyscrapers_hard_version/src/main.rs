//{"name":"C2. Skyscrapers (hard version)","group":"Codeforces - Codeforces Round 622 (Div. 2)","url":"https://codeforces.com/problemset/problem/1313/C2","interactive":false,"timeLimit":3000,"tests":[{"input":"5\n1 2 3 2 1\n","output":"1 2 3 2 1\n"},{"input":"3\n10 6 8\n","output":"10 6 6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"C2SkyscrapersHardVersion"}}}

use std::cmp::{max, min};

use algo_lib::collections::iter_ext::iters::Iters;
use algo_lib::collections::specs::ArqSpec;
use algo_lib::collections::static_arq::StaticArq;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::misc::recursive_function::{Callable2, RecursiveFunction2};

type PreCalc = ();

pub enum AssignMinPos {}
impl ArqSpec for AssignMinPos {
    // value, pos
    type S = (usize, usize);
    type F = usize;
    fn op(&a: &Self::S, &b: &Self::S) -> Self::S {
        a.min(b)
    }
    fn identity() -> Self::S {
        (usize::MAX, 0)
    }
    fn compose(&f: &Self::F, &s: &Self::F) -> Self::F {
        min(f, s)
    }
    fn apply(&f: &Self::F, s: &Self::S, _: i64) -> Self::S {
        (min(f, s.0), s.1)
    }
}

// TODO: fuck I honestly have no idea what's wrong. But probably there is simpler solution without segtree
fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a: Vec<(usize, usize)> = input
        .read_size_vec(n)
        .enumerate()
        .map(|(a, &b)| (b, a))
        .collect();

    let mut tree = StaticArq::<AssignMinPos>::new(&a);

    let mut actions = vec![];

    let mut f = RecursiveFunction2::new(|f, l, r| {
        let (min_val, min_pos) = tree.query(l, r);
        // println!("for {l} {r}: val: {min_val} pos: {min_pos}");
        if l > r {
            return 0;
        }
        if l == r {
            return min_val;
        }
        if min_pos == l {
            actions.push((min_pos, -1));
            return min_val + f.call(min_pos + 1, r);
        }
        if min_pos == r {
            actions.push((min_pos, 1));
            return min_val + f.call(l, min_pos - 1);
        }

        let left_len = min_pos - l + 1;
        let right_len = r - min_pos + 1;
        let left_ans = min_val * left_len + f.call(min_pos + 1, r);
        let right_ans = min_val * right_len + f.call(l, min_pos - 1);
        if right_ans > left_ans {
            actions.push((min_pos, 1));
            right_ans
        } else {
            actions.push((min_pos, -1));
            left_ans
        }
    });
    let ans = f.call(0, n - 1);
    // out.print_line(ans);
    for (pos, dir) in actions {
        let v = tree.query_point(pos).0;
        if dir == 1 {
            tree.update(pos, n - 1, &v);
        } else {
            tree.update(0, pos, &v)
        }
    }
    for i in 0..n {
        out.print(tree.query_point(i).0);
        out.print(" ");
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
