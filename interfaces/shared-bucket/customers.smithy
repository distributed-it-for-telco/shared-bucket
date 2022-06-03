namespace com.orange.sharedbucket
use org.wasmcloud.model#wasmbus

/// Description of SharedBucket service
@wasmbus( actorReceive: true )
service Customers {
  version: "0.1",
  operations: [ CreateCustomer, FindCustomer ]
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
  input: String,
  output: AddCustomerReply
}

operation ListCustomers {
  input: String,
  output: ListCustomersReply
}

structure CustomerGroup {
  @required
  name: String,
}

structure CreateCustomerGroupReply {
  @required
  success: Boolean,
}

structure AddCustomerReply {
  customer: Customer
}

list ListCustomersReply {
  member: Customer
}