mod speedometer;
mod config;
mod util;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use config::read::read_config;
use glib::ControlFlow;
use gtk4::{self as gtk, DrawingArea, Grid};
use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow};
use speedometer::lib::on_draw;

fn main() -> glib::ExitCode {
    let app = Application::builder()
        .application_id("org.example.HelloWorld")
        .build();

    app.connect_activate(|app| {
        // We create the main window.
        let window = ApplicationWindow::builder()
            .application(app)
            .default_width(800)
            .default_height(800)
            .halign(gtk::Align::Center)
            .valign(gtk::Align::Center)
            .title("Dashboard")
            .build();

        // Show the window.
        let grid = Grid::builder()
        .hexpand(true)
        .vexpand(true)
        .build();

        let config = read_config();
        let config = Arc::new(config.speedometer.to_decimal());

        // width_request and height_request need to be set to config values
        let canvas = DrawingArea::builder()
        .width_request(800)
        .height_request(800)
        .build();
        grid.attach(&canvas, 1, 1, 1, 1);

        let button_1 = gtk::Button::with_label("Left");
        button_1.connect_clicked(move |_| println!("TURN LEFT !!!!!"));
        grid.attach(&button_1, 0, 2, 1, 1);
        let button_2 = gtk::Button::with_label("Right");
        button_1.connect_clicked(move |_| println!("TURN RIGHT !!!!"));
        grid.attach(&button_2, 2, 2, 1, 1);
        

        //tf2 coconut
        let speed = Arc::new(Mutex::new(0.5));
        let speed2 = speed.clone();

        canvas.set_draw_func(move |_, cr, _, _| {
            on_draw(cr, config.clone(), speed.clone());
        });
        window.set_child(Some(&grid));
        glib::source::timeout_add_local(Duration::from_millis(10), move| | {
            interval(&canvas, speed2.clone())
        });
        window.present();
    });

    app.run()
}

//kill everyone

fn interval( debra :&DrawingArea, speed: Arc<Mutex<f64>>) -> ControlFlow{
    let mut speed_loc = speed.lock().unwrap();
    *speed_loc = *speed_loc * 1.01;
    debra.queue_draw();
    return ControlFlow::Continue;
}
