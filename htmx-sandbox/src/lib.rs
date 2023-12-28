use anyhow::Result;
use maud::{html, PreEscaped};
use spin_sdk::http::{Params, Request, Response, Router};
use spin_sdk::http_component;

fn root_uri() -> String {
    let local_dev = false;

    match local_dev {
        true => String::from(""),
        false => String::from("https://cb-cloud-ksksxilp.fermyon.app")
    }
}

macro_rules! register_routes {
    ($router:expr, $preflight_handler:expr, $( $path:expr => $handler:expr ),*) => {
        $(
            $router.get($path, $handler);
            $router.options($path, $preflight_handler);
        )*
    }
}

#[http_component]
fn handle_htmx_sandbox(req: Request) -> Response {
    let mut router = Router::new();
    register_routes!(
        router,
        preflight_cors_allow,
        "/rest/" => root_button,
        "/rest/0" => step_zero,
        "/rest/1" => step_one,
        "/rest/2" => step_two
    );
    router.handle(req)
}

fn preflight_cors_allow(_req: Request, _params: Params) -> Result<Response> {
    Ok(Response::builder()
        .status(200)
        .header("Access-Control-Allow-Origin", "*")
        .header("Access-Control-Allow-Methods", "GET, OPTIONS")
        .header("Access-Control-Allow-Headers", "*")
        .build())
}

fn xs_res(body: PreEscaped<String>) -> Response {
    // Cross site response headers
    Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .header("Access-Control-Allow-Origin", "*")
        .body(body.into_string())
        .build()
}

fn root_button(_req: Request, _params: Params) -> Result<Response> {
    let htmx_uri = "https://unpkg.com/htmx.org@1.9.10";
    let sha = "sha384-D1Kt99CQMDuVetoL1lrYwg5t+9QdHe7NLX/SoJYkXDFfX37iInKRy5xLSi8nO7UC";
    Ok(xs_res(html! {
        head {
            title {"My HTMX Sandbox"}
            script src=(htmx_uri) integrity=(sha) crossorigin="anonymous" {}
        }
        body {
            button hx-get="/rest/1" hx-swap="outerHTML" {
                "I have a favor I need to ask you..."
            }
        }
    }))
}

fn step_zero(_req: Request, _params: Params) -> Result<Response> {
    let next_step = root_uri() + "/rest/1";
    Ok(xs_res(html! {
        body {
            button hx-get=(next_step) hx-swap="outerHTML" {
                "I have a favor I need to ask you..."
            }
        }
    }))
}
fn step_one(_req: Request, _params: Params) -> Result<Response> {
    let next_step = root_uri() + "/rest/2";
    Ok(xs_res(html! {
        p{
            "You Found a Secret!"
        }
        button hx-get=(next_step) hx-swap="outerHTML" {
            "Claim Your Reward!"
        }
    }))
}

fn step_two(_req: Request, _params: Params) -> Result<Response> {
    Ok(xs_res(html! {
        strong {
            p {
                "Wow! It's a..."
                br;
            }
            a href="https://en.wikipedia.org/wiki/Pikachu" {
                img
                    src="https://upload.wikimedia.org/wikipedia/en/thumb/7/73/Pikachu_artwork_for_Pok%C3%A9mon_Red_and_Blue.webp/256px-Pikachu_artwork_for_Pok%C3%A9mon_Red_and_Blue.webp.png"
                    alt="It's a Pikachu!";
            }
        }
    }))
}
