mod global;
use global::{get_server, proto};

use crate::proto::{BwserverRequest, ServerusagebwRequest};
use proto::streamingcalc_server::{Streamingcalc, StreamingcalcServer};
use std::error::Error;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug, Default)]
struct StreamingcalcService {}

#[tonic::async_trait]
impl Streamingcalc for StreamingcalcService {
    async fn bwserver(
        &self,
        request: Request<BwserverRequest>,
    ) -> Result<Response<proto::Response>, Status> {
        println!("Got a request: {request:?}");
        let input = request.get_ref();
        let response = proto::Response {
            result: 125.0 * input.nblisteners * input.bitrate / 128.0,
        };
        Ok(Response::new(response))
    }
    async fn serverusagebw(
        &self,
        request: Request<ServerusagebwRequest>,
    ) -> Result<Response<proto::Response>, Status> {
        println!("Got a request: {request:?}");
        let input = request.get_ref();
        let response = proto::Response {
            result: 28125.0 * input.nbdays * input.nbhours * input.bitrate * input.nblisteners
                / 65536.0,
        };
        Ok(Response::new(response))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = get_server(false).parse()?;
    let calc = StreamingcalcService::default();
    let service = tonic_reflection::server::Builder::configure()
        .register_encoded_file_descriptor_set(proto::FILE_DESCRIPTOR_SET)
        .build()?;
    Server::builder()
        .add_service(service)
        .add_service(StreamingcalcServer::new(calc))
        .serve(addr)
        .await?;
    Ok(())
}
