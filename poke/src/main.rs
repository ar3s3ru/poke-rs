use warp::Filter;

#[tokio::main]
async fn main() {
    env_logger::init();

    let logger = warp::log("poke");

    let hello = warp::path("hello")
        .and(warp::path::param())
        .and(warp::header("User-Agent"))
        .map(|param: String, agent: String| format!("Hello {}, from {}", param, agent));

    let routes = hello.or(poke_http::api()).with(logger);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
