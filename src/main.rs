use crate::settings::settings::Settings;

mod handlers;
mod settings;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::with_level(tide::log::LevelFilter::Info);

    let settings = Settings::new("development").unwrap();
    let mut app = tide::new();

    app.at("/").get(handlers::home::show);
    app.at("/metadata").get(handlers::aws::metadata);

    app.listen(format!("{}", settings.server)).await?;
    Ok(())
}
