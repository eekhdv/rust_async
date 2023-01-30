use std::collections::HashMap;
use std::error::Error;

use zbus::{zvariant::Value, Connection};

pub async fn send_notif() -> Result<(), Box<dyn Error>> {
    let conn = Connection::session().await?;

    let m = conn.call_method(
        Some("org.freedesktop.Notifications"),
        "/org/freedesktop/Notifications",
        Some("org.freedesktop.Notifications"),
        "Notify",
        &(
            "test-app",
            0u32,
            "dialog-information",
            "Test title",
            "Test body, teeeest body",
            vec![""; 0],
            HashMap::<&str, &Value>::new(),
            5000,
        ),
    ).await?;
    let reply: u32 = m.body().unwrap();
    dbg!(reply);
    Ok(())
}
