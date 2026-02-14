use opencv:: {
    highgui,
    prelude::*,
    videoio,
    core,
};

use std::io::{self, Write, BufRead};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use std::thread;


fn main() -> opencv::Result<()> {

    let cam_on = Arc::new(AtomicBool::new(true));
    let cam_on_clone = cam_on.clone();

    thread::spawn(move || {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            if line.unwrap().trim() == "exit" {
                print!("Goodbye!");
                io::stdout().flush().unwrap();
                cam_on_clone.store(false, Ordering::SeqCst);
                break;
            }
            else {
                print!("Type 'exit' to quit: ");
                io::stdout().flush().unwrap();
            }
        }
    });

    // open webcam
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2)?;
    if !videoio::VideoCapture::is_opened(&cam)? {
        panic!("Camera was not opened")
    }

    // display webcam
    highgui::named_window("Video", highgui::WINDOW_AUTOSIZE)?;
    
    print!("Type 'exit' to quit: ");
    io::stdout().flush().unwrap(); // Ensure prompt appears

    while cam_on.load(Ordering::SeqCst) {
        let mut frame = Mat::default();
        cam.read(&mut frame)?;

        if frame.size()?.width > 0 {
            let _ = highgui::imshow("Video", &frame);
        }
        
        highgui::wait_key(1)?;

    }

    Ok(())
}
