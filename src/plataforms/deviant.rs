use deviantart::types::Deviation;
use url::Url;
use std::vec::Vec;

use crate::scanner::Results::{Suspeitos, SuspectItem};

#[derive(Debug)]
pub struct Client {
    client: deviantart::Client,
    suspects_mgr: tokio::sync::mpsc::UnboundedSender<SuspectItem>,
}

impl Client {
    pub fn new(suspects_mgr: tokio::sync::mpsc::UnboundedSender<SuspectItem>) -> Self {
        Self {
            client: deviantart::Client::new(),
            suspects_mgr
        }
    }

    pub async fn login(&mut self, username: String, password: String) {
        self.client.login(&username, &password).await.expect("Falha no login");
    }

    pub async fn home(&mut self) {
        let r = self.client.scrape_webpage("https://www.deviantart.com/".to_string().as_ref()).await;

        match r.unwrap().entities {
            Some (entitie) => {
                //esses são os deviants dessa página. 
                //hashmap
                //let deviations = entitie.deviation;
                for(_, deviant) in entitie.deviation.into_iter() {

                    //vamos analisar item a item
                    self.get_by_id(deviant.url).await;
                }

            },
            None  => {}
        }
    }

    async fn get_by_id(&mut self, url: Url){

        let url_str = url.into_string();

        let response = self.client.scrape_webpage(&url_str).await;

        match response.unwrap().entities {
            Some(entitie) => {
                //Vamos pegar o primeiro valor no hashmap
                let (_, deviation) = entitie.deviation.into_iter().next().unwrap();
                let (_, deviation_extended) = entitie.deviation_extended.unwrap().into_iter().next().unwrap();

                let description : Description = serde_json::from_value(deviation_extended.unknown.get("descriptionText").unwrap().clone()).expect("Não foi possível parsear a descrição");
                let published_time : String = serde_json::from_value(deviation.unknown.get("publishedTime").unwrap().clone()).expect("Não foi possível parsear a data de publicação");

                // Aqui devemos fazer as verificações.
                // se encontrarmos algo vamos adcionar nos itens suspeitos.

                let new_deviation = SuspectItem {
                    titulo: deviation.title,
                    descricao: description.excerpt,
                    url: url_str,
                    published_time: published_time
                };

                let _ = self.suspects_mgr.send(new_deviation);

            },
            None => {}
        }
    }
}

#[derive(serde::Deserialize, Debug)]
struct Description {
    excerpt: String
}