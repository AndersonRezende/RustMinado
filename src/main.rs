mod mines_generator;

use gtk::prelude::*;
use gtk::{Align, Application, ApplicationWindow, Button, Grid};
use crate::mines_generator::*;

fn main() {
    let lines: usize = 10;
    let columns: usize = 10;
    let mines_generator = MinesManager::new(lines, columns, 25);

    let app = Application::builder()
        .application_id("rust_minado")
        .build();

    app.connect_activate(build_ui);
    app.run();
}



fn on_button_clicked(button: &Button) {
    eprintln!("Clicado na função!");
}

fn build_ui(app: &Application) {
    let window = ApplicationWindow::builder()
        .application(app)
        .default_width(600)
        .default_height(800)
        .title("Rust Minado!")
        .build();

    let icon_path = "assets/images/icons/logo.png";
    if window.set_icon_from_file(icon_path).is_err() {
        eprintln!("Erro ao carregar o icone: {}", icon_path)
    }

    let grid: Grid = Grid::builder()
        .margin(1)
        .halign(Align::Center)
        .valign(Align::Center)
        .row_homogeneous(true)
        .column_homogeneous(true)
        .row_spacing(5)
        .column_spacing(5)
        .build();

    window.set_child(Some(&grid));

    let mut buttons: Vec<Button> = vec![Button::with_label("Start"); 10];

    let button = Button::with_label("Botão");
    button.connect_clicked(|button| on_button_clicked(button));

    for i in 0..10 {
        let label = String::from(i.to_string().as_str());
        buttons[i as usize] = Button::with_label(label.as_str());

        grid.attach(&buttons[i as usize], i, 0, 1, 1);
    }

    window.show_all();
}