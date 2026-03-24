use opencv::{prelude::*, videoio, core};
use opencv::videoio::VideoWriterTrait;
use anyhow::{bail, Result};

pub fn open_camera() -> Result<videoio::VideoCapture> {
    let mut cam = videoio::VideoCapture::new(0, videoio::CAP_V4L2)?;
    if !videoio::VideoCapture::is_opened(&cam)? {
        anyhow::bail!("Unable to open camera");
    }
    cam.set(videoio::CAP_PROP_FRAME_WIDTH, 640.0)?;
    cam.set(videoio::CAP_PROP_FRAME_HEIGHT, 480.0)?;
    Ok(cam)

}

pub fn get_frame(cam: &mut videoio::VideoCapture) -> Result<Mat> {
    let mut frame = Mat::default();
    cam.read(&mut frame)?;
    if frame.size()?.width == 0 {
        anyhow::bail!("Emtpy Frame");
    }
    Ok(frame)
}

pub fn open_writer(output_path: &str, frame: &Mat) -> opencv::Result<videoio::VideoWriter> {
    let fourcc = videoio::VideoWriter::fourcc('M' , 'J' , 'P' , 'G' )?;
    let fps = 30.0;
    let width = frame.cols();
    let height = frame.rows();
    let writer = videoio::VideoWriter::new(
            output_path,
            fourcc,
            fps,
            core::Size::new(width, height),
            true
        );
    Ok(writer?)
}
