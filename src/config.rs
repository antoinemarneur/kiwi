// The configuration parameters for the application.
// These can either be passed through the command line or pulled from the environment.
#[derive(clap::Parser)]
pub struct Config {
    // The connection URL for the database this application should use.
    #[clap(long, env)]
    pub database_url: String,
}