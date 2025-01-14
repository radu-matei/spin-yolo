use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

mod bindings;

/// A simple Spin HTTP component.
#[http_component]
fn handle_face_detection(req: Request) -> anyhow::Result<impl IntoResponse> {
    let res = bindings::deps::component::face_detection_lib::face_detection::detect(
        &req.body().to_vec(),
        0.1,
    )
    .unwrap();

    println!("{:?}", res);
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello, Fermyon")
        .build())
}
