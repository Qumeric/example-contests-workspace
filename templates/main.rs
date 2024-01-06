//$JSON

use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

$SOLVE

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
    // don't forget to set test_type = Single if you do it
    // tester::stress_test(run, tester::check);
}
//END MAIN
