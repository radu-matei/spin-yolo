use bindings::exports::component::face_detection_lib::face_detection::{
    DetectionError, DetectionResult, Guest, Image,
};

#[allow(warnings)]
mod bindings;

struct Component;

impl Guest for Component {
    fn detect(
        img: Image,
        confidence_treshold: f32,
    ) -> Result<Vec<DetectionResult>, DetectionError> {

        let detections = detection::detect(img, confidence_treshold)?;
        let detections = detections.into_iter().map(|d| d.into()).collect();
        Ok(detections)
    }
}

bindings::export!(Component with_types_in bindings);

impl From<detection::DetectionResult> for DetectionResult {
    fn from(value: detection::DetectionResult) -> Self {
        DetectionResult {
            x1: value.x1,
            y1: value.y1,
            x2: value.x2,
            y2: value.y2,
            confidence: value.confidence,
        }
    }
}

impl From<anyhow::Error> for DetectionError {
    fn from(e: anyhow::Error) -> Self {
        DetectionError::ModelError(e.to_string())
    }
}
