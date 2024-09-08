use std::{fs, str::FromStr};

use clap::{Parser,Subcommand};
use reqwest::{Client,Url};
use server::models::model::Order;




#[derive(Subcommand)]
enum HttpMethod{
    Post{
        path:String
    },
    Get{
        order_uid:usize
    },
}


#[derive(Parser)]
#[command(name = "script")]
struct Cli{
    #[command(subcommand)]
    method:HttpMethod,
}


#[tokio::main]
async fn main(){


    let cli=Cli::parse();
    let client=Client::new();

    match cli.method {
        HttpMethod::Get { order_uid }=>{
            let url=format!("http://localhost:8080/get_order?order_uid={}",order_uid);
            let url=Url::from_str(&url).expect("Error while parse url");
            let result=client.get(url).send().await;
            match result {
                Ok(resp)=>{
                    println!("Resposne:{}",resp.text().await.unwrap())
                }
                Err(err)=>{
                    eprintln!("Error while send request:{}",err);
                }
                
            }
        }
        HttpMethod::Post { path }=>{
            let url=Url::from_str("http://localhost:8080/save_order").expect("Error while parse url");
            let order=fs::read_to_string(path).expect("Error while read file");
            let order=serde_json::from_str::<Order>(&order).expect("Error while serialize order from file");
            let result=client.post(url).json(&order).send().await;
            match result {
                Ok(resp)=>{
                    println!("Response:{}",resp.text().await.unwrap())
                }
                Err(err)=>{
                        eprintln!("Error while send request:{}",err)
                }
            }
        }
        
    }



}