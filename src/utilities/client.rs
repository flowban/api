use mongodb::Client;

pub struct AppState {
    pub(crate) client: Client
}