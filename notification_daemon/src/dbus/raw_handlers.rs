use super::prep_notifications::{DbusChannel, Notification, Rect};

use std::collections::HashMap;

use tokio::sync::mpsc::Sender;
use zbus::{dbus_interface, zvariant::Value};

pub struct NotificationsHandler {
    pub dbus_tx: Sender<DbusChannel>,
}

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl NotificationsHandler {
    #[dbus_interface(name = "Notify")]
    pub async fn notify(
        &self,
        app_name: String,
        _replaced_id: u32,
        app_icon: String,
        title: String,
        body: String,
        _actions: Vec<String>,
        _hints: HashMap<String, Value<'_>>,
        expire_timeout: i32,
    ) -> zbus::fdo::Result<u32> {
        let notif = Notification {
            app_name: (app_name),
            app_icon: (app_icon),
            title: (title),
            body: (body),
            expire_timeout: (expire_timeout),
            window: Rect::default(),
        };
        if let Err(_) = self.dbus_tx
            .send(DbusChannel::Notify {
                notification: notif,
            })
            .await {
            return Ok(1);
        }
        Ok(0)
    }
    
    #[dbus_interface(name = "GetCapabilities")]
    pub async fn get_capabilities(&self) -> Vec<String> {
        let capabilities = vec!["actions".to_string(), "body".to_string(), "persistence".to_string()];
        capabilities
    }

    #[dbus_interface(name = "CloseNotification")]
    pub async fn close_notification(&self) {
        unimplemented!()
    }

    #[dbus_interface(name = "GetServerInformation")]
    pub async fn get_serv_info(&self, _name: String, _vendor: String, _version: String, _spec_version: String) {
        unimplemented!()
    }

    #[dbus_interface(name = "ActionInvoked")]
    pub async fn action_invoked(&self, _id: u32, _action_key: String) {
        unimplemented!()
    }

    #[dbus_interface(name = "ActivationToken")]
    pub async fn activation_token(&self, _id: u32, _token: String) {
        unimplemented!()
    }

    #[dbus_interface(name = "NotificationClosed")]
    pub async fn notification_closed(&self, _id: u32, _reason: u32) {
        unimplemented!()
    }
}
