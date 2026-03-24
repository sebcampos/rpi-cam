use std::time::Duration;
use tokio::time::sleep;
use video_cam::detection::Yolo;
use video_cam::opencv_utils;

#[tokio::test]
async fn test_open_camera() -> anyhow::Result<()>
{
    let mut camera = opencv_utils::open_camera()?;
    let mut frame = opencv_utils::get_frame(&mut camera)?;
    let mut writer = opencv_utils::open_writer("./test.avi", &frame)?;
    let mut prediction_model = Yolo::new("../yolov8n_saved_model/yolov8n_float16.tflite", 256, 0.50, 0.50).unwrap();
    for _ in 0..300 { // 10 seconds at 30 fps
        frame = opencv_utils::get_frame(&mut camera)?;
        let detection = prediction_model.predict(frame)?;
        print!("Class id: {}" , detection[0].class_id);
        sleep(Duration::from_millis(33)).await;
    }
    Ok(())
}