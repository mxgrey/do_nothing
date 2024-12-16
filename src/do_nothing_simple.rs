use rclrs::*;
use std::sync::Arc;
use futures::future::BoxFuture;

pub struct DoNothingRuntime;

impl ExecutorRuntime for DoNothingRuntime {
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

pub(crate) struct DoNothingChannel;

impl ExecutorChannel for DoNothingChannel {
    fn create_worker(
        &self,
        options: ExecutorWorkerOptions,
    ) -> Arc<dyn WorkerChannel> {
        Arc::new(DoNothingWorker)
    }

    fn wake_all_wait_sets(&self) {
        // Do nothing
    }
}

struct DoNothingWorker;

impl WorkerChannel for DoNothingWorker {
    fn add_activity_listener(&self, listener: WeakActivityListener) {
        // Do nothing
    }

    fn add_async_task(&self, f: BoxFuture<'static, ()>) {
        // Do nothing
    }

    fn add_to_waitset(&self, new_entity: Waitable) {
        // Do nothing
    }

    fn send_payload_task(&self, f: PayloadTask) {
        // Do nothing
    }
}
