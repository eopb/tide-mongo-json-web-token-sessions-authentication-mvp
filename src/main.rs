mod routes;
mod state;

use state::State;

use dotenv::dotenv;
use std::env;

#[async_std::main]
async fn main() -> tide::Result<()> {
    dotenv().ok();
    femme::start(log::LevelFilter::Debug)?;

    let state = State::new().await?;
    let mut app = tide::with_state(state);

    app.at("/authenticate").post(routes::authenticate);
    app.at("/create-user").post(routes::create_user);
    app.at("/users/:user").post(routes::user_page);
    app.listen("localhost:8080").await?;

    Ok(())
}
