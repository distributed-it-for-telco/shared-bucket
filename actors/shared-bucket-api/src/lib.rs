use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use shared_bucket::{Customers};
use wasmcloud_interface_logging::debug;

const CUSTOMERS_ACTORS: &str = "shared_bucket/customers"; //TODO not sure about the prefix here
const VISITS_ACTOR: &str = "shared_bucket/visits";


#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct SharedBucketAPIActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for SharedBucketAPIActor {

    async fn handle_request(
        &self,
        _ctx: &Context,
        req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        debug!("API request: {:?}", req);

        let path = &req.path[1..req.path.len()];
        let segments: Vec<&str> = path.trim_end_matches('/').split('/').collect();
        debug!("Segments: {:?}", segments);

        match (req.method.as_ref(), segments.as_slice()) {
            ("GET", ["customers", customer_id]) => todo!(),
            (_, _) => Ok(HttpResponse::not_found()),
       }
    }
}
