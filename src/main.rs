extern crate gotham;
#[macro_use] extern crate gotham_derive;
extern crate hyper;
extern crate mime;
extern crate serde;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate serde_json;
#[macro_use] extern crate slog;
extern crate sloggers;

use hyper::{Body, Response, StatusCode};
use gotham::http::response::create_response;
use gotham::middleware::pipeline::new_pipeline;
use gotham::router::Router;
use gotham::router::route::dispatch::{new_pipeline_set, finalize_pipeline_set};
use gotham::state::{State, FromState};

use sloggers::Build;
use sloggers::terminal::{TerminalLoggerBuilder, Destination};
use sloggers::types::Severity;

fn main() {
    use gotham::router::builder::*;

    let mut builder = TerminalLoggerBuilder::new();
    builder.level(Severity::Debug);
    builder.destination(Destination::Stderr);

    let logger = builder.build().unwrap();

    info!(logger, "API server started!");

    gotham::start("0.0.0.0:7878", router());
}

fn json_response<T : serde::Serialize>(state: &State, status_code: StatusCode, data: &T) -> Response {
    let json_data = json!(data);
    let json_bytes = serde_json::to_vec(&json_data).unwrap();

    create_response(
        state,
        status_code,
        Some((json_bytes, mime::APPLICATION_JSON)),
    )
}

#[derive(Serialize)]
struct DummyPost {
    no: i64,
    comment: Option<String>,
}

#[derive(StateData, PathExtractor, StaticResponseExtender)]
struct GetThreadRequestPath {
    no: i64,
}

fn get_thread(state: State) -> (State, Response) {
    //let req = GetThreadRequestPath::borrow_from(&state);

    let posts: Vec<DummyPost> = Vec::new(); //store.get_thread(req.no);

    let res = json_response(
        &state,
        StatusCode::Ok,
        &posts,
    );
    (state, res)
}

fn router() -> Router {
    use gotham::router::builder::*;

    let pipelines = new_pipeline_set();
    let (pipelines, global) = pipelines.add(
        new_pipeline().build(),
    );

    let default_pipeline_chain = (global, ());

    let pipelines = finalize_pipeline_set(pipelines);

    build_router(default_pipeline_chain, pipelines, |route| {
        route.get("/threads/:no").to(get_thread);
    })
}
