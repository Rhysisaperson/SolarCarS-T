mod speedometer;
mod config;
mod util;

use std::sync::{Arc, Mutex};
use std::time::Duration;
use config::read::read_config;
//use config::structs::Speedometer;
use glib::ControlFlow;
use gtk4::{self as gtk, DrawingArea};
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
            .title("Dashboard")
            .build();

        // Show the window.
        let canvas = DrawingArea::default();
        //let config = Arc::new(Speedometer::defualt());
        let config = read_config();
        if config.is_none() {
            panic!("no config file meownd")
        }
        let config = Arc::new(config.unwrap().speedometer);
        let speed = Arc::new(Mutex::new(0.5));
        let speed2 = speed.clone();

        canvas.set_draw_func(move |_, cr, _, _| {
            on_draw(cr, config.clone(), speed.clone());
        });
        window.set_child(Some(&canvas));
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
