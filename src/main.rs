use rclrs::*;

mod do_nothing_simple;
use do_nothing_simple::*;

mod do_nothing_advanced;
use do_nothing_advanced::*;

fn main() {
    let mut executor: Executor = Context::default().create_executor(DoNothingRuntime);
    executor.spin(SpinOptions::default());
}
