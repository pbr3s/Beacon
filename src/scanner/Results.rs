use std::sync::{Arc, RwLock};
use std::collections::{HashMap};

#[derive(Debug)]
pub struct Suspeitos {
    pub inner: Arc<RwLock<SuspeitosStore>>
}

impl Suspeitos {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(RwLock::new(SuspeitosStore { items: HashMap::new() }))
        }
    }

    pub fn add_suspect(&mut self, suspeito: SuspectItem){
        let mut suspects = self.inner.write().unwrap();

        println!("Tamanho atual da lista de suspeitos: {}", suspects.items.len());
        println!("{:#?}", &suspeito);

        if suspects.items.contains_key(&suspeito.url) {
            return
        }

        suspects.items.insert(suspeito.url.clone(), suspeito);
    }
}

#[derive(Debug)]
pub struct SuspeitosStore {
    pub items: HashMap<String, SuspectItem>
}

#[derive(Debug)]
pub struct SuspectItem {
    pub titulo: String,
    pub descricao: String,
    pub url: String,
    pub published_time: String,
}
