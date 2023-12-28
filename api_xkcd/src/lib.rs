use spin_sdk::http::{IntoResponse, Request, Method,Response};
use spin_sdk::http_component;
use serde::Deserialize;
use maud::html;

#[derive(Deserialize, Debug)]
struct ComicData {
    img: String,
    safe_title: String,
    alt: String,
}

#[http_component]
async fn handle_api_xkcd(_req: Request) -> anyhow::Result<impl IntoResponse> {
    // Create the outbound request object
    let req = Request::builder()
        .method(Method::Get)
        .uri("https://xkcd.com/info.0.json")
        .build();

    // Send the request and await the response
    let res: Response = spin_sdk::http::send(req).await?;
    let comic_data: ComicData = serde_json::from_slice(res.body()).unwrap();
    let body = html! {
        img src=(comic_data.img) title=(comic_data.alt) alt=(comic_data.safe_title);
    }.into_string();
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(body)
        .build())
}