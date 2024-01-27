//{"name":"F. Random Query","group":"Codeforces - Educational Codeforces Round 28","url":"https://codeforces.com/contest/846/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"2\n1 2\n","output":"1.500000\n"},{"input":"2\n2 2\n","output":"1.000000\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FRandomQuery"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let a = input.read_size_vec(n);
    // prob of array of len 1 is lower? it's 1/n^2 while all others are 2/n^2

    // unique -- len - num_same
    // linearity of math expectation?
    // ans for all_diff (len + 1)/2? - expected_number_of_same
    // all lengths contribute the same

    // 1 1
    // 1 2 x 2
    // 1 3 x 2
    // 1 4 x 2
    // 2 2
    // 2 3 x 2
    // 2 4 x 2
    // 3 3
    // 3 4 x 2
    // 4 4

    // 16 arrs of len 4

    let mut ans = 0f64;
    for i in 1..=n {
        let mult = if i == 1 { 1f64 } else { 2f64 };
        ans += (i * (n + 1 - i)) as f64 * mult;
    }
    ans /= (n * n) as f64;
    let mut neg = 0f64;

    let mut arrays = vec![vec![]; 1_000_000];

    for i in 0..n {
        arrays[a[i] - 1].push(i);
    }

    for arr in arrays {
        if arr.len() < 2 {
            continue;
        }

        let mut arr = arr.clone();
        arr.push(n);

        let mut ax = vec![];
        ax.push(arr[0] + 1);
        for i in 1..arr.len() {
            ax.push(arr[i] - arr[i - 1]);
        }

        out.print_line(&ax);

        let mut tot = 0f64;
        let mut val = 0f64;
        for i in 0..ax.len() {
            let mut cur = 0f64;
            for j in i + 2..ax.len() {
                cur += ((j - i - 1) * ax[j]) as f64;
            }
            tot += cur * ax[i] as f64;

            // tot += (val * a[i] as f64) as f64;
            // if i > 0 {
            //     val += ax[i - 1] as f64;
            // }
        }

        out.print_line(tot.to_string());

        neg += tot * 2f64 / (n * n) as f64;
    }

    out.print_line(ans.to_string());
    out.print_line(neg.to_string());
    out.print_line((ans - neg).to_string());
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
