// customers.smithy
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "com.orange.sharedbucket", crate: "customers" } ]

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