use std::sync::Arc;

use tokio_postgres::Client;

#[derive(Clone)]
pub struct PostgresRepo {
    client: Arc<Client>,
}

impl PostgresRepo {
    pub fn new(client: Client) -> Self {
        Self {
            client: Arc::new(client),
        }
    }
}
