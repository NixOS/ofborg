use std::sync::Arc;

use crate::worker::Action;

#[async_trait::async_trait]
pub trait SimpleNotifyWorker {
    type J;

    async fn consumer(
        &self,
        job: Self::J,
        notifier: Arc<dyn NotificationReceiver + std::marker::Send + std::marker::Sync>,
    );

    fn msg_to_job(
        &self,
        routing_key: &str,
        content_type: &Option<String>,
        body: &[u8],
    ) -> Result<Self::J, String>;
}

#[async_trait::async_trait]
pub trait NotificationReceiver {
    async fn tell(&self, action: Action);
}

#[derive(Default)]
pub struct DummyNotificationReceiver {
    pub actions: parking_lot::Mutex<Vec<Action>>,
}

impl DummyNotificationReceiver {
    pub fn new() -> DummyNotificationReceiver {
        Default::default()
    }
}

#[async_trait::async_trait]
impl NotificationReceiver for DummyNotificationReceiver {
    async fn tell(&self, action: Action) {
        let mut actions = self.actions.lock();
        actions.push(action);
    }
}
