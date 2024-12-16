use rclrs::*;

mod do_nothing_simple;
use do_nothing_simple::*;

mod do_nothing_advanced;
use do_nothing_advanced::*;

fn main() {
    let mut executor: Executor = Context::default().create_executor(DoNothingRuntime);
    executor.spin(SpinOptions::default());

    let mut executor: Executor = Context::default().create_do_nothing_executor(
        DoNothingOptions { how_many_nothings: 10 }
    );
    executor.spin(SpinOptions::spin_once());
}
