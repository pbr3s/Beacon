use tokio::time::{interval, Duration};
mod plataforms;

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    //vamos criar uma intancia de deviantart
    let mut deviant_art = plataforms::deviant::Client::new();
    let deviant_user = env::var("DEVIANT_USERNAME").expect("DEVIANT_USERNAME não definido");
    let deviant_pass = env::var("DEVIANT_PASSWORD").expect("DEVIANT_PASSWORD não definido");
    deviant_art.login(deviant_user, deviant_pass).await;

    let mut intevalo = interval(Duration::from_secs(3600));

    loop {
        intevalo.tick().await;

        deviant_art.home().await;
    }
}
