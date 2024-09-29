mod mines_generator;

use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Button};

fn main() {

    let app = Application::builder()
        .application_id("rust_minado")
        .build();

    app.connect_activate(|app| {
       let win = ApplicationWindow::builder()
           .application(app)
           .default_width(600)
           .default_height(800)
           .title("Rust Minado!")
           .build();

        let icon_path = "assets/images/icons/logo.png";
        if win.set_icon_from_file(icon_path).is_err() {
            eprintln!("Erro ao carregar o icone: {}", icon_path)
        }

        let buttons: Vec<Button>;

        let button = Button::with_label("Botão");
        button.connect_clicked(|button| on_button_clicked(button));
        win.add(&button);

        win.show_all();
    });

    app.run();
}



fn on_button_clicked(button: &Button) {
    eprintln!("Clicado na função!");
}