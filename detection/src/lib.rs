use std::{cmp::Ordering, io::Cursor};
use std::time::{Duration, Instant};
use anyhow::Error;
use image::DynamicImage;

use tract_ndarray::s;
use tract_onnx::prelude::*;

#[derive(Debug, Clone)]
pub struct DetectionResult {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub confidence: f32,
}

pub fn detect(img: Vec<u8>, confidence: f32) -> Result<Vec<DetectionResult>, Error> {
    let now = Instant::now();
    let mut times: Vec<Duration> = vec![];
    let model = tract_onnx::onnx()
        .model_for_read(&mut Cursor::new(include_bytes!("../yolov8n-face.onnx")))?
        .with_input_fact(0, f32::fact([1, 3, 640, 640]).into())?
        .into_optimized()?
        .into_runnable()?;

    times.push(now.elapsed());
    let raw_image = image::load_from_memory(&img)?;
    times.push(now.elapsed());

    // scale the image with black padding
    let width = raw_image.width();
    let height = raw_image.height();
    let scale = 640.0 / width.max(height) as f32;
    let new_width = (width as f32 * scale) as u32;
    let new_height = (height as f32 * scale) as u32;
    let resized = image::imageops::resize(
        &raw_image.to_rgb8(),
        new_width,
        new_height,
        image::imageops::FilterType::Triangle,
    );
    times.push(now.elapsed());
    let mut padded = image::RgbImage::new(640, 640);
    times.push(now.elapsed());
    image::imageops::replace(
        &mut padded,
        &resized,
        (640 - new_width as i64) / 2,
        (640 - new_height as i64) / 2,
    );
    times.push(now.elapsed());
    let image: Tensor = tract_ndarray::Array4::from_shape_fn((1, 3, 640, 640), |(_, c, y, x)| {
        padded.get_pixel(x as u32, y as u32)[c] as f32 / 255.0
    })
        .into();

    times.push(now.elapsed());
    //run model
    //
    let forward = model.run(tvec![image.to_owned().into()])?;
    let results = forward[0].to_array_view::<f32>()?.view().t().into_owned();
    let mut bbox_vec: Vec<DetectionResult> = vec![];
    for i in 0..results.len_of(tract_ndarray::Axis(0)) {
        let row = results.slice(s![i, .., ..]);
        let confidence = row[[4, 0]];

        if confidence >= 0.5 {
            let x = row[[0, 0]];
            let y = row[[1, 0]];
            let w = row[[2, 0]];
            let h = row[[3, 0]];
            let x1 = x - w / 2.0;
            let y1 = y - h / 2.0;
            let x2 = x + w / 2.0;
            let y2 = y + h / 2.0;
            let bbox = DetectionResult::new(x1, y1, x2, y2, confidence)
                .apply_image_scale(&raw_image, 640.0, 640.0);
            bbox_vec.push(bbox);
        }
    }

    times.push(now.elapsed());
    println!("times: {:?}", times);

    Ok(non_maximum_suppression(bbox_vec, confidence))

    // Ok(DetectionResult {
    //     x1,
    //     y1,
    //     x2,
    //     y2,
    //     confidence,
    // })
    // let mut new_img = raw_image.clone();
    // draw_red_box(&mut new_img, x1, y1, x2, y2, "red_box.png".to_string());
    // todo!()
}

impl DetectionResult {
    pub fn new(x1: f32, y1: f32, x2: f32, y2: f32, confidence: f32) -> Self {
        Self {
            x1,
            y1,
            x2,
            y2,
            confidence,
        }
    }
    pub fn apply_image_scale(
        &mut self,
        original_image: &DynamicImage,
        x_scale: f32,
        y_scale: f32,
    ) -> Self {
        let normalized_x1 = self.x1 / x_scale;
        let normalized_x2 = self.x2 / x_scale;
        let normalized_y1 = self.y1 / y_scale;
        let normalized_y2 = self.y2 / y_scale;

        let cart_x1 = original_image.width() as f32 * normalized_x1;
        let cart_x2 = original_image.width() as f32 * normalized_x2;
        let cart_y1 = original_image.height() as f32 * normalized_y1;
        let cart_y2 = original_image.height() as f32 * normalized_y2;

        Self {
            x1: cart_x1,
            y1: cart_y1,
            x2: cart_x2,
            y2: cart_y2,
            confidence: self.confidence,
        }
    }
}

pub fn non_maximum_suppression(
    mut boxes: Vec<DetectionResult>,
    iou_threshold: f32,
) -> Vec<DetectionResult> {
    boxes.sort_by(|a, b| {
        a.confidence
            .partial_cmp(&b.confidence)
            .unwrap_or(Ordering::Equal)
    });
    let mut keep = Vec::new();
    while !boxes.is_empty() {
        let current = boxes.remove(0);
        keep.push(current.clone());
        boxes.retain(|box_| calculate_iou(&current, box_) <= iou_threshold);
    }

    keep
}

fn calculate_iou(box1: &DetectionResult, box2: &DetectionResult) -> f32 {
    let x1 = box1.x1.max(box2.x1);
    let y1 = box1.y1.max(box2.y1);
    let x2 = box1.x2.min(box2.x2);
    let y2 = box1.y2.min(box2.y2);

    let intersection = (x2 - x1).max(0.0) * (y2 - y1).max(0.0);
    let area1 = (box1.x2 - box1.x1) * (box1.y2 - box1.y1);
    let area2 = (box2.x2 - box2.x1) * (box2.y2 - box2.y1);
    let union = area1 + area2 - intersection;
    intersection / union
}
