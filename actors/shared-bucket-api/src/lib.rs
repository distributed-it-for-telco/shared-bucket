use serde::Deserialize;
use shared_bucket::{AddCustomerRequest, Customer, CustomerGroup, CustomerGroups, CustomerGroupsSender, Customers, CustomersSender};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_logging::info;

const CUSTOMERS_ACTOR: &str = "customers";

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct SharedBucketAPIActor {}

/// Util functions
fn deserialize<'de, T: Deserialize<'de>>(raw: &'de [u8]) -> RpcResult<T> {
    serde_json::from_slice(raw).map_err(|e| RpcError::Deser(format!("{}", e)))
}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for SharedBucketAPIActor {
    async fn handle_request(
        &self,
        ctx: &Context,
        req: &HttpRequest,
    ) -> Result<HttpResponse, RpcError> {
        info!("API request: {:?}", req);

        let path = &req.path[1..req.path.len()];
        let segments: Vec<&str> = path.trim_end_matches('/').split('/').collect();
        info!("Segments: {:?}", segments);

        match (req.method.as_ref(), segments.as_slice()) {
            ("POST", ["customers"]) => create_customer(ctx, deserialize(&req.body)?).await,
            ("GET", ["customers", customer_id]) => find_customer(ctx, customer_id).await,
            ("POST", ["customer-groups"]) => create_customer_group(ctx, deserialize(&req.body)?).await,
            ("GET", ["customer-groups", group_name, "customers"]) => get_group_customers(ctx, group_name).await,
            ("POST", ["customer-groups", group_name, "customers", customer_id]) => add_customer_group(ctx, group_name, customer_id).await,
            (_, _) => Ok(HttpResponse::not_found()),
        }
    }
}

async fn create_customer(ctx: &Context, customer: Customer) -> RpcResult<HttpResponse> {
    info!("Customer: {:?}", customer);
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

async fn find_customer(ctx: &Context, customer_id: &str) -> RpcResult<HttpResponse> {
    info!("Find customer : {}", customer_id);
    let x = CustomersSender::to_actor(CUSTOMERS_ACTOR)
        .find_customer(ctx, customer_id)
        .await?;

    match x.customer {
        Some(customer) => HttpResponse::json(customer, 200),
        None => Ok(HttpResponse::not_found())
    }

}

async fn create_customer_group(ctx: &Context, group: CustomerGroup) -> RpcResult<HttpResponse> {
    info!("Customer Group: {:?}", group);
    let x = CustomerGroupsSender::to_actor(CUSTOMERS_ACTOR)
        .create_customer_group(ctx, &group)
        .await?;
    if x.success {
        HttpResponse::json(x, 200)
    } else {
        Ok(HttpResponse::internal_server_error(
            "Failed to create customer",
        ))
    }
}

async fn get_group_customers(ctx: &Context, group_name: &str) -> RpcResult<HttpResponse> {
    info!("Get group customers : {}", group_name);
    let x = CustomerGroupsSender::to_actor(CUSTOMERS_ACTOR)
        .list_customers(ctx, group_name)
        .await?;

    HttpResponse::json(x, 200)
}

async fn add_customer_group(ctx: &Context, group_name: &str, customer_id: &str) -> RpcResult<HttpResponse> {
    info!("Add customer '{}' to group '{}'", customer_id, group_name);

    let x = CustomerGroupsSender::to_actor(CUSTOMERS_ACTOR)
        .add_customer(ctx, &AddCustomerRequest {
            customer: customer_id.to_string(),
            group: group_name.to_string(),
        })
        .await?;

    match x.success {
        true => HttpResponse::json((), 200),
        false => Ok(HttpResponse::internal_server_error("Error adding customer to group")),
    }
}