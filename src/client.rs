mod global;
use global::{get_server, proto};

use clap::Parser;
use proto::streamingcalc_client::StreamingcalcClient;
use std::error::Error;

#[derive(Debug, Parser)]
struct Options {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
enum Command {
    Bwserver(BwserverOptions),
    Serverusagebw(ServerusagebwOptions),
}

#[derive(Debug, Parser)]
struct BwserverOptions {
    nblisteners: f32,
    bitrate: f32,
}

#[derive(Debug, Parser)]
struct ServerusagebwOptions {
    nblisteners: f32,
    bitrate: f32,
    nhdays: f32,
    nbhours: f32,
}
async fn bwserver(opts: BwserverOptions) -> Result<(), Box<dyn Error>> {
    let req = proto::BwserverRequest {
        nblisteners: opts.nblisteners,
        bitrate: opts.bitrate,
    };
    let request = tonic::Request::new(req);
    let resp = StreamingcalcClient::connect(get_server(true))
        .await?
        .bwserver(request)
        .await?;
    println!("Response: {:?}", resp.get_ref().result);
    Ok(())
}

async fn serverusagebw(opts: ServerusagebwOptions) -> Result<(), Box<dyn Error>> {
    let req = proto::ServerusagebwRequest {
        nblisteners: opts.nblisteners,
        bitrate: opts.bitrate,
        nbdays: opts.nhdays,
        nbhours: opts.nbhours,
    };
    let request = tonic::Request::new(req);
    let resp = StreamingcalcClient::connect(get_server(true))
        .await?
        .serverusagebw(request)
        .await?;
    println!("Response: {:?}", resp.get_ref().result);
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    use Command::{Bwserver, Serverusagebw};
    let opts = Options::parse();
    match opts.command {
        Bwserver(opts) => bwserver(opts).await?,
        Serverusagebw(opts) => serverusagebw(opts).await?,
    };
    Ok(())
}
