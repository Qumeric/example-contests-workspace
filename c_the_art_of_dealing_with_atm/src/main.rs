//{"name":"C. The Art of Dealing with ATM","group":"Codeforces - VK Cup 2015 - Round 1","url":"https://codeforces.com/contest/524/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6 20\n10 50 100 500 1000 5000\n8\n4200\n100000\n95000\n96000\n99000\n10100\n2015\n9950\n","output":"6\n20\n19\n20\n-1\n3\n-1\n-1\n"},{"input":"5 2\n1 2 3 5 8\n8\n1\n3\n5\n7\n9\n11\n13\n15\n","output":"1\n1\n1\n2\n2\n2\n2\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTheArtOfDealingWithATM"}}}

use std::cmp::min;
use std::collections::BTreeMap;

use algo_lib::collections::min_max::MinimMaxim;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let k = input.read_size();

    let mut a = input.read_size_vec(n);
    a.sort();

    let mut prec = BTreeMap::<usize, usize>::new();

    for &e in &a {
        for i in 1..=k {
            let val = e * i;

            prec.entry(val).and_modify(|x| *x = min(*x, i)).or_insert(i);
        }
    }

    let q = input.read_size();
    for _ in 0..q {
        let x = input.read_size();

        let mut ans = k + 1;
        for i in 0..n {
            let big = a[i];
            let max_big = x / big;
            if big * max_big == x {
                ans.minim(max_big);
            }

            for take_big in 1..=min(max_big, k) {
                let left = x - big * take_big;

                ans.minim(take_big + *prec.get(&left).unwrap_or(&1_000));
            }
        }

        if ans > k {
            out.print_line(-1);
        } else {
            out.print_line(ans);
        }
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
