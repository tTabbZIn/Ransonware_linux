use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Box as GtkBox, Orientation, CssProvider};
use gtk::gdk::Display;
use std::env;



const APP_ID: &str = "org.gtk_rs.HelloWorld2";

pub fn alert() -> glib::ExitCode {

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    

    app.run()
}

fn build_ui(app: &Application) {

    let msg = Button::builder()
        .label("Alerta: Seu computador foi infectado por um ransomware. Seus arquivos estão sendo criptografados. 
                    Não feche esta janela caso contrario sera irreversível.
                    Aguarde um tempo para obter mais instruções.")
        .build();
    msg.set_widget_name("msg");

    let load = Button::builder()
        .label("")
        .build();
    load.set_size_request(25, 50);
    load.set_widget_name("load");

    let boxMsg = GtkBox::new(Orientation::Horizontal, 0);
    boxMsg.append(&msg);

    let boxLoad = GtkBox::new(Orientation::Horizontal, 0);
    boxLoad.append(&load);








    let container1 = GtkBox::new(Orientation::Vertical, 0);
    container1.append(&boxMsg);
    container1.append(&boxLoad);


    let container_main = GtkBox::new(Orientation::Horizontal,0);

    container_main.append(&container1);
    container_main.set_widget_name("container");

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Alerta ")
        .child(&container_main)
        .build();


    window.present();
}

fn load_css() {

    let provider = CssProvider::new();

    provider.load_from_string(include_str!("styles/style.css"));


    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Não foi possível conectar a um display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

