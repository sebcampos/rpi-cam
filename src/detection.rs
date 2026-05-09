use opencv::{core, dnn, prelude::*, Result};

#[derive(Debug, Clone)]
pub struct Detection {
    pub class_id: i32,
    pub class_name: &'static str,
    pub score: f32,
    pub bbox: core::Rect
}

pub struct Yolo {
    net: dnn::Net,
    input_size: i32,
    conf_threshold: f32,
    iou_threshold: f32
}

const COCO_CLASSES: [&str; 80] = [
    "person", "bicycle", "car", "motorcycle", "airplane",
    "bus", "train", "truck", "boat", "traffic light",
    "fire hydrant", "stop sign", "parking meter", "bench", "bird",
    "cat", "dog", "horse", "sheep", "cow",
    "elephant", "bear", "zebra", "giraffe", "backpack",
    "umbrella", "handbag", "tie", "suitcase", "frisbee",
    "skis", "snowboard", "sports ball", "kite", "baseball bat",
    "baseball glove", "skateboard", "surfboard", "tennis racket", "bottle",
    "wine glass", "cup", "fork", "knife", "spoon",
    "bowl", "banana", "apple", "sandwich", "orange",
    "broccoli", "carrot", "hot dog", "pizza", "donut",
    "cake", "chair", "couch", "potted plant", "bed",
    "dining table", "toilet", "tv", "laptop", "mouse",
    "remote", "keyboard", "cell phone", "microwave", "oven",
    "toaster", "sink", "refrigerator", "book", "clock",
    "vase", "scissors", "teddy bear", "hair drier", "toothbrush"
];

impl Yolo {
    pub fn new(
        onnx_path: &str,
        input_size: i32,
        conf_threshold: f32,
        iou_threshold: f32
    ) -> Result<Self> {

        let mut net = dnn::read_net_from_onnx(onnx_path)?;

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


    pub fn predict(&mut self, img: Mat) -> Result<Vec<Detection>> {
        if img.empty() {
            return Ok(vec![]);
        }

        let size = core::Size::new(self.input_size, self.input_size);

        // 1) create block
        // For most YOLO ONNX exports: scale=1/255, swapRB=true, crop=false
        let blob = dnn::blob_from_image(
            &img,
            1.0 / 255.0,
            size,
            core::VecN([0.0, 0.0, 0.0, 0.0]),
            true,  // swapRB (BGR -> RGB)
            false, // crop
            core::CV_32F
        )?;

        self.net.set_input(&blob, "", 1.0, core::VecN([0.0, 0.0, 0.0, 0.0]))?;

        // 2) Forward
        // Many YOLO ONNX models have a single output. call forward (no idea what forward does)
        let mut out = self.net.forward_single_def()?;

        // 3) Parse Detections
        // Support the two common layouts:
        // A) [1, N, 85] (YOLO v5)
        // B) [1, 84, N] or [1, 116, N]
        let (boxes, scores, class_ids) = Self::parse_yolo_output(&img, &mut out, self.conf_threshold)?;

        // 4) NMS
        let mut indices =  core::Vector::<i32>::new();
        dnn::nms_boxes(
           &boxes,
           &scores,
           self.conf_threshold,
           self.iou_threshold,
           &mut indices,
           1.0,
           0
        )?;

        // 5) Pack results
        let mut dets = Vec::with_capacity(indices.len());
        for i in indices.iter() {
            dets.push(
                Detection {
                    class_id: class_ids.get(i as usize)?,
                    score: scores.get(i as usize)?,
                    bbox: boxes.get(i as usize)?,
                    class_name: COCO_CLASSES[class_ids.get(i as usize)? as usize],
                }
            );
        }

        Ok(dets)

    }

    fn parse_yolo_output(
        img: &Mat,
        out: &mut Mat,
        conf_threshold: f32,
    ) -> Result<(core::Vector<core::Rect>, core::Vector<f32>, core::Vector<i32>)> {
        if out.typ() != core::CV_32F {
            let mut converted = core::Mat::default();
            out.convert_to(&mut converted, core::CV_32F, 1.0, 0.0)?;
            *out = converted;
        }

        let dims = out.dims();
        if dims != 3 {
            // anyhow::bail!("Expected 3D YOLO output, got {dims}D");
        }

        let (d1, d2) = {
            let size = out.mat_size();
            (size[1] as usize, size[2] as usize)
        };

        let img_w = img.cols() as f32;
        let img_h = img.rows() as f32;

        let mut boxes = core::Vector::<core::Rect>::new();
        let mut scores = core::Vector::<f32>::new();
        let mut class_ids = core::Vector::<i32>::new();

        let data: &[f32] = unsafe { out.data_typed()? };

        let (num_boxes, attrs, transposed) = if d1 > d2 {
            (d1, d2, false) // [1, N, attrs]
        } else {
            (d2, d1, true) // [1, attrs, N]
        };

        for i in 0..num_boxes {
            let get = |a: usize| -> f32 {
                if !transposed {
                    data[i * attrs + a]
                } else {
                    data[a * num_boxes + i]
                }
            };

            let cx = get(0);
            let cy = get(1);
            let w = get(2);
            let h = get(3);

            let mut best_class = -1;
            let mut best_score = 0.0f32;

            for c in 4..attrs {
                let s = get(c);
                if s > best_score {
                    best_score = s;
                    best_class = (c - 4) as i32;
                }
            }

            let conf = best_score;
            if conf < conf_threshold {
                continue;
            }

            let input_size = 250.0;
            let x_factor = img_w / input_size;
            let y_factor = img_h / input_size;

            let left = ((cx - w * 0.5) * img_w * x_factor) as i32;
            let top = ((cy - h * 0.5) * img_h * y_factor) as i32;
            let width = (w * x_factor) as i32;
            let height = (h * y_factor) as i32;

            boxes.push(core::Rect::new(left, top, width, height));
            scores.push(conf);
            class_ids.push(best_class);
        }

        Ok((boxes, scores, class_ids))
    }

}

