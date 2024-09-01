use serde::{Deserialize, Serialize};
use sqlx::{
    prelude::{FromRow, Type},
    Decode,
};

use crate::domain::models::DeliveriEntity;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Delivery {
    name: String,
    //todo mb create type for phones number
    phone: String,
    zip: String,
    city: String,
    address: String,
    region: String,
    //todo mb create type for emails
    email: String,
}

impl DeliveriEntity for Delivery {
    fn get_name(&self) -> &str {
        &self.name
    }
    fn get_phone(&self) -> &str {
        &self.phone
    }
    fn get_zip(&self) -> &str {
        &self.zip
    }
    fn get_city(&self) -> &str {
        &self.city
    }
    fn get_address(&self) -> &str {
        &self.address
    }
    fn get_region(&self) -> &str {
        &self.region
    }
    fn get_email(&self) -> &str {
        &self.email
    }
}
