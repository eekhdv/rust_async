mod dbus;
use dbus::service;

// mod qt;
// use qt::notif_widget::NotificationWidget;

use std::error::Error;

use tokio;
use tokio::sync::mpsc;
use uuid::Uuid;

// use qt_widgets::QApplication;
// use qt_core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    service::setup_server().await?;

    let (tx, mut rx) = mpsc::channel(uuid::Uuid::new_v4().to_u128_le() % usize::MAX);

    // loop {
    //     std::future::pending::<()>().await;
    // }
    
    // unsafe {qt_core::QCoreApplication::set_attribute_1a(qt_core::ApplicationAttribute::AAShareOpenGLContexts)};
    // QApplication::init(|_| {

    //     let test = NotificationWidget::new();

    //     test.show();

    //     unsafe {
    //         QApplication::exec()
    //     }
    // });
    Ok(())
}

