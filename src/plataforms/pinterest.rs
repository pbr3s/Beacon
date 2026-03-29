use ripht_php_sapi::prelude::*;
use std::path::PathBuf;

pub struct Client {
    sapi: RiphtSapi,
    script: PathBuf
}

impl Client {
    pub fn new() -> Self {
        Self {
            sapi: RiphtSapi::instance(),
            script: PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                        .join("pinterest")
                        .join("bot.php")
        }
    }

    pub fn home(&self) {
        let req = CliRequest::new()
            .build(&self.script)
            .expect("Falhou ao criar script");

        let res = self.sapi.execute(req).expect("Execução do script falhou");
        println!("{}", res.body_string());
    }
}