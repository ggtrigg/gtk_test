extern crate gtk;
extern crate gdk;
extern crate gio;
extern crate gdk_pixbuf;

use gtk::prelude::*;
use gio::prelude::*;

use gtk::{Application, ApplicationWindow, Box, Orientation, GesturePan, GestureLongPress};
use gdk::EventMask;

fn main() {
    let application = Application::new(
        Some("com.github.gtk-rs.examples.basic"),
        Default::default(),
    ).expect("failed to initialize GTK application");

    application.connect_activate(|application| {
        let window = ApplicationWindow::new(application);
        window.set_title("First GTK+ Program");
        window.set_default_size(350, 70);
        
        let vbox = Box::new(Orientation::Vertical, 5);
        vbox.set_homogeneous(true);
        window.add(&vbox);
        vbox.add_events(EventMask::POINTER_MOTION_MASK | EventMask::BUTTON_PRESS_MASK | EventMask::BUTTON_RELEASE_MASK | EventMask::TOUCH_MASK);
        println!("Events: {:?}", vbox.get_events());
        
        let gesture = GesturePan::new(&vbox, gtk::Orientation::Horizontal);
        gesture.connect_pan(|gesture, pd, offset| {
            println!("Got pan gesture! {:?}, direction {}, offset {}", gesture, pd, offset);
        });
        gesture.set_propagation_phase(gtk::PropagationPhase::Bubble);
            
        let lp = GestureLongPress::new(&vbox);
        lp.set_propagation_phase(gtk::PropagationPhase::Bubble);
        lp.connect_pressed(|_glp, x, y| {
            println!("Got long press at {},{}", x, y);
        });

        window.show_all();
    });

    application.run(&[]);
}