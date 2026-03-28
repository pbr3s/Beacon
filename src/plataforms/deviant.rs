pub struct Client {
    client: deviantart::Client
}

impl Client {
    pub async fn new(&mut self, username: String, password: String){
        self.client = deviantart::Client::new();
        self.client.login(&username, &password).await.expect("Falha no login");
    }
}