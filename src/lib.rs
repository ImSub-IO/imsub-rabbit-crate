pub mod error;
pub mod consumer;
pub mod sender;

use std::ops::DerefMut;
use std::sync::Arc;

use tokio::sync::RwLock;

use lapin::{Channel, Connection, ConnectionProperties, ConnectionState};
use error::Error;

#[derive(Clone)]
pub struct RabbitClient {
    ampq_url: String,
    conn: Arc<RwLock<Connection>>,
    reconnection_max: u8,
}

impl RabbitClient {
    pub async fn new(ampq_url: String) -> Result<Self, Error> {
        let conn = RabbitClient::connect(ampq_url.clone()).await?;

        Ok(Self {
            ampq_url,
            conn: Arc::new(RwLock::new(conn)),
            reconnection_max: 4,
        })
    }

    async fn connect(ampq_url: String) -> Result<Connection, Error> {
        Ok(Connection::connect(&ampq_url, ConnectionProperties::default()).await?)
    }

    pub async fn check_status(&self) -> bool {
        let status = self.status().await;
        use ConnectionState::*;

        match status {
            Initial | Connecting | Connected => true,
            Closing | Closed | Error => false,
        }
    }

    pub async fn status(&self) -> ConnectionState {
        self.conn.read().await.status().state()
    }

    pub async fn get_channel(&self) -> Result<Channel, Error> {
        Ok(self.conn.read().await.create_channel().await?)
    }

    pub async fn reconnect(&self) -> Result<(), Error> {
        let conn = Self::connect(self.ampq_url.clone()).await?;

        let mut write_guard = self.conn.write().await;
        let write_ref = write_guard.deref_mut();
        *write_ref = conn;

        Ok(())
    }
}
