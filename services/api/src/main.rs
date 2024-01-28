#![deny(warnings)]
use warp::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = warp::any().map(|| "Hello, World!");
    let client = prisma_client::build();

    warp::serve(routes)
    .run(([127, 0, 0, 1], 3030))
    .await;
}
