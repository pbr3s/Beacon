use tokio::time::{interval, Duration};

mod plataforms;

#[tokio::main]
async fn main() {
    
    //vamos criar uma intancia de deviantart
    let mut deviant_art = plataforms::deviant::Client::new();
    //deviant_art.login("teste".to_string(), "teste123".to_string()).await;

    let mut intevalo = interval(Duration::from_secs(60));

    loop {
        intevalo.tick().await;

        deviant_art.home().await;
    }
}
