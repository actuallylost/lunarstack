#![allow(unused)]
mod prisma;

use prisma::PrismaClient;
use prisma_client_rust::NewClientError;
 
#[tokio::main]
pub async fn build() -> PrismaClient {
    let client: Result<PrismaClient, NewClientError> = PrismaClient::_builder().build().await;
    client.unwrap()
}
