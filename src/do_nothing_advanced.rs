use rclrs::*;
use std::sync::Arc;
use futures::future::BoxFuture;

use crate::do_nothing_simple::*;

pub struct DoNothingAdvanced {
    context: Context,
    options: DoNothingOptions,
}

pub struct DoNothingOptions {
    pub how_many_nothings: usize,
}

impl ExecutorRuntime for DoNothingAdvanced {
    fn channel(&self) -> Arc<dyn ExecutorChannel> {
        Arc::new(DoNothingChannel)
    }

    fn spin(&mut self, conditions: SpinConditions) -> Vec<RclrsError> {
        // Do nothing
        Vec::new()
    }

    fn spin_async(
        self: Box<Self>,
        conditions: SpinConditions,
    ) -> BoxFuture<'static, (Box<dyn ExecutorRuntime>, Vec<RclrsError>)> {
        // Do nothing
        Box::pin(async move {
            (self as Box<dyn ExecutorRuntime>, Vec::new())
        })
    }
}

pub trait DoNothingAdvancedFromContext {
    fn create_do_nothing_executor(&self, options: DoNothingOptions) -> Executor;
}

impl DoNothingAdvancedFromContext for Context {
    fn create_do_nothing_executor(&self, options: DoNothingOptions) -> Executor {
        let runtime = DoNothingAdvanced {
            context: self.clone(),
            options,
        };

        self.create_executor(runtime)
    }
}
