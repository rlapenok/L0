use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::domain::models::DeliveriEntity;

#[derive(Serialize, Deserialize, Validate,Debug)]
#[serde(deny_unknown_fields)]
pub struct Delivery {
    #[validate(length(min = 1, message = "Can not be empty"))]
    name: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    phone: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    zip: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    city: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    address: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    region: String,
    #[validate(length(min = 1, message = "Can not be empty"))]
    email: String,
}
impl Delivery {
    pub fn new(
        name: String,
        phone: String,
        zip: String,
        city: String,
        address: String,
        region: String,
        email: String,
    ) -> Self {
        Delivery {
            name,
            phone,
            zip,
            city,
            address,
            region,
            email,
        }
    }
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
