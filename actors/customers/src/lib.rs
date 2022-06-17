use anyhow::bail;
use shared_bucket::{AddCustomerReply, AddCustomerRequest, CreateCustomerGroupReply, CreateCustomerReply, Customer, CustomerGroup, CustomerGroups, CustomerGroupsReceiver, Customers, CustomersReceiver, FindCustomerReply, HealthzReply, HealthzRequest, ListCustomersReply};

use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_keyvalue::{GetResponse, KeyValue, KeyValueSender, SetRequest};
use wasmcloud_interface_logging::{error, info};
use wasmcloud_interface_numbergen::generate_guid;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Customers, CustomerGroups)]
struct CustomersActor {}

impl CustomersActor {
    async fn create(ctx: &Context, customer: &Customer) -> anyhow::Result<String> {

        let id = generate_guid().await?;
        info!("Creating customer with id {}", id);
        let request = SetRequest {
            key: format!("customer:{}", &id),
            value: serde_json::to_string(customer)?,
            expires: 0,
        };

        KeyValueSender::new().set(ctx, &request).await?;

        Ok(id)
    }

    async fn find(ctx: &Context, id: String) -> anyhow::Result<Option<Customer>> {
        match KeyValueSender::new().get(ctx, &format!("customer:{}", id)).await {
            Ok(GetResponse { exists: true, value }) => Ok(Some(serde_json::from_str(&value)?)),
            Ok(GetResponse { exists: false, .. }) => Ok(None),
            Err(e) => bail!("Error searching for customer '{}' : {:?}", id, e)
        }
    }

    async fn create_group(ctx: &Context, group: &CustomerGroup) -> anyhow::Result<()> {

        info!("Creating customer group '{}'", group.name);
        let request = SetRequest {
            key: format!("customer_group:{}", group.name),
            value: serde_json::to_string(group)?,
            expires: 0,
        };

        KeyValueSender::new().set(ctx, &request).await?;

        Ok(())
    }

    async fn find_group(ctx: &Context, name: &String) -> anyhow::Result<Option<CustomerGroup>> {
        match KeyValueSender::new().get(ctx, &&format!("customer_group:{}", name)).await {
            Ok(GetResponse { exists: true, value }) => Ok(Some(serde_json::from_str(&value)?)),
            Ok(GetResponse { exists: false, .. }) => Ok(None),
            Err(e) => bail!("Error searching for customer group '{}' : {:?}", name, e)
        }
    }

    async fn add_customer_to_group(ctx: &Context, group: &String, customer: &String) -> anyhow::Result<()> {

        let group = match Self::find_group(ctx, group).await {
            Ok(Some(group)) => group,
            Ok(None) => bail!("Group does not exists '{}'", group),
            Err(e) => bail!("Error searching for group '{}' : {}", group, e),
        };

        let mut group = group;
        if group.customers.is_none() {
            group.customers = Some(Vec::new());
        }

        let customers = group.customers.as_mut().unwrap();

        if !customers.contains(customer) {
            customers.push(customer.clone());

            let request = SetRequest {
                key: format!("customer_group:{}", group.name),
                value: serde_json::to_string(&group)?,
                expires: 0,
            };

            KeyValueSender::new().set(ctx, &request).await?;
        }

        Ok(())
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
        info!("create customer");

        let reply = match Self::create(ctx, arg).await {
            Ok(id) => {
                info!("Customer created : {}", id);
                CreateCustomerReply { id, success: true }
            },
            Err(e) => {
                error!("Error creating customer : {}", e);
                CreateCustomerReply {
                    id: "".to_string(),
                    success: false,
                }
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

    async fn healthz(&self, ctx: &Context, arg: &HealthzRequest) -> RpcResult<HealthzReply> {
        Ok(HealthzReply { success: true })
    }
}

#[async_trait]
impl CustomerGroups for CustomersActor {
    async fn create_customer_group(
        &self,
        ctx: &Context,
        arg: &CustomerGroup,
    ) -> RpcResult<CreateCustomerGroupReply> {
        info!("create customer group");

        match Self::find_group(ctx, &arg.name).await {
            Ok(Some(_)) => {
                error!("The group '{}' already exists", arg.name);
                return Err(RpcError::Other(format!("The group '{}' already exists", arg.name)))
            },
            _ => {}
        }

        let reply = match Self::create_group(ctx, arg).await {
            Ok(_) => {
                info!("Group created : {}", arg.name);
                CreateCustomerGroupReply { success: true }
            },
            Err(e) => {
                error!("Error creating customer : {}", e);
                CreateCustomerGroupReply { success: false }
            },
        };

        Ok(reply)
    }

    async fn add_customer(&self, ctx: &Context, arg: &AddCustomerRequest) -> RpcResult<AddCustomerReply> {
        let success = match Self::add_customer_to_group(ctx, &arg.group, &arg.customer).await {
            Ok(()) => true,
            Err(_) => false
        };

        Ok(AddCustomerReply {
            success
        })
    }


    async fn list_customers<TS: ToString + ?Sized + Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<ListCustomersReply> {

        match Self::find_group(ctx, &arg.to_string()).await {
            Ok(Some(group)) => {

                let mut res = Vec::new();
                match group.customers {
                    Some(customers) => {
                        for customer_id in customers {
                            match Self::find(ctx, customer_id).await {
                                Ok(Some(customer)) => res.push(customer),
                                _ => {}
                            }
                        }
                    },
                    None => {}
                }

                Ok(res)
            },
            _ => {
                Err(RpcError::Other(format!("The group '{}' doesn't exists", arg.to_string())))
            }
        }
    }
}
