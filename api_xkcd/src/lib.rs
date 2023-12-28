use anyhow::Result;
use spin_sdk::http::{Request, Method, Response};
use spin_sdk::http_component;
use serde::Deserialize;
use maud::html;


#[http_component]
async fn route_xkcd_requests(req: Request) -> Result<Response> {
    match req.method() {
        Method::Get => handle_api_xkcd(req).await,
        Method::Options => preflight_cors_allow(req),
        _ => panic!("Unexpected Request")
    }
}

fn preflight_cors_allow(_req: Request) -> Result<Response> {
    Ok(Response::builder()
        .status(200)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, OPTIONS")
        .header("Access-Control-Allow-Headers", "*")
        .build())
}

#[derive(Deserialize, Debug)]
struct ComicData {
    img: String,
    safe_title: String,
    alt: String,
}
async fn handle_api_xkcd(_req: Request) -> Result<Response> {
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
        .header("Access-Control-Allow-Origin", "*")
        .body(body)
        .build())
}
