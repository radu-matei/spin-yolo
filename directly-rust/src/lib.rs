use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

mod detection;

/// A simple Spin HTTP component.
#[http_component]
fn handle_directly_rust(req: Request) -> anyhow::Result<impl IntoResponse> {
    let res = detection::detect(req.body().to_vec(), 0.1).unwrap();
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")
        .build())
}
