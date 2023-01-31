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

#[ui_form("./ui/new_simple_ui.ui")]
#[derive(Debug)]
struct NotificationForm {
    widget: QBox<QWidget>,
    frame: QPtr<QFrame>,
    app_title_label: QPtr<QLabel>,
    body_label: QPtr<QLabel>,
    title_label: QPtr<QLabel>,
}

pub struct NotificationWidget {
    form: NotificationForm,
    timer: QBox<QTimer>,
}

impl StaticUpcast<QObject> for NotificationWidget {
    unsafe fn static_upcast(ptr: Ptr<Self>) -> Ptr<QObject> {
        ptr.form.widget.as_ptr().static_upcast()
    }
}

impl NotificationWidget {
    pub fn new() -> Rc<Self> {
        unsafe {
            let timer = QTimer::new_0a();
            timer.set_interval(100);

            let this = Rc::new(Self {
                form: NotificationForm::load(),
                timer: (timer),
            });

            this.init();
            this
        }
    }

    unsafe fn init(self: &Rc<Self>) {
        self.timer.start_0a();

        // self.form
        //     .frame
        //     .set_frame_style(6);
        self.form
            .app_title_label
            .set_text(&QString::from_std_str("APPLICATION NAME"));
        self.form
            .title_label
            .set_text(&QString::from_std_str("Title"));
        self.form
            .body_label
            .set_text(&QString::from_std_str("Teeest some text here."));
    }

    pub fn show(self: &Rc<Self>) {
        unsafe {
            self.form.widget.show();
        }
    }

}

