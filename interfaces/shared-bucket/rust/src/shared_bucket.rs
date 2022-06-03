// This file is @generated by wasmcloud/weld-codegen 0.4.5.
// It is not intended for manual editing.
// namespace: com.orange.sharedbucket

#[allow(unused_imports)]
use async_trait::async_trait;
#[allow(unused_imports)]
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use std::{borrow::Borrow, borrow::Cow, io::Write, string::ToString};
#[allow(unused_imports)]
use wasmbus_rpc::{
    //cbor::*,
    common::{
        deserialize, message_format, serialize, Context, Message, MessageDispatch, MessageFormat,
        SendOpts, Transport,
    },
    error::{RpcError, RpcResult},
    Timestamp,
};

#[allow(dead_code)]
pub const SMITHY_VERSION: &str = "1.0";

#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct CreateCustomerReply {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub success: bool,
}

// Encode CreateCustomerReply as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_create_customer_reply<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &CreateCustomerReply,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("id")?;
    e.str(&val.id)?;
    e.str("success")?;
    e.bool(val.success)?;
    Ok(())
}

// Decode CreateCustomerReply from cbor input stream
#[doc(hidden)]
pub fn decode_create_customer_reply(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<CreateCustomerReply, RpcError> {
    let __result = {
        let mut id: Option<String> = None;
        let mut success: Option<bool> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct CreateCustomerReply, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => id = Some(d.str()?.to_string()),
                    1 => success = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "id" => id = Some(d.str()?.to_string()),
                    "success" => success = Some(d.bool()?),
                    _ => d.skip()?,
                }
            }
        }
        CreateCustomerReply {
            id: if let Some(__x) = id {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field CreateCustomerReply.id (#0)".to_string(),
                ));
            },

            success: if let Some(__x) = success {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field CreateCustomerReply.success (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Customer {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    #[serde(default)]
    pub email: String,
    #[serde(rename = "firstName")]
    #[serde(default)]
    pub first_name: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "lastName")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub telephone: Option<String>,
}

// Encode Customer as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_customer<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Customer,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(7)?;
    if let Some(val) = val.address.as_ref() {
        e.str("address")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.city.as_ref() {
        e.str("city")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    e.str("email")?;
    e.str(&val.email)?;
    e.str("firstName")?;
    e.str(&val.first_name)?;
    if let Some(val) = val.id.as_ref() {
        e.str("id")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.last_name.as_ref() {
        e.str("lastName")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    if let Some(val) = val.telephone.as_ref() {
        e.str("telephone")?;
        e.str(val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode Customer from cbor input stream
#[doc(hidden)]
pub fn decode_customer(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Customer, RpcError> {
    let __result = {
        let mut address: Option<Option<String>> = Some(None);
        let mut city: Option<Option<String>> = Some(None);
        let mut email: Option<String> = None;
        let mut first_name: Option<String> = None;
        let mut id: Option<Option<String>> = Some(None);
        let mut last_name: Option<Option<String>> = Some(None);
        let mut telephone: Option<Option<String>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Customer, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        address = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    1 => {
                        city = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    2 => email = Some(d.str()?.to_string()),
                    3 => first_name = Some(d.str()?.to_string()),
                    4 => {
                        id = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    5 => {
                        last_name = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    6 => {
                        telephone = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "address" => {
                        address = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "city" => {
                        city = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "email" => email = Some(d.str()?.to_string()),
                    "firstName" => first_name = Some(d.str()?.to_string()),
                    "id" => {
                        id = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "lastName" => {
                        last_name = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    "telephone" => {
                        telephone = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(d.str()?.to_string()))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        Customer {
            address: address.unwrap(),
            city: city.unwrap(),

            email: if let Some(__x) = email {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Customer.email (#2)".to_string(),
                ));
            },

            first_name: if let Some(__x) = first_name {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field Customer.first_name (#3)".to_string(),
                ));
            },
            id: id.unwrap(),
            last_name: last_name.unwrap(),
            telephone: telephone.unwrap(),
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct FindCustomerReply {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub customer: Option<Customer>,
}

// Encode FindCustomerReply as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_find_customer_reply<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &FindCustomerReply,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(1)?;
    if let Some(val) = val.customer.as_ref() {
        e.str("customer")?;
        encode_customer(e, val)?;
    } else {
        e.null()?;
    }
    Ok(())
}

// Decode FindCustomerReply from cbor input stream
#[doc(hidden)]
pub fn decode_find_customer_reply(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<FindCustomerReply, RpcError> {
    let __result = {
        let mut customer: Option<Option<Customer>> = Some(None);

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct FindCustomerReply, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => {
                        customer = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_customer(d).map_err(|e| {
                                format!("decoding 'com.orange.sharedbucket#Customer': {}", e)
                            })?))
                        }
                    }

                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "customer" => {
                        customer = if wasmbus_rpc::cbor::Type::Null == d.datatype()? {
                            d.skip()?;
                            Some(None)
                        } else {
                            Some(Some(decode_customer(d).map_err(|e| {
                                format!("decoding 'com.orange.sharedbucket#Customer': {}", e)
                            })?))
                        }
                    }
                    _ => d.skip()?,
                }
            }
        }
        FindCustomerReply {
            customer: customer.unwrap(),
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct Service {
    #[serde(default)]
    pub id: String,
}

// Encode Service as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_service<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &Service,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(1)?;
    e.str("id")?;
    e.str(&val.id)?;
    Ok(())
}

// Decode Service from cbor input stream
#[doc(hidden)]
pub fn decode_service(d: &mut wasmbus_rpc::cbor::Decoder<'_>) -> Result<Service, RpcError> {
    let __result = {
        let mut id: Option<String> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct Service, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => id = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "id" => id = Some(d.str()?.to_string()),
                    _ => d.skip()?,
                }
            }
        }
        Service {
            id: if let Some(__x) = id {
                __x
            } else {
                return Err(RpcError::Deser("missing field Service.id (#0)".to_string()));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ServiceOrder {
    #[serde(default)]
    pub amount: String,
    pub service: Service,
}

// Encode ServiceOrder as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_service_order<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ServiceOrder,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("amount")?;
    e.str(&val.amount)?;
    e.str("service")?;
    encode_service(e, &val.service)?;
    Ok(())
}

// Decode ServiceOrder from cbor input stream
#[doc(hidden)]
pub fn decode_service_order(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ServiceOrder, RpcError> {
    let __result = {
        let mut amount: Option<String> = None;
        let mut service: Option<Service> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ServiceOrder, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => amount = Some(d.str()?.to_string()),
                    1 => {
                        service = Some(decode_service(d).map_err(|e| {
                            format!("decoding 'com.orange.sharedbucket#Service': {}", e)
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "amount" => amount = Some(d.str()?.to_string()),
                    "service" => {
                        service = Some(decode_service(d).map_err(|e| {
                            format!("decoding 'com.orange.sharedbucket#Service': {}", e)
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        }
        ServiceOrder {
            amount: if let Some(__x) = amount {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ServiceOrder.amount (#0)".to_string(),
                ));
            },

            service: if let Some(__x) = service {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ServiceOrder.service (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
#[derive(Clone, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub struct ServiceUsage {
    #[serde(default)]
    pub client: String,
    pub service: Service,
}

// Encode ServiceUsage as CBOR and append to output stream
#[doc(hidden)]
#[allow(unused_mut)]
pub fn encode_service_usage<W: wasmbus_rpc::cbor::Write>(
    mut e: &mut wasmbus_rpc::cbor::Encoder<W>,
    val: &ServiceUsage,
) -> RpcResult<()>
where
    <W as wasmbus_rpc::cbor::Write>::Error: std::fmt::Display,
{
    e.map(2)?;
    e.str("client")?;
    e.str(&val.client)?;
    e.str("service")?;
    encode_service(e, &val.service)?;
    Ok(())
}

// Decode ServiceUsage from cbor input stream
#[doc(hidden)]
pub fn decode_service_usage(
    d: &mut wasmbus_rpc::cbor::Decoder<'_>,
) -> Result<ServiceUsage, RpcError> {
    let __result = {
        let mut client: Option<String> = None;
        let mut service: Option<Service> = None;

        let is_array = match d.datatype()? {
            wasmbus_rpc::cbor::Type::Array => true,
            wasmbus_rpc::cbor::Type::Map => false,
            _ => {
                return Err(RpcError::Deser(
                    "decoding struct ServiceUsage, expected array or map".to_string(),
                ))
            }
        };
        if is_array {
            let len = d.fixed_array()?;
            for __i in 0..(len as usize) {
                match __i {
                    0 => client = Some(d.str()?.to_string()),
                    1 => {
                        service = Some(decode_service(d).map_err(|e| {
                            format!("decoding 'com.orange.sharedbucket#Service': {}", e)
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        } else {
            let len = d.fixed_map()?;
            for __i in 0..(len as usize) {
                match d.str()? {
                    "client" => client = Some(d.str()?.to_string()),
                    "service" => {
                        service = Some(decode_service(d).map_err(|e| {
                            format!("decoding 'com.orange.sharedbucket#Service': {}", e)
                        })?)
                    }
                    _ => d.skip()?,
                }
            }
        }
        ServiceUsage {
            client: if let Some(__x) = client {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ServiceUsage.client (#0)".to_string(),
                ));
            },

            service: if let Some(__x) = service {
                __x
            } else {
                return Err(RpcError::Deser(
                    "missing field ServiceUsage.service (#1)".to_string(),
                ));
            },
        }
    };
    Ok(__result)
}
/// Description of SharedBucket service
/// wasmbus.actorReceive
#[async_trait]
pub trait Customers {
    async fn create_customer(
        &self,
        ctx: &Context,
        arg: &Customer,
    ) -> RpcResult<CreateCustomerReply>;
    async fn find_customer<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<FindCustomerReply>;
}

/// CustomersReceiver receives messages defined in the Customers service trait
/// Description of SharedBucket service
#[doc(hidden)]
#[async_trait]
pub trait CustomersReceiver: MessageDispatch + Customers {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "CreateCustomer" => {
                let value: Customer = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'Customer': {}", e)))?;

                let resp = Customers::create_customer(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(Message {
                    method: "Customers.CreateCustomer",
                    arg: Cow::Owned(buf),
                })
            }
            "FindCustomer" => {
                let value: String = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'String': {}", e)))?;

                let resp = Customers::find_customer(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(Message {
                    method: "Customers.FindCustomer",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "Customers::{}",
                message.method
            ))),
        }
    }
}

/// CustomersSender sends messages to a Customers service
/// Description of SharedBucket service
/// client for sending Customers messages
#[derive(Debug)]
pub struct CustomersSender<T: Transport> {
    transport: T,
}

impl<T: Transport> CustomersSender<T> {
    /// Constructs a CustomersSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> CustomersSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl CustomersSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> Customers for CustomersSender<T> {
    #[allow(unused)]
    async fn create_customer(
        &self,
        ctx: &Context,
        arg: &Customer,
    ) -> RpcResult<CreateCustomerReply> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.CreateCustomer",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: CreateCustomerReply = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': CreateCustomerReply", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn find_customer<TS: ToString + ?Sized + std::marker::Sync>(
        &self,
        ctx: &Context,
        arg: &TS,
    ) -> RpcResult<FindCustomerReply> {
        let buf = wasmbus_rpc::common::serialize(&arg.to_string())?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "Customers.FindCustomer",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: FindCustomerReply = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': FindCustomerReply", e)))?;
        Ok(value)
    }
}

/// Description of SharedBucket service
/// wasmbus.actorReceive
#[async_trait]
pub trait ServiceVendors {
    async fn authorize_service_usage(&self, ctx: &Context, arg: &ServiceUsage)
        -> RpcResult<String>;
    async fn buy_service(&self, ctx: &Context, arg: &ServiceOrder) -> RpcResult<String>;
}

/// ServiceVendorsReceiver receives messages defined in the ServiceVendors service trait
/// Description of SharedBucket service
#[doc(hidden)]
#[async_trait]
pub trait ServiceVendorsReceiver: MessageDispatch + ServiceVendors {
    async fn dispatch<'disp__, 'ctx__, 'msg__>(
        &'disp__ self,
        ctx: &'ctx__ Context,
        message: &Message<'msg__>,
    ) -> Result<Message<'msg__>, RpcError> {
        match message.method {
            "AuthorizeServiceUsage" => {
                let value: ServiceUsage = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'ServiceUsage': {}", e)))?;

                let resp = ServiceVendors::authorize_service_usage(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(Message {
                    method: "ServiceVendors.AuthorizeServiceUsage",
                    arg: Cow::Owned(buf),
                })
            }
            "BuyService" => {
                let value: ServiceOrder = wasmbus_rpc::common::deserialize(&message.arg)
                    .map_err(|e| RpcError::Deser(format!("'ServiceOrder': {}", e)))?;

                let resp = ServiceVendors::buy_service(self, ctx, &value).await?;
                let buf = wasmbus_rpc::common::serialize(&resp)?;

                Ok(Message {
                    method: "ServiceVendors.BuyService",
                    arg: Cow::Owned(buf),
                })
            }
            _ => Err(RpcError::MethodNotHandled(format!(
                "ServiceVendors::{}",
                message.method
            ))),
        }
    }
}

/// ServiceVendorsSender sends messages to a ServiceVendors service
/// Description of SharedBucket service
/// client for sending ServiceVendors messages
#[derive(Debug)]
pub struct ServiceVendorsSender<T: Transport> {
    transport: T,
}

impl<T: Transport> ServiceVendorsSender<T> {
    /// Constructs a ServiceVendorsSender with the specified transport
    pub fn via(transport: T) -> Self {
        Self { transport }
    }

    pub fn set_timeout(&self, interval: std::time::Duration) {
        self.transport.set_timeout(interval);
    }
}

#[cfg(not(target_arch = "wasm32"))]
impl<'send> ServiceVendorsSender<wasmbus_rpc::provider::ProviderTransport<'send>> {
    /// Constructs a Sender using an actor's LinkDefinition,
    /// Uses the provider's HostBridge for rpc
    pub fn for_actor(ld: &'send wasmbus_rpc::core::LinkDefinition) -> Self {
        Self {
            transport: wasmbus_rpc::provider::ProviderTransport::new(ld, None),
        }
    }
}
#[cfg(target_arch = "wasm32")]
impl ServiceVendorsSender<wasmbus_rpc::actor::prelude::WasmHost> {
    /// Constructs a client for actor-to-actor messaging
    /// using the recipient actor's public key
    pub fn to_actor(actor_id: &str) -> Self {
        let transport =
            wasmbus_rpc::actor::prelude::WasmHost::to_actor(actor_id.to_string()).unwrap();
        Self { transport }
    }
}
#[async_trait]
impl<T: Transport + std::marker::Sync + std::marker::Send> ServiceVendors
    for ServiceVendorsSender<T>
{
    #[allow(unused)]
    async fn authorize_service_usage(
        &self,
        ctx: &Context,
        arg: &ServiceUsage,
    ) -> RpcResult<String> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "ServiceVendors.AuthorizeServiceUsage",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: String = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': String", e)))?;
        Ok(value)
    }
    #[allow(unused)]
    async fn buy_service(&self, ctx: &Context, arg: &ServiceOrder) -> RpcResult<String> {
        let buf = wasmbus_rpc::common::serialize(arg)?;

        let resp = self
            .transport
            .send(
                ctx,
                Message {
                    method: "ServiceVendors.BuyService",
                    arg: Cow::Borrowed(&buf),
                },
                None,
            )
            .await?;

        let value: String = wasmbus_rpc::common::deserialize(&resp)
            .map_err(|e| RpcError::Deser(format!("'{}': String", e)))?;
        Ok(value)
    }
}
