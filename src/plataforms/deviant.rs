use deviantart::types::Deviation;
use url::Url;

#[derive(Debug)]
pub struct Client {
    client: deviantart::Client
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: deviantart::Client::new()
        }
    }

    pub async fn login(&mut self, username: String, password: String) {
        self.client.login(&username, &password).await.expect("Falha no login");
    }

    pub async fn home(&self) {
        let r = self.client.scrape_webpage("https://www.deviantart.com/".to_string().as_ref()).await;
        
        match r.unwrap().entities {
            Some (entitie) => {
                //esses são os deviants dessa página. 
                //baseado em assinaturas, já podemos começar a analisar conteúdo suspeito aqui.
                //entretanto, vamos também analisar item por item. 

                //hashmap
                //let deviations = entitie.deviation;
                for(_, deviant) in entitie.deviation.into_iter() {
                    self.get_by_id(deviant.url).await;
                }

            },
            None  => {}
        }
    }

    async fn get_by_id(&self, url: Url) -> Option<Deviation>{

        let deviant = self.client.scrape_webpage(url.into_string().as_ref()).await;

        println!("{:#?}", deviant);

        todo!("Retorna Deviation")
    }
}