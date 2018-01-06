extern crate gotham;
#[macro_use] extern crate gotham_derive;
extern crate hyper;
#[macro_use] extern crate slog;

fn main() {
}

#[derive(StateData, PathExtractor, StaticResponseExtender)]
struct GetThreadRequestPath {
    no: i64,
}
