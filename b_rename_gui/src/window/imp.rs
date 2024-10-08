use b_rename_core::dir::InputDir;
use glib::subclass::InitializingObject;
use gtk::gio;
use gtk::prelude::*;
use gtk::subclass::prelude::*;
use gtk::{glib, CompositeTemplate};
use std::process;

use b_rename_core::dir::Dir;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct GUIData {
    base_dir: Option<Dir>,
    output_dir: Option<Dir>,
}

impl GUIData {
    fn set_base_dir(&mut self, dir: Dir) {
        self.base_dir = Some(dir);
    }
    fn set_output_dir(&mut self, dir: Dir) {
        self.output_dir = Some(dir);
    }
    fn check_get_data(&self) -> Option<Self> {
        match (&self.base_dir, &self.output_dir) {
            // may be need in gui? can't move out
            (Some(_), Some(_)) => Some(self.clone()),
            _ => None,
        }
    }
}
impl Default for GUIData {
    fn default() -> Self {
        Self {
            base_dir: None,
            output_dir: None,
        }
    }
}
impl Clone for GUIData {
    fn clone(&self) -> Self {
        GUIData {
            base_dir: self.base_dir.clone(),
            output_dir: self.output_dir.clone(),
        }
    }
}

// Object holding the state
#[derive(CompositeTemplate, Default)]
#[template(resource = "/io/github/sittymin/b_rename/window.ui")]
pub struct Window {
    // 字段名要是 UI 文件中的 id
    // TemplateChild 表示在初始化之后才可访问
    #[template_child]
    pub left_stack: TemplateChild<gtk::Stack>,
    #[template_child]
    pub right_stack: TemplateChild<gtk::Stack>,
    #[template_child]
    pub left_list: TemplateChild<gtk::ListView>,
    #[template_child]
    pub right_list: TemplateChild<gtk::ListView>,
    #[template_child]
    pub add_base_dir_button: TemplateChild<gtk::Button>,
    #[template_child]
    pub add_modify_dir_button: TemplateChild<gtk::Button>,

    pub gui_data: Rc<RefCell<GUIData>>,
}

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
impl Window {
    fn init_list_view(&self, list: &gtk::ListView) {
        // 创建数据
        let model = gtk::StringList::new(&[]);
        let factory = gtk::SignalListItemFactory::new();

        // 初始化显示方式
        factory.connect_setup(glib::clone!(
            #[weak(rename_to = obj)]
            self,
            #[weak]
            list,
            move |_, list_item| {
                let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();

                let box_ = gtk::Box::new(gtk::Orientation::Horizontal, 6);
                let label = gtk::Label::new(None);

                let copy_button = gtk::Button::from_icon_name("edit-copy-symbolic");
                let delete_button = gtk::Button::from_icon_name("user-trash-symbolic");

                label.set_hexpand(true);
                label.set_halign(gtk::Align::Start);
                // 超出范围文本的省略位置
                label.set_ellipsize(gdk::pango::EllipsizeMode::Middle);

                box_.append(&label);
                box_.append(&copy_button);
                box_.append(&delete_button);
                list_item.set_child(Some(&box_));

                // 拖动逻辑
                // 为列表视图添加 DropTarget, 外部文件拖入
                let drop_target =
                    gtk::DropTarget::new(gio::File::static_type(), gdk::DragAction::COPY);
                drop_target.connect_drop(glib::clone!(
                    #[weak]
                    obj,
                    #[weak]
                    list,
                    #[upgrade_or]
                    false,
                    move |_, value, _, _| {
                        if let Ok(file) = value.get::<gio::File>() {
                            if let Some(path) = file.path() {
                                let mut gui_data = obj.gui_data.borrow_mut();
                                let dir = if list == obj.left_list.get() {
                                    gui_data.base_dir.as_mut().unwrap()
                                } else {
                                    gui_data.output_dir.as_mut().unwrap()
                                };

                                match dir.add_new_file(path) {
                                    Ok(()) => {
                                        update_list_ui(&list, dir);
                                        true
                                    }
                                    Err(e) => {
                                        eprintln!("Error: {e}");
                                        false
                                    }
                                }
                            } else {
                                false
                            }
                        } else {
                            false
                        }
                    }
                ));
                list.add_controller(drop_target);
            }
        ));

        // 滚动，更新列表时的重新渲染各个列表项
        factory.connect_bind(move |_, list_item| {
            let list_item = list_item.downcast_ref::<gtk::ListItem>().unwrap();

            let box_ = list_item.child().unwrap().downcast::<gtk::Box>().unwrap();
            let label = box_
                .first_child()
                .unwrap()
                .downcast::<gtk::Label>()
                .unwrap();
            let copy_button = box_
                .first_child()
                .unwrap()
                // 下一个
                .next_sibling()
                .unwrap()
                .downcast::<gtk::Button>()
                .unwrap();
            let delete_button = box_
                .last_child()
                .unwrap()
                .downcast::<gtk::Button>()
                .unwrap();

            let file_name = list_item
                .item()
                .unwrap()
                .downcast::<gtk::StringObject>()
                .unwrap();

            label.set_label(&file_name.string());

            // 设置复制按钮的点击事件
            copy_button.connect_clicked(move |_| {
                let file_name = file_name.clone();
                if let Some(display) = gdk::Display::default() {
                    let clipboard = display.clipboard();
                    clipboard.set_text(&file_name.string());
                } else {
                    eprintln!("无法获取剪贴板");
                    // TODO: 显示一个Dialog
                }
            });

            // TODO: 设置删除按钮的点击事件
            delete_button.connect_clicked(move |_| print!("待办"));
        });

        // 创建选择包含容器
        let selection_model = gtk::SingleSelection::new(Some(model));
        // 设置最终 List
        list.set_model(Some(&selection_model));
        list.set_factory(Some(&factory));
    }
}

// 回调部分
#[gtk::template_callbacks]
impl Window {
    #[template_callback]
    fn close_windows(&self, _button: &gtk::Button) {
        self.obj().close();
    }
    #[template_callback]
    fn start_rename(&self, _button: &gtk::Button) {
        // TODO: Need imp Same Dir file rename
        match self.gui_data.borrow().check_get_data() {
            Some(data) => {
                if let (Some(base_dir), Some(modify_dir)) = (data.base_dir, data.output_dir) {
                    let output_dir_path = modify_dir.dir_path.clone();
                    let mut input_dir = InputDir::new(base_dir, modify_dir);
                    match input_dir.output_rename(output_dir_path, false) {
                        Ok(_) => {
                            self.right_stack.get().set_visible_child_name("empty");

                            let button_content = adw::ButtonContent::new();
                            button_content.set_icon_name("document-open-symbolic");
                            // button_content.add_css_class("loading-icon");
                            button_content.set_label("添加修改文件夹");
                            self.add_modify_dir_button
                                .get()
                                .set_child(Some(&button_content));

                            let alert = gtk::AlertDialog::builder()
                                .message("操作已完成")
                                .modal(true)
                                .build();

                            alert.show(Some(self.obj().upcast_ref::<gtk::Window>()));
                        }
                        Err(e) => {
                            eprintln!("{e}");
                            // TEMP: if get this need back to set dir
                            process::exit(1);
                        }
                    };
                }
            }
            _ => eprintln!("No Full Gui Data!"),
        }
    }
    #[template_callback]
    fn add_button_clicked(&self, button: &gtk::Button) {
        let obj = self.obj();
        let window = obj.upcast_ref::<gtk::Window>();

        // 存储 child 之后用来还原用(后续已经不可用child)
        let before_button_child = button.child();

        // 提前将结构体子段提取，避免生命周期问题
        let (stack, list, is_left_box) = match button.widget_name().as_str() {
            "add_base_dir_button" => (self.left_stack.get(), self.left_list.get(), true),
            "add_modify_dir_button" => (self.right_stack.get(), self.right_list.get(), false),
            _ => panic!("Unexpected button name: {}", button.widget_name()),
        };

        let button_content = adw::ButtonContent::new();
        button_content.set_icon_name("process-working-symbolic");
        button_content.add_css_class("loading-icon");
        button_content.set_label("选择文件中...");
        button.set_child(Some(&button_content));
        // 冻结窗口
        window.set_sensitive(false);

        let file_dialog = gtk::FileDialog::builder()
            .title("选择文件或目录")
            .accept_label("确认")
            .build();

        file_dialog.select_folder(
            Some(window),
            gio::Cancellable::NONE,
            glib::clone!(
                #[weak]
                window,
                #[weak]
                button,
                #[weak]
                stack,
                #[weak]
                list,
                #[weak(rename_to = gui_data)]
                self.gui_data,
                #[upgrade_or]
                return,
                move |result| {
                    let mut gui_data = gui_data.borrow_mut();

                    match result {
                        Ok(file) => {
                            if let Some(path) = file.path() {
                                println!("选择的目录是路径是: {:?}", path);

                                // 存储获得的数据
                                let dir: &mut Dir;
                                if is_left_box {
                                    gui_data.set_base_dir(Dir::new(path));
                                    dir = gui_data
                                        .base_dir
                                        .as_mut()
                                        .expect("未能初始化self.base_dir");
                                } else {
                                    gui_data.set_output_dir(Dir::new(path));
                                    dir = gui_data
                                        .output_dir
                                        .as_mut()
                                        .expect("未能初始化self.output_dir");
                                }

                                list.model()
                                    .and_then(|model| model.downcast::<gtk::SingleSelection>().ok())
                                    .and_then(|selection_model| selection_model.model())
                                    .and_then(|model| model.downcast::<gtk::StringList>().ok())
                                    .map(|string_list| {
                                        dir.sort_files_by_name_unstable();
                                        let file_names: Vec<String> = dir
                                            .get_files_full_name()
                                            .into_iter()
                                            .map(|file_name| {
                                                file_name.to_string_lossy().into_owned()
                                            })
                                            .collect();

                                        // 添加进列表
                                        for file_name in file_names.iter() {
                                            string_list.append(&file_name);
                                        }
                                    })
                                    .unwrap_or_else(|| {
                                        eprintln!("无法获取或处理列表模型");
                                    });

                                stack.set_visible_child_name("list");
                            }
                        }
                        Err(err) => {
                            println!("{:?}", err);
                            if let Some(child) = before_button_child {
                                button.set_child(Some(&child));
                            } else {
                                button.set_child(None::<&gtk::Widget>);
                                eprintln!("代码逻辑错误");
                            }
                        }
                    }
                    window.set_sensitive(true);
                }
            ),
        );
    }
}

fn update_list_ui(list: &gtk::ListView, dir: &Dir) {
    if let Some(string_list) = list
        .model()
        .and_then(|model| model.downcast::<gtk::SingleSelection>().ok())
        .and_then(|selection_model| selection_model.model())
        .and_then(|model| model.downcast::<gtk::StringList>().ok())
    {
        let file_names = dir.get_files_full_name();
        let str_slice: Vec<&str> = file_names
            .iter()
            .map(|os_string| os_string.to_str().unwrap_or("此文件名转换失败"))
            .collect();

        string_list.splice(0, string_list.n_items(), &str_slice);
    }
}

// Trait shared by all GObjects
impl ObjectImpl for Window {
    fn constructed(&self) {
        self.parent_constructed();
        // 初始化列表
        self.init_list_view(&self.left_list.get());
        self.init_list_view(&self.right_list.get());
    }
}
// Trait shared by all widgets
impl WidgetImpl for Window {}

// Trait shared by all windows
impl WindowImpl for Window {}

// Trait shared by all application windows
impl ApplicationWindowImpl for Window {}
