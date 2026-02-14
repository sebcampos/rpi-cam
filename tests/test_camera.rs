use tokio::time::{sleep, Duration};
use opencv::videoio::VideoWriterTrait;
use video_cam::camera;
use anyhow::Result;

#[tokio::test]
async fn test_open_camera() -> Result<()>
{
    let mut camera = camera::open_camera()?;
    let mut frame = camera::get_frame(&mut camera)?;
    let mut writer = camera::open_writer("./test.avi", &frame)?;
    let mut count = 0;

    for _ in 0..300 { // 10 seconds at 30 fps
        frame = camera::get_frame(&mut camera)?;
        writer.write(&frame)?;              // <- handle errors
        sleep(Duration::from_millis(33)).await;
    }
    Ok(())
}
