use std::str::FromStr;

use rucord_api_types::GatewayDispatchEvents;

#[test]
fn test_gateway_event() {
    assert_eq!(
        GatewayDispatchEvents::from_str("READY"),
        Ok(GatewayDispatchEvents::Ready)
    )
}
