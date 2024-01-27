//{"name":"D. Good Trip","group":"Codeforces - Codeforces Round 921 (Div. 2)","url":"https://codeforces.com/contest/1925/problem/D","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n100 0 24\n2 1 10\n1 2 1\n3 1 2\n2 1 1\n5 2 4\n1 2 25\n3 2 24\n","output":"0\n55\n777777784\n40000020\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DGoodTrip"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;
use algo_lib::numbers::mod_int::ModInt7;
use algo_lib::numbers::num_traits::invertable::Invertable;
use algo_lib::numbers::num_traits::primitive::Primitive;
use algo_lib::numbers::number_ext::Power;

type PreCalc = ();

fn solve(input: &mut Input, out: &mut Output, _test_case: usize, _data: &PreCalc) {
    let n = input.read_size();
    let m = input.read_size();
    let k = input.read_size();

    let mut edges = vec![];
    for _ in 0..m {
        input.read1();
        input.read1();
        edges.push(ModInt7::new(input.read_int()));
    }

    if m == 0 {
        out.print_line(0);
        return;
    }

    let mut p = ModInt7::new(n.to());
    p *= ModInt7::new((n - 1).to());
    p *= ModInt7::new(2).inv().unwrap();
    p = p.inv().unwrap();

    let mut prob = vec![];
    let mut facs = vec![];
    let mut base = ModInt7::new(1);
    facs.push(base);
    for i in 2..=(k + 5) {
        facs.push(base);
        base *= ModInt7::new(i.to());
    }

    prob.push(ModInt7::new(0));
    for i in 1..=k {
        let possible_choices = facs[k];
        let div1 = facs[k - i].inv().unwrap();
        let div2 = facs[i].inv().unwrap();

        let binomal = possible_choices * div1 * div2;

        let take = p.power(i);
        let no_take = (ModInt7::new(1) - p).power(k - i);

        let p_i = binomal * take * no_take;
        prob.push(p_i);
    }

    let mut ans = ModInt7::new(0);

    // This is correct but I need faster
    // for edge in edges.into_iter() {
    //     let mut val = edge.clone();
    //     let mut sum = ModInt7::new(0);
    //     for p in 1..=k {
    //         sum += val;
    //         val += ModInt7::new(1);
    //         ans += sum * prob[p];
    //     }
    // }

    let mut edge_val = ModInt7::new(0);
    for edge in edges.iter() {
        edge_val += edge.clone();
    }
    let edge_count = ModInt7::new(m.to());

    let mut edge_sum = ModInt7::new(0);
    for p in 1..=k {
        edge_sum += edge_val;
        edge_val += edge_count;
        ans += edge_sum * prob[p];
    }

    out.print_line(ans);

    // given biderectional weighted graph.
    // choose two verticies randomly, if there is an edge:
    // SUM += w
    // w += 1

    // linearity of math expectation.
    // For each edge need to calc prob for each possible amount allocated
    // possibly chances of extreme allocation are super low so we don't have to calc them?
    // 1000 is more than enough even for head/tails. But we can't have 2 pairs, only 1 and 3.
    // So 200 should be safe too?
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
