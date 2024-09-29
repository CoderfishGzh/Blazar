use blazar::proxy;
use failure::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    proxy::run().await
}
