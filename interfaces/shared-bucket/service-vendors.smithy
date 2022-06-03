// customers.smithy
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "com.orange.sharedbucket", crate: "service-vendors" } ]

namespace com.orange.sharedbucket

use org.wasmcloud.model#wasmbus

/// Description of SharedBucket service
@wasmbus( actorReceive: true )
service ServiceVendors {
  version: "0.1",
  operations: [ AuthorizeServiceUsage, BuyService ]
}

operation AuthorizeServiceUsage {
  input: ServiceUsage,
  output: String
}

operation BuyService {
  input: ServiceOrder,
  output: String
}

structure Service {
  @required
  id: String
}

structure ServiceUsage {
  @required
  service: Service,
  @required
  client: String
}

structure ServiceOrder {
  @required
  service: Service,
  @required
  amount: String
}
