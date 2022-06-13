use shared_bucket::{
    AddCustomerReply, CreateCustomerGroupReply, CreateCustomerReply, Customer, CustomerGroup,
    CustomerGroups, Customers, FindCustomerReply, ListCustomersReply,
};
use uuid::Uuid;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{GetResponse, KeyValue, KeyValueSender, SetRequest};
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor)]
struct CustomersActor {}

impl CustomersActor {
    async fn create(ctx: &Context, customer: &Customer) -> anyhow::Result<String> {
        let id = Uuid::new_v4();
        info!("Creating user with id {}", id);
        let request = SetRequest {
            key: id.to_string(),
            value: serde_json::to_string(customer)?,
            expires: 0,
        };

        KeyValueSender::new().set(ctx, &request).await?;

        Ok(id.to_string())
    }

    async fn find(ctx: &Context, id: String) -> anyhow::Result<Option<Customer>> {
        let customer = KeyValueSender::new().get(ctx, &id).await?;

        Ok(if customer.exists {
            Some(serde_json::from_str(&customer.value)?)
        } else {
            None
        })

        // IDIOMATIC ALTERNATIVE //TODO does not compile :)

        // match KeyValueSender::new().get(ctx, &id).await {
        //     Ok(GetResponse { exists: true, value: value } ) => Some(serde_json::from_str(&customer.value)?),
        //     Ok(GetResponse { exists: false, ..} ) => None,
        //     Err(RpcError::NotImplemented) => None, //TODO send error
        //     _ => None //TODO send error
        // }
    }
}

/// Implementation of Customers trait methods
#[async_trait]
impl Customers for CustomersActor {
    async fn create_customer(
        &self,
        ctx: &Context,
        arg: &Customer,
    ) -> RpcResult<CreateCustomerReply> {
        let reply = match Self::create(ctx, arg).await {
            Ok(id) => CreateCustomerReply { id, success: false },
            Err(_) => CreateCustomerReply {
                id: "".to_string(),
                success: false,
            },
        };

        Ok(reply)
    }

    async fn find_customer<TS: ToString + ?Sized + Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<FindCustomerReply> {
        match Self::find(ctx, arg.to_string()).await {
            Ok(customer) => Ok(FindCustomerReply { customer }),
            Err(_) => Ok(FindCustomerReply { customer: None }),
        }
    }
}

#[async_trait]
impl CustomerGroups for CustomersActor {
    async fn create_customer_group(
        &self,
        ctx: &Context,
        arg: &CustomerGroup,
    ) -> RpcResult<CreateCustomerGroupReply> {
        todo!()
    }

    async fn add_customer<TS: ToString + ?Sized + Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<AddCustomerReply> {
        todo!()
    }

    async fn list_customers<TS: ToString + ?Sized + Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<ListCustomersReply> {
        todo!()
    }
}
