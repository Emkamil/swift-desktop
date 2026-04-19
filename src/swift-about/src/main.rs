// Copyright (C) 2026  Kamil Machowski
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.


mod contributors;
mod metadata;
use gettextrs::*;
use gtk4::prelude::*;
use gtk4::{
    glib, Application, ApplicationWindow, Box as GtkBox, Button, Grid, Image, Label, 
    Orientation, ScrolledWindow, SizeGroup, SizeGroupMode, TextBuffer, TextView, 
    Stack, StackSwitcher, ListBox, ListBoxRow, Align, WrapMode
};
use std::{env, fs};

fn main() -> glib::ExitCode {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|arg| arg == "--version" || arg == "-v") {
        println!("Version: {}", metadata::SWIFT_VERSION);
        println!("Codename: {}", metadata::SWIFT_CODENAME);
        return glib::ExitCode::SUCCESS;
    }

    setlocale(LocaleCategory::LcAll, "");
    let _ = bindtextdomain("swift-about", "/usr/share/locale");
    let _ = textdomain("swift-about");

    gtk4::init().expect("Nie udało się zainicjować GTK4");
    let app = Application::builder().application_id("swift-about").build();

    app.connect_activate(build_ui);
    app.run()
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .title(gettext("About"))
        .default_width(820)
        .default_height(620)
        .build();

    let main_box = GtkBox::new(Orientation::Vertical, 0);

    let header_bar = gtk4::HeaderBar::new();
    
    let view_stack = Stack::builder()
        .transition_type(gtk4::StackTransitionType::SlideLeftRight)
        .vexpand(true)
        .hexpand(true)
        .build();

    let view_switcher = StackSwitcher::new();
    view_stack.set_vexpand(true);
    view_stack.set_hexpand(true);
    view_switcher.set_stack(Some(&view_stack));
    view_switcher.set_can_focus(false);
    view_switcher.set_focus_on_click(false);
    
    let switcher_box = GtkBox::new(Orientation::Horizontal, 0);
    switcher_box.add_css_class("linked");
    switcher_box.append(&view_switcher);

    header_bar.set_title_widget(Some(&switcher_box));
    window.set_titlebar(Some(&header_bar));

    let sys_holder = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true) 
        .hexpand(true)
        .build();

    let info_holder = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .hexpand(true)
        .build();   

    let credits_holder = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .hexpand(true)
        .build();
    let copy_holder = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .vexpand(true)
        .hexpand(true)
        .build();

    view_stack.add_titled(&sys_holder, Some("system"), &gettext("System"));
    view_stack.add_titled(&info_holder, Some("information"), &gettext("Information"));
    view_stack.add_titled(&credits_holder, Some("credits"), &gettext("Credits"));
    view_stack.add_titled(&copy_holder, Some("copyright"), &gettext("Licenses"));

    // 7. Lazy Loading logic
    view_stack.connect_visible_child_name_notify(move |stack| {
        if let Some(name) = stack.visible_child_name() {
            if let Some(child) = stack.visible_child() {
                let holder = child.downcast_ref::<GtkBox>().expect("Must be a GtkBox");
                if holder.first_child().is_none() {
                    match name.as_str() {
                        "information" => holder.append(&create_information_page()),
                        "credits" => holder.append(&create_credits_page()),
                        "copyright" => holder.append(&create_copyright_page()),
                        _ => {}
                    }
                }
            }
        }
    });

    sys_holder.append(&create_system_page());

    main_box.append(&view_stack);
    window.set_child(Some(&main_box));
    window.present();
}
fn create_system_page() -> ScrolledWindow {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .build();

    let content_box = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(32)
        .margin_bottom(32)
        .margin_start(32)
        .margin_end(32)
        .halign(Align::Center)
        .vexpand(true)
        .hexpand(true)
        .build();

    let title_label = Label::builder()
        .label(&gettext("System Information"))
        .halign(Align::Start)
        .css_classes(vec!["title-1".to_string()])
        .build();
    content_box.append(&title_label);

    let content_grid = Grid::new();
    content_grid.set_column_spacing(40);
    content_grid.set_row_spacing(12);

    let logo = Image::builder()
        .icon_name(get_os_logo())
        .pixel_size(128)
        .halign(Align::Start)
        .valign(Align::Start)
        .build();
    content_grid.attach(&logo, 0, 0, 1, 1);

    let info_grid = Grid::new();
    info_grid.set_column_spacing(20);
    info_grid.set_row_spacing(8);

    let label_size_group = SizeGroup::new(SizeGroupMode::Horizontal);

    struct InfoRow {
        label: String,
        value: String,
        is_header_start: bool,
    }

    let info_rows = vec![
        InfoRow { label: gettext("Device"), value: get_hostname(), is_header_start: false },
        InfoRow { label: gettext("Operating System"), value: read_os_release().unwrap_or_default(), is_header_start: false },
        InfoRow { label: gettext("System Type"), value: if cfg!(target_pointer_width = "64") { "64-bit".into() } else { "32-bit".into() }, is_header_start: false },
        InfoRow { label: gettext("Swift Desktop Version"), value: format!("{} ({})", metadata::SWIFT_VERSION, metadata::SWIFT_CODENAME), is_header_start: true },
        InfoRow { label: gettext("Kernel Version"), value: get_kernel_version(), is_header_start: false },
        InfoRow { label: gettext("Windowing System"), value: detect_windowing_system(), is_header_start: false },
        InfoRow { label: gettext("Processor"), value: get_cpu_model(), is_header_start: true },
        InfoRow { label: gettext("Memory"), value: get_total_memory(), is_header_start: false },
    ];

    for (i, row) in info_rows.into_iter().enumerate() {
        let key_label = Label::builder()
            .label(&row.label)
            .halign(Align::Start)
            .xalign(0.0)
            .build();
        key_label.add_css_class("dim-label");
        label_size_group.add_widget(&key_label);

        let value_label = Label::builder()
            .label(&row.value)
            .halign(Align::Start)
            .xalign(0.0)
            .selectable(true)
            .can_focus(false)
            .build();

        if row.is_header_start {
            key_label.set_margin_top(20);
            value_label.set_margin_top(20);
        }

        info_grid.attach(&key_label, 0, i as i32, 1, 1);
        info_grid.attach(&value_label, 1, i as i32, 1, 1);
    }

    content_grid.attach(&info_grid, 1, 0, 1, 1);
    content_box.append(&content_grid);
    scrolled.set_child(Some(&content_box));
    scrolled
}

fn create_information_page() -> ScrolledWindow {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .build();

    let vbox = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(24)
        .margin_top(32)
        .margin_bottom(32)
        .margin_start(32)
        .margin_end(32)
        .halign(Align::Center)
        .vexpand(true)
        .hexpand(true)
        .build();

    let logo = Image::builder().icon_name("swift-about").pixel_size(128).build();
    vbox.append(&logo);

    let desc = Label::builder()
        .label(gettext("Swift is a set of programs that together provide a fully functional desktop environment."))
        .wrap(true)
        .justify(gtk4::Justification::Center)
        .max_width_chars(60)
        .build();
    vbox.append(&desc);

    let components = vec![
        ("preferences-system-windows", "Window Manager", "labwc", "Manages the placement of windows."),
        ("swift-panel", "Panel", "swift-panel", "Provides a place for window buttons."),
        ("swift-desktop-view", "Desktop Manager", "swift-desktop", "Sets backgrounds and icons."),
        ("swift-file-manager", "File Manager", "nemo", "Manages files in a modern way."),
    ];

    for (icon, title, app, desc_text) in components {
        let comp_hbox = GtkBox::new(Orientation::Horizontal, 16);
        let comp_icon = Image::builder().icon_name(icon).pixel_size(48).build();
        let comp_vbox = GtkBox::new(Orientation::Vertical, 4);

        let comp_title = Label::builder()
            .label(format!("{} ({})", gettext(title), app))
            .halign(Align::Start)
            .build();
        comp_title.add_css_class("heading");

        let comp_desc = Label::builder()
            .label(gettext(desc_text))
            .wrap(true)
            .halign(Align::Start)
            .build();
        comp_desc.add_css_class("dim-label");

        comp_vbox.append(&comp_title);
        comp_vbox.append(&comp_desc);
        comp_hbox.append(&comp_icon);
        comp_hbox.append(&comp_vbox);
        vbox.append(&comp_hbox);
    }

    scrolled.set_child(Some(&vbox));
    scrolled
}

fn create_credits_page() -> ScrolledWindow {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .build();

    let vbox = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_start(32)
        .margin_end(32)
        .margin_top(32)
        .margin_bottom(32)
        .vexpand(true)
        .hexpand(true)
        .build();

    for group_data in contributors::SWIFT_CONTRIBUTORS {
    let group_container = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .halign(Align::Center)
        .width_request(600)  
        .build();

    let group_label = Label::builder()
        .label(group_data.name)
        .halign(Align::Start) 
        .margin_top(12)
        .build();
    group_label.add_css_class("title-3");
    group_container.append(&group_label);

    let list_box = ListBox::builder()
        .selection_mode(gtk4::SelectionMode::None)
        .css_classes(vec!["boxed-list".to_string()])
        .build();

    for contributor in group_data.contributors {
        let row_box = GtkBox::new(Orientation::Vertical, 4);
        row_box.set_margin_start(12);
        row_box.set_margin_end(12);
        row_box.set_margin_top(12);
        row_box.set_margin_bottom(12);

        let name_label = Label::builder()
            .label(contributor.name)
            .halign(Align::Start)
            .build();

        let email_label = Label::builder()
            .label(contributor.email)
            .halign(Align::Start)
            .build();
        email_label.add_css_class("dim-label");

        row_box.append(&name_label);
        row_box.append(&email_label);
        list_box.append(&row_box);
    }

    group_container.append(&list_box);
    vbox.append(&group_container);
}
    scrolled.set_child(Some(&vbox));
    scrolled
}

fn create_copyright_page() -> ScrolledWindow {
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .hexpand(true)
        .build();
        
    let vbox = GtkBox::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_start(32)
        .margin_end(32)
        .margin_top(32)
        .margin_bottom(32)
        .vexpand(true)
        .hexpand(true)
        .build();

    let info_label = Label::builder()
        .label(gettext("Swift components are licensed under:"))
        .halign(Align::Start)
        .build();
    vbox.append(&info_label);

    let list_box = ListBox::new();
    list_box.set_selection_mode(gtk4::SelectionMode::None);
    list_box.add_css_class("boxed-list");

    let licenses = [
        ("GNU General Public License v3.0", "gpl-3.0"),
        ("GNU Lesser General Public License", "lgpl-3.0"),
        ("The BSD 3-Clause License", "bsd-3-clause"),
    ];

    for (full_name, file_key) in licenses {
        let row = ListBoxRow::new();
        let row_hbox = GtkBox::new(Orientation::Horizontal, 12);
        row_hbox.set_margin_start(12);
        row_hbox.set_margin_end(12);
        row_hbox.set_margin_top(12);
        row_hbox.set_margin_bottom(12);

        let label = Label::new(Some(full_name));
        let arrow = Image::from_icon_name("go-next-symbolic");
        
        row_hbox.append(&label);
        let filler = GtkBox::new(Orientation::Horizontal, 0);
        filler.set_hexpand(true);
        row_hbox.append(&filler);
        row_hbox.append(&arrow);

        row.set_child(Some(&row_hbox));
        
        let key = file_key.to_string();
        list_box.append(&row);
    }

    list_box.connect_row_activated(move |_, row| {
        let index = row.index();
        let licenses = [("gpl-3.0"), ("lgpl-3.0"), ("bsd-3-clause")];
        if let Some(key) = licenses.get(index as usize) {
             if let Some(window) = row.root().and_then(|r| r.downcast::<ApplicationWindow>().ok()) {
                let license_content = get_license_text(key);
                show_license_dialog(&window, &license_content);
            }
        }
    });

    vbox.append(&list_box);
    scrolled.set_child(Some(&vbox));
    scrolled
}

fn show_license_dialog(parent: &ApplicationWindow, license_text: &str) {
    let dialog = ApplicationWindow::builder()
        .transient_for(parent)
        .modal(true)
        .title(gettext("License Information"))
        .default_width(600)
        .default_height(700)
        .build();

    let main_vbox = GtkBox::new(Orientation::Vertical, 0);
    
    let scrolled = ScrolledWindow::builder()
        .vexpand(true)
        .margin_start(24)
        .margin_end(24)
        .margin_top(24)
        .margin_bottom(24)
        .build();

    let text_view = TextView::builder()
        .editable(false)
        .cursor_visible(false)
        .wrap_mode(WrapMode::Word)
        .justification(gtk4::Justification::Center)
        .build();

    let buffer = TextBuffer::new(None);
    buffer.set_text(license_text);
    text_view.set_buffer(Some(&buffer));
    scrolled.set_child(Some(&text_view));

    let close_button = Button::with_label(&gettext("Close"));
    close_button.set_margin_bottom(24);
    close_button.set_halign(Align::Center);
    close_button.set_width_request(120);
    
    close_button.connect_clicked(glib::clone!(#[weak] dialog, move |_| dialog.close()));

    main_vbox.append(&scrolled);
    main_vbox.append(&close_button);

    dialog.set_child(Some(&main_vbox));
    dialog.present();
}

fn read_os_release() -> Option<String> {
    let content = fs::read_to_string("/etc/os-release").ok()?;
    for line in content.lines() {
        if let Some(stripped) = line.strip_prefix("PRETTY_NAME=") {
            return Some(stripped.trim_matches('"').to_string());
        }
    }
    None
}

fn detect_windowing_system() -> String {
    let session_type = env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "Unknown".into()).to_lowercase();
    match session_type.as_str() {
        "wayland" => "Wayland".into(),
        "x11" => "X11".into(),
        _ => "Unknown".into(),
    }
}

fn get_hostname() -> String {
    fs::read_to_string("/proc/sys/kernel/hostname").ok().map(|v| v.trim().to_string()).unwrap_or_else(|| "Unknown".into())
}

fn get_kernel_version() -> String {
    fs::read_to_string("/proc/sys/kernel/osrelease").ok().map(|v| v.trim().to_string()).unwrap_or_else(|| "Unknown Kernel".into())
}

fn get_total_memory() -> String {
    fs::read_to_string("/proc/meminfo").ok().and_then(|content| {
        content.lines().find(|l| l.starts_with("MemTotal:")).map(|l| {
            let kb = l.split_whitespace().nth(1).and_then(|v| v.parse::<f64>().ok()).unwrap_or(0.0);
            format!("{:.1} GB", kb / 1024.0 / 1024.0)
        })
    }).unwrap_or_else(|| "N/A".into())
}

fn get_cpu_model() -> String {
    fs::read_to_string("/proc/cpuinfo").ok().and_then(|content| {
        content.lines().find(|l| l.starts_with("model name")).map(|l| l.split(':').nth(1).unwrap_or("N/A").trim().to_string())
    }).unwrap_or_else(|| "Unknown CPU".into())
}

fn get_os_logo() -> String {
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(stripped) = line.strip_prefix("LOGO=") {
                return stripped.trim_matches('"').trim_matches('\'').to_string();
            }
        }
    }
    "dialog-information".to_string()
}

fn get_license_text(name: &str) -> String {
    let path = format!("/usr/share/swift/licenses/{}.txt", name.to_lowercase());
    fs::read_to_string(&path).unwrap_or_else(|_| format!("Error: License file not found at {}", path))
}