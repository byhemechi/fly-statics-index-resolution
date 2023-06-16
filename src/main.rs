mod env;
use env::Env;
use warp::Filter;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env: Env = envy::from_env()?;

    let index_route = warp::path::end().map(|| "This is coming from the app");

    warp::serve(index_route).run(env.host).await;

    Ok(())
}
