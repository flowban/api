use mongodb::options::ClientOptions;
use mongodb::Client;
use once_cell::sync::Lazy;
use tokio::runtime::Runtime;

pub static CLIENT_RUNTIME: Lazy<(Client, Runtime)> = Lazy::new(|| {
    let rt = Runtime::new().unwrap();
    let client = rt.block_on(async {
        let client_options = ClientOptions::parse(dotenv!("TESTING_URL")).await.unwrap();
        Client::with_options(client_options).unwrap()
    });
    (client, rt)
});
