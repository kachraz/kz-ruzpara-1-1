
# Proposed Change to `ExecutionPayloadInputV1`

Based on the `TODO` in `crates/rpc/rpc-types/src/eth/engine.rs`, I propose the following changes to the `ExecutionPayloadInputV1` struct to align it with the geth implementation.

## Current Implementation

```rust
// crates/rpc/rpc-types/src/eth/engine.rs

// TODO: Add fields according to geth
// <https://github.com/paradigmxyz/reth/pull/8853>
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPayloadInputV1 {
    /// The V6 payload.
    #[serde(flatten)]
    pub payload_v6: ExecutionPayloadV6,
}
```

## Proposed Change

```rust
// crates/rpc/rpc-types/src/eth/engine.rs

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionPayloadInputV1 {
    /// The V6 payload.
    #[serde(flatten)]
    pub payload_v6: ExecutionPayloadV6,
    /// The parent beacon block root.
    pub parent_beacon_block_root: Option<B256>,
}
```

This change adds the `parent_beacon_block_root` field to the `ExecutionPayloadInputV1` struct, which is in line with the geth implementation. This is a non-breaking change and only involves adding a new field.
