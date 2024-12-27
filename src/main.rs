use feedwave::configuration::get_configuration;
use feedwave::starter::Application;
use feedwave::telemetry::{get_subscriber, init_subscriber};

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber = get_subscriber("FeedWave".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);

    let configuration = get_configuration().expect("Failed to read configuration");
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await?;

    Ok(())
}
