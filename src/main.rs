#![feature(proc_macro, conservative_impl_trait, generators)]

extern crate futures_await as futures;
extern crate gotham;
#[macro_use] extern crate gotham_derive;
extern crate hyper;
#[macro_use] extern crate log;
extern crate serde;
#[macro_use] extern crate serde_json;

use futures::prelude::*;
use futures::stream::*;
use hyper::{Body, Response, StatusCode};
use gotham::handler::{HandlerError, HandlerFuture, IntoHandlerError};
use gotham::state::{State, FromState};

fn main() {
}

#[async(boxed)]
fn json_body<T : serde::de::DeserializeOwned>(state: &mut State) -> Result<T, hyper::Error> {
    use std::io::*;

    let res = {
        let body = Body::take_from(state);
        let valid_body = await!(body.concat2())?;

        serde_json::from_reader(valid_body.to_vec().as_slice()).unwrap()
    };

    Ok(res)
}
