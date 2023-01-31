mod dbus;
use dbus::service;

mod qt;
use qt::notif_widget::NotificationWidget;

use std::error::Error;

use tokio;
use uuid::Uuid;

use qt_widgets::QApplication;
use qt_core;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // service::setup_server().await?;

    // loop {
    //     std::future::pending::<()>().await;
    // }
    
    unsafe {qt_core::QCoreApplication::set_attribute_1a(qt_core::ApplicationAttribute::AAShareOpenGLContexts)};
    QApplication::init(|_| {

        let test = NotificationWidget::new();

        test.show();

        // let notitification_signal = SignalOfQString::new();
        // notitification_signal.connect(
        //     &test.slot_on_spawn_notification(),
        // );

        // let guid = Uuid::new_v4().to_string();
        // notitification_signal.as_raw_ref().expect("oh").emit(&QString::from_std_str(&guid));

        // {
        //     let widget = QWidget::new_1a(&frame);

        //     let guid = Uuid::new_v4().to_string();
        //     widget.set_object_name(&qs(&guid));

        //     widget.set_attribute_1a(WidgetAttribute::WADeleteOnClose);

        //
        //     // Set the default action overlay
        //     let overlay = QDialog::new_1a(&widget);
        //     overlay.set_object_name(&qs("overlay"));

        //     overlay.set_window_flags(
        //         WindowType::WindowStaysOnTopHint
        //             | WindowType::Tool
        //             | WindowType::FramelessWindowHint
        //             | WindowType::BypassWindowManagerHint,
        //     );

        //     overlay.set_attribute_1a(WidgetAttribute::WADeleteOnClose);

        //     overlay.set_window_opacity(0.0);

        //     widget.show();
        //     overlay.show();

        //     widget.find_child(name)
        //     // overlay.hide();
        // }
        unsafe {
            QApplication::exec()
        }
    });
}

