use std::error::Error;

use crate::domain::{
    local_repositories::in_file_order_presentation_repository::InFileOrderPresentationRepository,
    remote_repositories::Destination,
    services::local_order_presentation_remote_services::LocalOrderRepresentationService,
};

/*pub struct LocalService<T>
where
    T: InFileOrderPresentationRepository,
{
    repo: T,
}

impl<T> LocalService<T>
where
    T: InFileOrderPresentationRepository,
{
    pub fn new(repo: T) -> Self {
        Self { repo: repo }
    }
}

impl<T> LocalOrderRepresentationService for LocalService<T>
where
    T: InFileOrderPresentationRepository,
{
    unimplemented!()
}*/
