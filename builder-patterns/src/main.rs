// NOTE: !! REF: https://github.com/jeremychone-channel/rust-builder/blob/main/src/web_consuming.rs
// NOTE: !! Jeremy prefers the CONSUMING Builder Pattern over Non-Consuming...
// REF: https://youtu.be/Z_3WOSiYYFY?t=1107

// region:       -- Modules
#![allow(unused)]
use crate::{
    task::Task,
    web::{RequestBuilderConsuming, RequestBuilderTypeState},
};
use prelude::*;
use web::{Request, RequestBuilderNonConsuming};

mod error;
mod prelude;
mod task;
mod web;

// endregion:    -- Modules

fn main() -> Result<()> {
    // let task: Option<Task> = None;
    // let task = task.unwrap_or_default();

    // let task = Task::new("Task 01");

    // let task = Task {
    //     done: true,
    //     ..Task::new("Cool Task") // ..Default::default(),
    // };
    // println!("{task:#?}");

    // region:       -- Non-Consuming Builder Pattern

    // -- Basic RequestBuilder example:
    // let req = RequestBuilder::new()
    //     .url("https://some-url.com/task/123")
    //     .method("GET")
    //     .header("token", "user_uuid.exp.sign")
    //     .build()?;
    // println!("{req:#?}");

    // -- Reusable RequestBuilder example:
    let mut request_builder = RequestBuilderNonConsuming::new();
    request_builder
        .url("https://non-consuming.com/task/123")
        .method("GET");
    // ... do some stuff
    let req = request_builder
        .header("token", "user_uuid.exp.sign")
        .build()?;
    println!("{req:#?}");

    // Update/reuse the builder
    request_builder.header("Client-Version", "1.2");
    let req = request_builder.build()?;
    println!("{req:#?}");

    // endregion:    -- Non-Consuming Builder Pattern

    // region:       -- Consuming Builder Pattern (*preferred)

    let request_builder = RequestBuilderConsuming::new()
        .url("https://consuming.com/task/123")
        .method("GET");
    // ... do some stuff
    let request_builder = request_builder.header("token", "user_uuid.exp.sign");
    // NOTE: !! Now that we consume and return 'Self' from each method,
    // if we want to chain methods, we have to keep a handle/save our
    // 'request_builder', i.e., continually reassign: let rb = rb.method();
    // This means we'd have to split up the 'request_builder' & 'request'
    // NOTE: !! We derive(Clone) to enable us to reuse our builder.
    // This means we only clone() the FIRST time we call build(), and then
    // we can reuse the builder until the last/final call of build(), which
    // will then MOVE 'self' and consume it.
    // REF: https://www.youtube.com/watch?v=Z_3WOSiYYFY&t=12s
    let req = request_builder.clone().build()?;
    println!("{req:#?}");

    // Update/reuse the builder
    // NOTE: !! Since we've consumed the builder using 'build()?', it's no longer available.
    // Therefore, we need to derive(Clone) on the struct and use clone() the FIRST time
    // we call build() (see above). It's this final call of build() before this main function
    // ends that consumes and drops the request builder!
    let req = request_builder.header("Client-Version", "1.2").build()?;
    println!("{req:#?}");

    // endregion:    -- Consuming Builder Pattern (*preferred)

    // region:       -- TypeState Consuming Pattern (compile time checks)

    let request_builder = RequestBuilderTypeState::new()
        // NOTE:!! If we comment out the .url() that sets the url property on our builder,
        // we'll see a compile error when calling build() because build() only lives
        // inside the RequestBuilderTypeState<Url> type!
        .url("https://typestate.com/task/123")
        // REF: https://youtu.be/pwmIQzLuYl0?t=442
        .method("GET");

    let req = request_builder
        .header("Token", "user_uuid.exp.sign")
        .build()?;
    println!("{req:#?}");

    // endregion:    -- TypeState Consuming Pattern (compile time checks)

    Ok(())
}
