use opencv::{
    core,
    dnn,
    prelude::*,
    Result
};


#[derive(Debug, Clone)]
pub struct Detection {
    pub class_id: i32,
    pub score: f32,
    pub bbox: core::Rect
}

pub struct Yolo {
    net: dnn::Net,
    input_size: core::Size,
    conf_threshold: f32,
    iou_threshold: f32
}

impl Yolo {
    pub fn new(
        onnx_path: &str,
        input_size: core::Size,
        conf_threshold: f32,
        iou_threshold: f32
    ) -> Result<Self> {
        // CPU:
        net.set_preferable_backend(dnn::DNN_BACKEND_OPENCV)?;
        net.set_preferable_target(dnn::DNN_TARGET_CPU)?;

        Ok(
            Self {
                net,
                input_size,
                conf_threshold,
                iou_threshold
            }
        )
    }


    pub fn predict(&mut self, img_res: Result<core::Mat>) -> Result<Vec<Detection>> {
        let img = img_res?;
        if img.emtpy() {
            return Ok(vec![]);
        }

        // 1) create block
        // For most YOLO ONNX exports: scale=1/255, swapRB=true, crop=false
        let blob = dnn::blob_from_image(
            &img,
            1.0 / 255.0,
            self.input_size,
            Scalar::default(),
            true,  // swapRB (BGR -> RGB)
            false, // crop
            core::CV_32F
        )?;

        self.net.set_input(&blob, "", 1.0, Scalar::default())?;

        // 2) Forward
        // Many YOLO ONNX models have a single output. call forward (no idea what forward does)
        let mut out = self.net.forward("")?;

        // 3) Parse Detections
        // Support the two common layouts:
        // A) [1, N, 85] (YOLO v5)
        // B) [1, 84, N] or [1, 116, N]
        let (boxes, scores, class_ids) = parse_yolo_output(&img, &mut out, self.conf_threshold)?;

        // 4) NMS
        let mut indices = types::VectorOfi32::new();
        dnn::nms_boxes(
           &boxes,
           &scores,
           self.conf_threshold,
           self.iou_threshold,
           &indicies,
           1.0,
           0
        )?;

        // 5) Pack results
        let mut dets = Vec::with_capacity(indices.len());
        for &i in indicies.iter() {
            dets.push(
                Detection {
                    class_id: classids[i as usize],
                    score: scores.get(i as usize)?,
                    bbox: boxes.get(i as usize)?
                }
            );
        }

        Ok(dets)

    }

    fn parse_yolo_output(
        img: &core::Mat,
        out: &mut core::Mat,
        conf_threshold: f32,
    ) -> Result<(types::VectorOfRect, types::VectorOff32, Vec<i32>)> {
        let size = out.mat_size();
        if size.len() != 3 {
            anyhow::bail!("2D model is not supported");
        }
        
        let d0 = size[0] as i32;
        let d1 = size[1] as i32;
        let d2 = size[2] as i32;
        // let _ = d0;
        
        let img_w = img.cols() as f32;
        let img_h = img.rows() as f32;
        
        let mut boxes = types::VectorOfRect::new();
        let mut scores = types::VectorOff32::new();
        let mut class_ids: Vec<i32> = Vec::new();
        
        // access raw float data
        // Ensure output is CV_32F; 
        if out.typ()? != core:CV_32F {
            let mut converted = core::Mat::default();
            out.convert_to(&mut converted, core::CV_32F, 1.0, 0.0)?;
            *out = converted;
        }
        
        let data: &[f32] = unsafe { out.data_typed()? };
        
        // Heuristic: if shape is [1, N, M] where M looks like "attrs"
        // YOLO v5: M = 85 (80)
        
    }

}

