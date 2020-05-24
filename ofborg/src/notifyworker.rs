use crate::worker::Action;

pub trait SimpleNotifyWorker {
    type J;

    fn consumer(&self, job: &Self::J, notifier: &mut dyn NotificationReceiver);

    fn msg_to_job(
        &self,
        routing_key: &str,
        content_type: &Option<String>,
        body: &[u8],
    ) -> Result<Self::J, String>;
}

pub trait NotificationReceiver {
    fn tell(&mut self, action: Action);
}

#[derive(Default)]
pub struct DummyNotificationReceiver {
    pub actions: Vec<Action>,
}

impl DummyNotificationReceiver {
    pub fn new() -> DummyNotificationReceiver {
        Default::default()
    }
}

impl NotificationReceiver for DummyNotificationReceiver {
    fn tell(&mut self, action: Action) {
        self.actions.push(action);
    }
}
