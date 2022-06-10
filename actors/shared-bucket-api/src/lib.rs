use serde::{Deserialize, Serialize};
use shared_bucket::{Customer, Customers, CustomersSender};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::debug;

const CUSTOMERS_ACTOR: &str = "customers"; //TODO not sure about the prefix here
const SERVICE_VENDORS_ACTOR: &str = "service_vendors"; //TODO not sure about anything here

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct SharedBucketAPIActor {}

/// Util functions
fn deser<'de, T: Deserialize<'de>>(raw: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(raw).map_err(|e| RpcError::Deser(format!("{}", e)))
}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for SharedBucketAPIActor {
    async fn handle_request(
        &self,
        ctx: &Context,
        req: &HttpRequest,
    ) -> std::result::Result<HttpResponse, RpcError> {
        debug!("API request: {:?}", req);

        let path = &req.path[1..req.path.len()];
        let segments: Vec<&str> = path.trim_end_matches('/').split('/').collect();
        debug!("Segments: {:?}", segments);

        match (req.method.as_ref(), segments.as_slice()) {
            ("POST", ["customers"]) => create_customer(ctx, deser(&req.body)?).await,
            (_, _) => Ok(HttpResponse::not_found()),
        }
    }
}

async fn create_customer(ctx: &Context, customer: Customer) -> RpcResult<HttpResponse> {
    let x = CustomersSender::to_actor(CUSTOMERS_ACTOR)
        .create_customer(ctx, &customer)
        .await?;
    if x.success {
        HttpResponse::json(x, 200)
    } else {
        Ok(HttpResponse::internal_server_error(
            "Failed to create customer",
        ))
    }
}
