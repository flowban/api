use mongodb::options::ClientOptions;
use mongodb::Client;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

pub static CLIENT: Lazy<Client> = Lazy::new(|| {
    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let mut client_options = ClientOptions::parse(dotenv!("TESTING_URL")).await?;
        Client::with_options(client_options).await.unwrap()
    })
});
