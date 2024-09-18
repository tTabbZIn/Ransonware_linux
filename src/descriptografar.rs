use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Button, Box as GtkBox, Orientation, CssProvider, Entry};
use gtk::gdk::Display;
use std::env;



const APP_ID: &str = "org.gtk_rs.HelloWorld2";

pub fn descripty() -> glib::ExitCode {

    let app = Application::builder().application_id(APP_ID).build();

    app.connect_startup(|_| load_css());
    app.connect_activate(build_ui);
    

    app.run()
}

fn build_ui(app: &Application) {


    let button_image = Button::builder()
        .build();
    button_image.set_size_request(300, 300);
    button_image.set_widget_name("btimage");

    let button_1 = Button::builder()
        .build();
    button_1.set_size_request(100, 100);
    button_1.set_widget_name("bt1");

    let button_2 = Button::builder()
        .build();

    button_2.set_size_request(100, 100);
    button_2.set_widget_name("bt2");


    let enviar = Button::builder()
        .label("Enviar")
        .build();

    enviar.set_size_request(50, 50);
    enviar.set_widget_name("enviar");


    let music_name = Button::builder()
        .label("
            Voce acaba de sofre um ransonware, resumindo,
                ele consiste em criptografar todos seus arquivos,
                videos, audios e fotos importantes. Mas se acalme, 
            basta digitar a chave abaixo que tudo sera descriptografado.
            ")
        .build();
    music_name.set_widget_name("music_name");


    let entry = Entry::new();
    entry.set_placeholder_text(Some("Digite sua chave..."));
    entry.set_size_request(250, 50);
    entry.set_widget_name("enviar_input");


    

    let boximage = GtkBox::new(Orientation::Horizontal, 0);
    boximage.append(&button_1);
    boximage.append(&button_image);
    boximage.append(&button_2);

    let box2 = GtkBox::new(Orientation::Vertical, 0);
    box2.append(&music_name);

    let box1 = GtkBox::new(Orientation::Horizontal, 0);
    box1.append(&entry);
    box1.append(&enviar);



    let container1 = GtkBox::new(Orientation::Vertical, 0);
    container1.append(&boximage);
    container1.append(&box2);
    container1.append(&box1);

    let container_main = GtkBox::new(Orientation::Horizontal,0);

    container_main.append(&container1);
    container_main.set_widget_name("container");

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Descriptografar")
        .child(&container_main)
        .build();


    window.present();
}

fn load_css() {

    let provider = CssProvider::new();

    provider.load_from_string(include_str!("styles/style2.css"));


    gtk::style_context_add_provider_for_display(
        &Display::default().expect("Não foi possível conectar a um display."),
        &provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

