//{"name":"D. Recover it!","group":"Codeforces - Codeforces Round 565 (Div. 3)","url":"https://codeforces.com/contest/1176/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n3 5 2 3 2 4\n","output":"3 4 2\n"},{"input":"1\n2750131 199999\n","output":"199999\n"},{"input":"1\n3 6\n","output":"6\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DRecoverIt"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::generate_primes;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let mut b = input.read_size_vec(n * 2);

    b.sort();

    // if prime then p_ai added to b
    // otherwise greatest divisor != ai is added
    // array shuffled

    // seems like p_ai is always <= p_i (for 2 it's 3, for 3 it's 5)
    // so we need to take half small primes

    // take non-primes. For each take it and pair

    let p = generate_primes(3_000_000);

    let mut vals = vec![vec![]; 3_000_000];
    for i in 0..n * 2 {
        vals[b[i]].push(i);
    }

    let mut primes = vec![0];
    for i in 2..3_000_000 {
        if p[i] {
            primes.push(i);
        }
    }

    let mut ans = vec![];

    for i in (0..n * 2).rev() {
        if b[i] == 0 {
            continue;
        }

        if !p[b[i]] {
            let mut d = 2;
            while d * d <= b[i] {
                if b[i] % d == 0 {
                    break;
                }
                d += 1;
            }
            d = b[i] / d;
            let e = vals[d].pop().unwrap();
            b[e] = 0;
            ans.push(b[i]);
            // out.print_line(("FOUND COMPOSITE", b[i], "AND DIV", d));
        }
    }
    for i in 0..n * 2 {
        if b[i] == 0 {
            continue;
        }

        if p[b[i]] {
            let prime = primes[b[i]];
            let e = vals[prime].pop().unwrap();
            b[e] = 0;
            ans.push(b[i]);
            // out.print_line(("FOUND PRIME", b[i], "AND P_i", prime));
        }
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
