mod mines_generator;

use crate::mines_generator::*;
use gtk::gdk::{EventButton, EventMask};
use gtk::prelude::*;
use gtk::prelude::*;
use gtk::{glib, Align, Application, ApplicationWindow, Button, Grid};

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

    let mut buttons: Vec<Vec<Button>> = vec![vec![Button::with_label("Start"); 10]; 10];

    for i in 0..10 {
        for j in 0..10 {
            let label = String::from(i.to_string().as_str());
            buttons[i as usize][j as usize] = Button::with_label(label.as_str());

            buttons[i as usize][j as usize].add_events(EventMask::BUTTON_PRESS_MASK);                               // Conecta um sinal de evento de clique de botão do mouse
            buttons[i as usize][j as usize].connect_button_press_event(move |_, event| {
                handle_button_press(event, i, 0);
                glib::signal::Propagation::Stop
            });
            grid.attach(&buttons[i as usize][j as usize], i, j, 1, 1);
        }
    }

    window.show_all();
}

fn handle_button_press(event: &EventButton, row: i32, column: i32) {
    match event.button() {
        1 => println!("Botão ({}, {}) foi clicado com o botão ESQUERDO do mouse!", row + 1, column + 1),
        2 => println!("Botão ({}, {}) foi clicado com o botão do MEIO do mouse!", row + 1, column + 1),
        3 => println!("Botão ({}, {}) foi clicado com o botão DIREITO do mouse!", row + 1, column + 1),
        _ => println!("Botão ({}, {}) foi clicado com outro botão do mouse!", row + 1, column + 1),
    }
}