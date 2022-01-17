use structopt::StructOpt;

/// Make http requests
#[derive(StructOpt, Debug)]
pub struct Args {
    /// Method to use
    #[structopt(subcommand)]
    pub method: Method,

}

#[derive(StructOpt, Debug)]
pub enum Method{
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

