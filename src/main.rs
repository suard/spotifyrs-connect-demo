use serde_env::from_env;
use spotify_rs::{ClientCredsClient, ClientCredsFlow};

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    dotenvy::dotenv()?;

    let configuration: Configuration = from_env()?;

    let auth_flow = ClientCredsFlow::new(configuration.client_id, configuration.client_secret);
    let mut spotify = ClientCredsClient::authenticate(auth_flow).await?;

    let track = spotify.track("5ubHAQtKuFfiG4FXfLP804").get().await?;

    let artists = track
        .artists
        .iter()
        .map(|artist| artist.name.clone())
        .collect::<Vec<String>>();

    println!(
        "track: {}, artists: {}, year released: {}",
        track.name,
        artists.join(", "),
        track.album.release_date
    );

    Ok(())
}

#[derive(serde::Deserialize)]
struct Configuration {
    client_id: String,
    client_secret: String,
}
