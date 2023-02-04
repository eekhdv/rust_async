use std::sync::Arc;
use tokio::sync::Mutex;

use crate::dbus::prep_notifications::Notification;

#[derive(Debug, Clone)]
pub struct NotificationBox {
    pub notifications: Arc<Mutex<Vec<Notification>>>,
}

impl NotificationBox {
    pub fn new() -> Self {
        Self {
            notifications: Arc::new(Mutex::new(vec![])),
        }
    }
}
