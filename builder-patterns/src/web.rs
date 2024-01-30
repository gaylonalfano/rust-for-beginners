// NOTE: !! For Builder pattern, it's better to take a ref of self '&self'
// instead of mutable, so we can reuse the same request_builder variable
// again and again inside main.rs because this build() is cloning the data
// instead of moving ownership.
// REF: https://youtu.be/Z_3WOSiYYFY?t=885
// NOTE: !! That said, Jeremy prefers the CONSUMING pattern instead, so we
// take 'self' completely and therefore can move the data ownership, since
// nobody will have the RequestBuilderConsuming after we call build() since
// we DROP the RequestBuilderConsuming and only return Result<Request>.
// REF: https://youtu.be/Z_3WOSiYYFY?t=925
use crate::prelude::*;

#[derive(Debug)]
pub struct Request {
    pub url: String,
    pub method: String,                 // should be enum
    pub headers: Vec<(String, String)>, // (name, value)
    pub body: Option<String>,
}

// region:       -- TypeState Pattern (builds on top of Consuming)

// -- State Types for TypeState Pattern
// WARN: Q: How to add additional states?
// A: We first add 'M' generic and then matching NoMethod and Method
// types to the corresponding impl blocks.
// REF: https://youtu.be/pwmIQzLuYl0?t=494

// NOTE: !! We're using types as state, so we MUST use structs
// and NOT enums, because enums are just one type even with variants.
// NOTE: Both need to fit into the request builder, so they both
// need to implement Default and Clone traits.
#[derive(Default, Clone)]
pub struct NoUrl;
#[derive(Default, Clone)]
pub struct Url(String);

#[derive(Default, Clone)]
pub struct NoMethod;
#[derive(Default, Clone)]
pub struct Method(String);
// -- State Types for TypeState Pattern

// NOTE: We first use a generic 'U' and then we can use separate
// impl blocks with specific types to represent 'U' (e.g., Url, NoUrl)
#[derive(Default, Clone)]
pub struct RequestBuilderTypeState<U, M> {
    url: U,
    method: M,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

// NOTE: When we add a generic 'U' to the scope, this breaks
// Default because it doesn't know what type 'U' is. Therefore,
// we move our new() fn to a separate impl block and specify 'U'
// as type NoUrl to help default().
impl RequestBuilderTypeState<NoUrl, NoMethod> {
    // NOTE: Again, NoUrl implements Default so this works
    pub fn new() -> Self {
        Self::default()
    }
}

// NOTE: We do the same thing for build() but specify 'U' as Url.
impl RequestBuilderTypeState<Url, Method> {
    pub fn build(self) -> Result<Request> {
        // // NOTE: We no longer need the runtime check since this TypeState
        // // pattern is creating COMPILE TIME checks!
        // // NOTE: U: Can remove this after we add new types for Method & NoMethod
        // let method = self.method.unwrap_or_else(|| "GET".to_string());

        Ok(Request {
            // NOTE: Request.url is String, but our 'self' is RBTS<Url>,
            // so 'self.url' ->> Url(String), so we extract using 'self.url.0'
            // NOTE: U: After adding new types Method/NoMethod, we can simply
            // get 'method' value from self.method.0
            url: self.url.0,
            method: self.method.0,
            headers: self.headers,
            body: self.body,
        })
    }
}

impl<U, M> RequestBuilderTypeState<U, M> {
    // pub fn new() -> Self { self::default() } // Moved to separate impl block

    // NOTE: No longer need 'mut'
    // NOTE: We pass generic 'M' so it accepts both types NoMethod or Method.
    pub fn url(self, url: impl Into<String>) -> RequestBuilderTypeState<Url, M> {
        RequestBuilderTypeState {
            url: Url(url.into()),
            method: self.method,
            headers: self.headers,
            body: self.body,
        }
    }

    pub fn method(mut self, method: impl Into<String>) -> RequestBuilderTypeState<U, Method> {
        // self.method = Some(method.into());
        // self.method.insert(method.into());
        // self

        RequestBuilderTypeState {
            url: self.url,
            method: Method(method.into()),
            headers: self.headers,
            body: self.body,
        }
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        // self.body = Some(body.into());
        self.body.insert(body.into());
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    // pub fn build(self) -> {...} // Moved to separate impl block
}

// endregion:    -- TypeState Pattern (builds on top of Consuming)

// region:       -- Non-Consuming

#[derive(Default)]
pub struct RequestBuilderNonConsuming {
    // Q: How to macro wrap the value with 'Option<Value>' with vim?
    // A: Single word    ->> ciwOption< C-r">
    // A: Multiple words ->> cOption< C-r C-p">
    pub url: Option<String>,
    pub method: Option<String>,
    pub headers: Vec<(String, String)>,
    pub body: Option<String>,
}

impl RequestBuilderNonConsuming {
    pub fn new() -> Self {
        Self::default()
    }

    // NOTE: You create a method/fn for each struct property.
    // NOTE: !! '&mut self' is a non-consuming pattern. Meaning, it takes
    // a mutable reference of self each time to change it in-place.
    // You want to return -> &mut Self so you can chain these methods!
    pub fn url(&mut self, url: impl Into<String>) -> &mut Self {
        self.url = Some(url.into());
        self
    }

    pub fn method(&mut self, method: impl Into<String>) -> &mut Self {
        self.method = Some(method.into());
        self
    }

    pub fn body(&mut self, body: impl Into<String>) -> &mut Self {
        self.body = Some(body.into());
        self
    }

    pub fn header(&mut self, name: impl Into<String>, value: impl Into<String>) -> &mut Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    // NOTE: !! For Builder pattern, it's better to take a ref of self '&self'
    // instead of mutable, so we can reuse the same request_builder variable
    // again and again inside main.rs because this build() is cloning the data
    // instead of moving ownership.
    // REF: https://youtu.be/Z_3WOSiYYFY?t=885
    pub fn build(&self) -> Result<Request> {
        // -- Add a guard (runtime check) to ensure we have a 'url'
        let Some(url) = self.url.as_ref() else {
            return Err("No URL".to_string().into());
            // return Err(format!("ERROR WHILE RUN: {:?}", other).into());
        };
        let method = self.method.clone().unwrap_or_else(|| "GET".to_string());

        // NOTE: We have to clone the data because we only have a ref of self.
        Ok(Request {
            url: url.to_string(),
            method: method.to_string(),
            headers: self.headers.clone(),
            body: self.body.clone(),
        })
    }
}

// endregion:    -- Non-Consuming

// region:       -- Consuming (preferred)

// NOTE: !! Jeremy prefers the CONSUMING Builder Pattern (self vs. &self) over Non-Consuming...
// REF: https://youtu.be/Z_3WOSiYYFY?t=1107
// NOTE: !! We derive(Clone) to enable us to reuse our builder by simply doing:
// let req = request_builder.clone().build()?;
// This means we only clone() the FIRST time we call build(), and then
// we can reuse the builder until the last/final call of build(), which
// will then MOVE 'self' and consume it.
// REF: https://www.youtube.com/watch?v=Z_3WOSiYYFY&t=12s
#[derive(Default, Clone)]
pub struct RequestBuilderConsuming {
    // Q: How to macro wrap the value with 'Option<Value>' with vim?
    // A: Single word    ->> ciwOption< C-r">
    // A: Multiple words ->> cOption< C-r C-p">
    // NOTE: Generally you want to make your properties private and just expose the methods.
    url: Option<String>,
    method: Option<String>,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

impl RequestBuilderConsuming {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn url(mut self, url: impl Into<String>) -> Self {
        // NOTE: New assignment with a new Option, but
        // we can do better by simply reusing original Option
        // along with the Option.insert() method.
        // self.url = Some(url.into());
        self.url.insert(url.into());
        self
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        // self.method = Some(method.into());
        self.method.insert(method.into());
        self
    }

    pub fn body(mut self, body: impl Into<String>) -> Self {
        // self.body = Some(body.into());
        self.body.insert(body.into());
        self
    }

    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.push((name.into(), value.into()));
        self
    }

    // NOTE: !! For Builder pattern, it's better to take a ref of self '&self'
    // instead of mutable, so we can reuse the same request_builder variable
    // again and again inside main.rs because this build() is cloning the data
    // instead of moving ownership.
    // REF: https://youtu.be/Z_3WOSiYYFY?t=885
    // NOTE: !! That said, Jeremy prefers the CONSUMING pattern instead, so we
    // take 'self' completely and therefore can move the data ownership, since
    // nobody will have the RequestBuilderConsuming after we call build() since
    // we DROP the RequestBuilderConsuming and only return Result<Request>.
    // REF: https://youtu.be/Z_3WOSiYYFY?t=925
    pub fn build(self) -> Result<Request> {
        // -- Add a guard (runtime check) to ensure we have a 'url'
        // NOTE: Runtime check if the url is present.
        //       See the state builder pattern to do it a compile time.
        let Some(url) = self.url else {
            return Err("No URL".to_string().into());
            // return Err(format!("ERROR WHILE RUN: {:?}", other).into());
        };
        // With consuming 'self' we don't need clone() anymore. We just move data!
        // NOTE: In this example, foro method, we will get a default "GET" if none.
        //       In the state builder we will make this required as well to show multi-states.
        let method = self.method.unwrap_or_else(|| "GET".to_string());

        // NOTE: We have to clone the data because we only have a ref of self.
        Ok(Request {
            url,
            method,
            headers: self.headers.clone(),
            body: self.body.clone(),
        })
    }
}

// endregion:    -- Consuming (preferred)
