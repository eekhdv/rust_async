use std::collections::HashMap;

use zbus::{dbus_interface, zvariant::Value};

pub struct Notifications;

#[dbus_interface(name = "org.freedesktop.Notifications")]
impl Notifications {
    #[dbus_interface(name = "Notify")]
    pub async fn notify(
        &self,
        app_name: &str,
        _replaced_id: u32,
        _app_icon: &str,
        title: &str,
        body: &str,
        _actions: Vec<String>,
        hints: HashMap<&str, Value<'_>>,
        _expire_timeout: i32,
    ) -> String {
        format!("{app_name} -> {title}: {body} --- hints: {:?}", hints)
    }
}
