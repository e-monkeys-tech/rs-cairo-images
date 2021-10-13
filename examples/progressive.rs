use cairo::{ Context, Format, ImageSurface }; // FontSlant, FontWeight };
use std::f64::consts::PI;
use std::fs::File;
use std::error::Error;

fn main() {
    let surface = ImageSurface::create(Format::ARgb32, 600, 600).expect("Couldn’t create surface");
    let context = Context::new(&surface);
        context.scale(600f64, 600f64);
        context.set_source_rgb(0.0/255.0, 0.0/255.0, 0.0/255.0); // Black background
        context.rectangle(0.0, 0.0, 600f64, 600f64);
        context.fill();
        context.arc(0.50, 0.50, 0.40, 0.0, PI * 2.); // (50% RIGHT, 50% LEFT, radius, angle_1=0, angle_2=PI * 2.) for central round arc
        context.fill();

        let mut file = File::create("data/smiley.png")
        .expect("Couldn’t create file"); 
        match surface.write_to_png(&mut file) {
            Ok(_) => println!("data/smiley.png created"),
            Err(_) => println!("Error create file.png"),
        }    
}