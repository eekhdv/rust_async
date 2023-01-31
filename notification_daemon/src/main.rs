mod dbus;
use dbus::service;

use std::{error::Error, rc::Rc};

use cpp_core::{Ptr, Ref, StaticUpcast};
use qt_core::{
    qs, slot, ConnectionType, QBox, QObject, QPtr, QString, QTimer, SignalOfQString, SlotOfQString,
    WidgetAttribute, WindowType,
};
use qt_ui_tools::ui_form;
use qt_widgets::{QApplication, QDialog, QFrame, QLabel, QMainWindow, QWidget};

use tokio;
use uuid::Uuid;

#[ui_form("../ui/form.ui")]
#[derive(Debug)]
struct Form {
    widget: QBox<QWidget>,
    titleLabel: QPtr<QLabel>,
    bodyLabel: QPtr<QLabel>,
}

pub struct TestSpawner {
    form: Form,
    timer: QBox<QTimer>,
    close_sig: QBox<SignalOfQString>,
    qobj: QBox<QObject>,
    main_window: QBox<QFrame>,
}

impl StaticUpcast<QObject> for TestSpawner {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.form.widget.as_ptr().static_upcast()
    }
}

impl TestSpawner {
    fn new(main_window: QBox<QFrame>) -> Rc<Self> {
        unsafe {
            let widget = QWidget::new_0a();

            let timer = QTimer::new_0a();
            timer.set_interval(100);

            let close_signal = SignalOfQString::new();

            let qobject = QObject::new_0a();

            let this = Rc::new(Self {
                form: Form::load(),
                timer: (timer),
                close_sig: (close_signal),
                qobj: (qobject),
                main_window: (main_window),
            });

            this.init();

            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        self.timer.start_0a();

        self.form
            .titleLabel
            .set_text(&QString::from_std_str("teeestLABEL"));
        self.form
            .bodyLabel
            .set_text(&QString::from_std_str("teeest"));

        self.close_sig.connect_with_type(
            ConnectionType::QueuedConnection,
            &self.slot_on_widget_close(),
        );
    }

    fn show(self: &Rc<Self>) {
        unsafe {
            self.form.widget.show();
        }
    }

    #[slot(SlotOfQString)]
    unsafe fn on_widget_close(self: &Rc<Self>, _widget: Ref<QString>) {
        self.form.widget.close();
    }

    #[slot(SlotOfQString)]
    pub unsafe fn on_spawn_notification(self: &Rc<Self>, _guid: Ref<QString>) {
        let widget = QWidget::new_1a(&self.main_window);

        let guid = Uuid::new_v4().to_string();
        widget.set_object_name(&qs(&guid));

        widget.set_attribute_1a(WidgetAttribute::WADeleteOnClose);

        // Set the default action overlay
        let overlay = QDialog::new_1a(&widget);
        overlay.set_object_name(&qs("overlay"));

        overlay.set_window_flags(
            WindowType::WindowStaysOnTopHint
                | WindowType::Tool
                | WindowType::FramelessWindowHint
                | WindowType::BypassWindowManagerHint,
        );

        overlay.set_attribute_1a(WidgetAttribute::WADeleteOnClose);

        overlay.set_window_opacity(0.0);

        widget.show();
        overlay.show();
        overlay.hide();
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // service::setup_server().await?;

    // loop {
    //     std::future::pending::<()>().await;
    // }
    QApplication::init(|app| unsafe {
        let notification_windows = QMainWindow::new_0a();
        let desktop = QApplication::desktop();
        let top_right = desktop.screen_geometry().top_right();

        notification_windows.set_window_flags(
            WindowType::BypassWindowManagerHint
                | WindowType::FramelessWindowHint
                | WindowType::WindowStaysOnTopHint,
        );

        notification_windows.set_attribute_1a(WidgetAttribute::WADeleteOnClose);

        let frame = QFrame::new_1a(notification_windows.as_ptr());

        notification_windows.set_geometry_4a(top_right.x(), 0, 0, 0);

        notification_windows.show();

        let test = TestSpawner::new(frame);

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
        QApplication::exec()
    });
}

