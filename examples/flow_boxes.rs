mod data_set;
use std::str::FromStr;

use gtk4::gdk;
use gtk4::prelude::*;

fn main() {
    let application = gtk4::Application::new(
        Some("com.github.gtk-rs.examples.flowbox"),
        Default::default(),
    );

    application.connect_activate(build_ui);
    application.run();
}

fn build_ui(app: &gtk4::Application) {
    let window = gtk4::ApplicationWindow::builder()
        .default_width(600)
        .default_height(600)
        .application(app)
        .title("FlowBox")
        .build();

    let flow_box = gtk4::FlowBox::builder()
        .valign(gtk4::Align::Start)
        .max_children_per_line(30)
        .min_children_per_line(4)
        .selection_mode(gtk4::SelectionMode::None)
        .build();

    data_set::COLORS.iter().for_each(|color| {
        let color_widget = create_color_button(color);
        flow_box.insert(&color_widget, -1);
    });

    let scrolled_window = gtk4::ScrolledWindow::builder()
        .hscrollbar_policy(gtk4::PolicyType::Never) // Disable horizontal scrolling
        .min_content_width(360)
        .child(&flow_box)
        .build();

    window.set_child(Some(&scrolled_window));
    window.show();
}

fn create_color_button(color: &'static str) -> gtk4::Button {
    let button = gtk4::Button::new();
    let drawing_area = gtk4::DrawingArea::builder()
        .content_height(24)
        .content_width(24)
        .build();

    let rgba = gdk::RGBA::from_str(color).unwrap();
    drawing_area.set_draw_func(move |_, cr, _width, _height| {
        GdkCairoContextExt::set_source_rgba(cr, &rgba);
        cr.paint().expect("Invalid cairo surface state");
    });
    button.set_child(Some(&drawing_area));

    button
}