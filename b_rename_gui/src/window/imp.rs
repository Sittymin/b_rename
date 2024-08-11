use glib::subclass::InitializingObject;
use gtk::gio;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/io/github/sittymin/b_rename/window.ui")]
pub struct Window {}

// The central trait for subclassing a GObject
#[glib::object_subclass]
impl ObjectSubclass for Window {
    const NAME: &'static str = "BRenameGtkAppWindow";
    type Type = super::Window;
    type ParentType = gtk::ApplicationWindow;

    fn class_init(klass: &mut Self::Class) {
        klass.bind_template();
        // 绑定模板 UI 回调函数
        klass.bind_template_callbacks();
    }

    fn instance_init(obj: &InitializingObject<Self>) {
        obj.init_template();
    }
}

#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn close_windows(&self, _button: &gtk::Button) {
        self.obj().close();
    }
    #[template_callback]
    fn start_rename(&self, _button: &gtk::Button) {
        println!("好像开始重命名了");
    }
    #[template_callback]
    fn add_button_clicked(&self, button: &gtk::Button) {
        // 创建一个更长生命周期的绑定
        let obj = self.obj();
        let window = obj.upcast_ref::<gtk::Window>();

        button.set_label("正在选择文件...");
        window.set_sensitive(false); // 禁用整个窗口

        let file_dialog = gtk::FileDialog::builder()
            .title("选择文件或目录")
            .accept_label("确认")
            .build();

        // 创建窗口的弱引用
        let weak_window = window.downgrade();
        let weak_button = button.downgrade();

        file_dialog.open(Some(window), gio::Cancellable::NONE, move |result| {
            // 升级为强引用
            let Some(window) = weak_window.upgrade() else {
                return;
            };
            let Some(button) = weak_button.upgrade() else {
                return;
            };

            match result {
                Ok(file) => {
                    println!("选择的文件是: {:?}", file);
                    button.set_label("载入文件中...");
                }
                Err(err) => {
                    eprintln!("选择文件时发生错误: {err}");
                    button.set_label("添加项目");
                }
            }
            window.set_sensitive(true); // 重新启用窗口
        });
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {}

// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
