use serde::Deserialize;

#[derive(Deserialize)]
pub struct QueryParams {
    order_uid: String,
}
impl QueryParams {
    pub fn get_order_uid(&self) -> &str {
        &self.order_uid
    }
}
