# Ingress and Bridge Context

M2.4 routes before materializing request data. Request ownership moves into a worker-local slab. Handles remain opaque and generation-checked. Headers/query/body are decoded only when the verified RoutePlan requires them. Backpressure and read-once body behavior are mandatory.
