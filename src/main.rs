mod handlers;

#[async_std::main]
async fn main() -> Result<(), std::io::Error> {
    tide::log::with_level(tide::log::LevelFilter::Info);

    let mut app = tide::new();

    app.at("/").get(handlers::home::show);
    app.at("/metadata").get(handlers::aws::metadata);

    app.listen("0.0.0.0:3000").await?;
    Ok(())
}
