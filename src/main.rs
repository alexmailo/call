use structopt::StructOpt;
use tokio::runtime::Runtime;
use reqwest::{Client, RequestBuilder};

/// Make http requests
#[derive(StructOpt, Debug)]
struct Args {
    /// Method to use
    #[structopt(subcommand)]
    method: Method,

}

#[derive(StructOpt, Debug)]
enum Method{
    /// make a post request
    Post { 
        url: String,
        #[structopt(short,long)]
        data: Option<String>
    },
    /// make a put request
    Put { 
        url: String,
        #[structopt(short,long)]
        data: Option<String>
    },
    /// make a get request
    Get { 
        url: String
    },
    /// make a delete request
    Delete { 
        url: String
    },
    /// make a head request
    Head { 
        url: String
    },
}

async fn make_request( args: &Args){
    let c = Client::new();
    let mut rb : RequestBuilder;
    
    match &args.method{
        Method::Post { url, data } => {
            rb = c.post(url);
            if data.is_some(){
                rb = rb.body(data.clone().unwrap());
            }

            let response = rb.send().await;

            if response.is_err(){
                println!("Could not make post request to endpoint {}. Problem {}", url, response.unwrap_err().to_string());
                return;
            }

            // post succeded 
            let text_reponse = response.unwrap().text().await;

            if text_reponse.is_err(){
                println!("Could not get text of {} due to {}", url, text_reponse.unwrap_err().to_string());
                return;
            }
            println!("{}", text_reponse.unwrap());

        },
        Method::Put { url, data } => todo!(),
        Method::Get { url } => {
            rb = c.get(url);
            let response = rb.send().await;

            if response.is_err(){
                println!("Could not make get request to endpoint {}. Problem {}", url, response.unwrap_err().to_string());
                return;
            }

            // post succeded 
            let text_reponse = response.unwrap().text().await;

            if text_reponse.is_err(){
                println!("Could not get text of {} due to {}", url, text_reponse.unwrap_err().to_string());
                return;
            }

            println!("{}", text_reponse.unwrap());

        }
        Method::Delete { url } => todo!(),
        Method::Head { url } => {
            rb = c.head(url);
            let response = rb.send().await;

            if response.is_err(){
                println!("Could not make head request to endpoint {}. Problem {}", url, response.unwrap_err().to_string());
                return;
            }

            // post succeded 
            let text_reponse = response.unwrap().text().await;

            if text_reponse.is_err(){
                println!("Could not get text of {} due to {}", url, text_reponse.unwrap_err().to_string());
                return;
            }

            println!("{}", text_reponse.unwrap());
        }
    }
}

fn main() {
    let args = Args::from_args();
    dbg!(&args);
    let rt = Runtime::new().unwrap();
    rt.block_on(make_request(&args));
}
