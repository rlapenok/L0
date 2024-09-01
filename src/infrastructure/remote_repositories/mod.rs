
use deadpool_redis::Pool as RedisPool;
use postgres_order_repository::PostgresOrderRepository;
use redis_order_repository::RedisOrderRepository;
use sqlx::{postgres::PgRow, FromRow, Pool as PoolPostgres, Postgres};

use crate::{
    domain::{
        models::EntityForSave,
        remote_repositories::{
            postgres_order_presentation_repository::PostgresOrderPresentationRepository,
            redis_order_presentation_repository::RedisOrderPresentationRepository,
            OrderPresentationRemoteRepository,
        },
    },
    errors::remote_repository_error::RemoteRepositoryError,
};


pub(crate) mod postgres_order_repository;
pub(crate) mod redis_order_repository;

#[derive(Clone)]
pub struct RemoteRepository<P, R>
where
    P: PostgresOrderPresentationRepository,
    R: RedisOrderPresentationRepository,
{
    postgres: P,
    redis: R,
}

impl RemoteRepository<PostgresOrderRepository, RedisOrderRepository> {
    pub fn new(postgres_pool: PoolPostgres<Postgres>, redis_pool: RedisPool) -> Self {
        let postgres_repo = PostgresOrderRepository::new(postgres_pool);
        let redis_repo = RedisOrderRepository::new(redis_pool);
        Self {
            postgres: postgres_repo,
            redis: redis_repo,
        }
    }
}

/*impl ToOrderRepresentationRemoteRepositoryService<Service>
    for RemoteRepository<PostgresOrderRepository, RedisOrderRepository>
{
    fn to_service(self) -> Result<Service, Box<dyn std::error::Error>> {
        let service = RemoteService::new(self);
        Ok(service)
    }
}*/

impl OrderPresentationRemoteRepository
    for RemoteRepository<PostgresOrderRepository, RedisOrderRepository>
{
    async fn save_order<E: EntityForSave>(
        &self,
        entity: &E,
        key: &str,
        value: &str,
    ) -> Result<(), RemoteRepositoryError> {
        let postgres_result = self.postgres.save_order(entity).await;
        if let Err(postgres_err) = postgres_result {
            println!("{}",postgres_err);

            return Err(postgres_err);
        }
        let redis_result = self.redis.save_order(key, value).await;
        if let Err(redis_err) = redis_result {
            println!("{}",redis_err);
            return Err(redis_err);
        }
        Ok(())
    }
   async fn get_order<T>(&self, data_uid: String)->Result<T,RemoteRepositoryError> 
        where  T: for<'a> FromRow<'a, PgRow>+Send+Unpin
   {
       /*let redis_result=self.redis.get_order(&order_uid).await;
       if let Err(err) =redis_result  {
            println!("{}",err);
            let postgres_result=self.postgres.get_order::<T>(&order_uid).await;
            if let Err(err)=postgres_result{
                    println!("{}",err);
            }
       }*/
        self.postgres.get_order(&data_uid).await
   }

}
