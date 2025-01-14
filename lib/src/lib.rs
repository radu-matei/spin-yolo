use bindings::exports::component::face_detection_lib::face_detection::{
    DetectionError, DetectionResult, Guest, Image,
};

#[allow(warnings)]
mod bindings;

mod detection;

struct Component;

impl Guest for Component {
    fn detect(
        img: Image,
        confidence_treshold: f32,
    ) -> Result<Vec<DetectionResult>, DetectionError> {
        Ok(detection::detect(img, confidence_treshold)?)
    }
}

bindings::export!(Component with_types_in bindings);

impl From<anyhow::Error> for DetectionError {
    fn from(e: anyhow::Error) -> Self {
        DetectionError::ModelError(e.to_string())
    }
}
