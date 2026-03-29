use tokio::time::{interval, Duration};
mod plataforms;
mod scanner;

use scanner::Results::{Suspeitos, SuspectItem};

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let (task_tx, mut task_rx) = tokio::sync::mpsc::unbounded_channel::<(SuspectItem)>();

    //vamos criar a lista de suspeitos
    let mut suspeitos = Suspeitos::new();

    //vamos criar uma intancia de deviantart
    let mut deviant_art = plataforms::deviant::Client::new(task_tx.clone());

    //vamos criar uma instancia de pintrest
    let mut pinterest = plataforms::pinterest::Client::new();

    let deviant_user = env::var("DEVIANT_USERNAME").expect("DEVIANT_USERNAME não definido");
    let deviant_pass = env::var("DEVIANT_PASSWORD").expect("DEVIANT_PASSWORD não definido");
    deviant_art.login(deviant_user, deviant_pass).await;

    let mut intevalo = interval(Duration::from_secs(1560));

    let rx_task = tokio::spawn(async move {
        while let Some(SuspectItem) = task_rx.recv().await {
            suspeitos.add_suspect(SuspectItem);
        }
    });

    loop {
        intevalo.tick().await;

        pinterest.home();
        deviant_art.home().await;
    }
}
