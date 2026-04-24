use async_trait::async_trait;

#[derive(Debug, Clone)]
pub struct PublishedMessage {
    pub exchange: String,
    pub routing_key: String,
    pub body: Vec<u8>,
}

#[derive(Debug, Default)]
pub struct MockPublisher {
    published: tokio::sync::Mutex<Vec<PublishedMessage>>,
}

impl MockPublisher {
    pub fn new() -> Self {
        Self {
            published: tokio::sync::Mutex::new(Vec::new()),
        }
    }

    pub async fn get_published(&self) -> Vec<PublishedMessage> {
        self.published.lock().await.clone()
    }

    pub async fn clear(&self) {
        self.published.lock().await.clear();
    }
}

#[async_trait]
impl crate::MessagePublisher for MockPublisher {
    async fn publish(&self, exchange: &str, routing_key: &str, body: &[u8]) -> anyhow::Result<()> {
        let mut guard = self.published.lock().await;
        guard.push(PublishedMessage {
            exchange: exchange.to_string(),
            routing_key: routing_key.to_string(),
            body: body.to_vec(),
        });
        Ok(())
    }
}
