use tide::{Request, Response, http::mime};

pub async fn show(_req: Request<()>) -> tide::Result {
    let response = Response::builder(200)
        .body("<h1> Hello, World!</h1>")
        .content_type(mime::HTML)
        .build();

    Ok(response)
}