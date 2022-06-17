namespace com.orange.sharedbucket
use org.wasmcloud.model#wasmbus

/// Description of SharedBucket service
@wasmbus( actorReceive: true )
service Customers {
  version: "0.1",
  operations: [ CreateCustomer, FindCustomer, Healthz ]
}

operation Healthz {
  input: HealthzRequest,
  output: HealthzReply
}


operation CreateCustomer {
  input: Customer,
  output: CreateCustomerReply
}

operation FindCustomer {
  input: String,
  output: FindCustomerReply
}

structure Customer {
  id: String,
  @required
  firstName: String,
  lastName: String,
  address: String,
  city: String,
  telephone: String,
  @required
  email: String
}

structure CreateCustomerReply {
  @required
  success: Boolean,
  @required
  id: String
}

structure FindCustomerReply {
  customer: Customer
}

@wasmbus( actorReceive: true )
service CustomerGroups {
  version: "0.1",
  operations: [ CreateCustomerGroup, AddCustomer, ListCustomers ]
}

operation CreateCustomerGroup {
  input: CustomerGroup,
  output: CreateCustomerGroupReply
}

operation AddCustomer {
  input: AddCustomerRequest,
  output: AddCustomerReply
}

operation ListCustomers {
  input: String,
  output: ListCustomersReply
}

structure CustomerGroup {
  @required
  name: String,
  customers: GroupCustomers,
}

list GroupCustomers {
  member: String
}

structure CreateCustomerGroupReply {
  @required
  success: Boolean,
}

structure AddCustomerRequest {
  @required
  group: String,
  @required
  customer: String,
}

structure AddCustomerReply {
  success: Boolean
}

list ListCustomersReply {
  member: Customer
}

structure HealthzRequest {
}

structure HealthzReply {
  success: Boolean
}