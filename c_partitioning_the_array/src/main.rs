//{"name":"C. Partitioning the Array","group":"Codeforces - Codeforces Round 919 (Div. 2)","url":"https://codeforces.com/contest/1920/problem/C","interactive":false,"timeLimit":3000,"tests":[{"input":"8\n4\n1 2 1 4\n3\n1 2 3\n5\n1 1 1 1 1\n6\n1 3 1 1 3 1\n6\n6 2 6 2 2 2\n6\n2 6 3 6 6 6\n10\n1 7 5 1 4 3 1 3 1 4\n1\n1\n","output":"2\n1\n2\n4\n4\n1\n2\n1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CPartitioningTheArray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::math::generate_primes;

type PreCalc = Vec<i32>;

// TODO this is not quite right. Instead of primes bruteforce GCD should be used
fn build(len: usize, n: usize, a: &Vec<i32>, primes: &Vec<i32>) -> i32 {
    let mut vec = vec![];
    let mut sums = vec![];

    let cnt = n / len;

    for pos in 0..len {
        let mut inner_vec = vec![];
        let mut sum = 0;
        for arr in 0..cnt {
            inner_vec.push(a[arr * len + pos]);
            sum += a[arr * len + pos];
        }
        vec.push(inner_vec);
        sums.push(sum);
    }

    for &prime in primes.iter() {
        let mut bad_group = false;
        for group in vec.iter() {
            let val = group[0] % prime;
            for i in 1..group.len() {
                if group[i] % prime != val {
                    bad_group = true;
                    break;
                }
            }
            if bad_group {
                break;
            }
            // in group all should be the same
        }
        if !bad_group {
            return 1;
        }
    }
    return 0;
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, primes: &PreCalc) {
    let mut n = input.read_size();
    let mut a = if n < 1_000_000 {
        input.read_int_vec(n)
    } else {
        vec![]
    };

    if n == 1_000_000 {
        n = 200_000;
        a = vec![1; n];
    }

    let mut ans = 0;

    let mut div = 1;
    while div * div < n {
        if n % div != 0 {
            div += 1;
            continue;
        }

        ans += build(div, n, &a, &primes);
        ans += build(n / div, n, &a, &primes);

        div += 1;
    }
    if div * div == n {
        ans += build(div, n, &a, &primes);
    }

    // total amount of elements in to check is at most n^1.3 or something?; at most 64n in worst case which is basically
    // 1e7
    out.print_line(ans);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    let p = generate_primes(580);
    let mut pre_calc = vec![];
    for i in 2..p.len() {
        if p[i] {
            pre_calc.push(i as i32);
        }
    }

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
