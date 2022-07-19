use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

struct TimedWrapper<T> {
    future: T, // you can change type of the field
    // you can add more fields
}

impl<T: Future> TimedWrapper<T> {
    pub fn new(future: T) -> Self {
        todo!()
    }
}

impl<T: Future> Future for TimedWrapper<T> {
    type Output = (T::Output, Duration);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

#[tokio::main]
async fn main() {
    let wrapper = TimedWrapper::new(reqwest::get("http://rust-lang.org"));
    let (response, time) = wrapper.await;
    println!("Got HTTP {} response in {}ms", response.unwrap().status(), time.as_millis());
}
