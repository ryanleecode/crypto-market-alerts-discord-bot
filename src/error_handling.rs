use tokio::sync::broadcast;
use tracing::error;

use crate::bot::{self, handler::HandlerError};

pub struct BotErrorHandler {
    shutdown_send: broadcast::Sender<()>,
}

impl BotErrorHandler {
    pub fn new(shutdown_send: broadcast::Sender<()>) -> Self {
        BotErrorHandler { shutdown_send }
    }
}

#[async_trait::async_trait]
impl bot::handler::ErrorHandler for BotErrorHandler {
    async fn on_error(&self, e: HandlerError) {
        match e {
            HandlerError::Setup(_) => {
                self.shutdown_send.send(()).expect("shutdown");
            }
            HandlerError::Interaction(e) => {
                error!("interaction error: {}", e);
            }
            HandlerError::Internal(e) => {
                error!("internal error: {}", e);
            }
        }
    }
}
