//! Replicated state types.
//!
//! Note [Replicated State Invariants]
//! ==================================
//!
//! Guidelines for handling invariants that are internal to datastructures in the
//! replicated state:
//!
//! - In this context, the term invariant is used to refer to something that (1) holds
//!   all the time, and (2) whose violation would affect code correctness:
//!   - We check these during deserialization and return an error if they don't hold.
//!   - It is also fine to assert/debug_assert (depending on how expensive these checks
//!     are) for them in production code.
//!   - Proptests for these invariants are recommended, but can be skipped if there is
//!     consensus that they are not needed.
//! - There can also be soft invariants which are a superset of the invariants above.
//!   - These include things that don't affect correctness of the code, but we still
//!     aim to uphold them at all times.
//!   - They can be self healing, i.e., a violation will be fixed upon the next
//!     modification of after the next couple of modifications.
//!   - We don't assert for them in production code, but may debug_assert and raise
//!     critical errors in case of a violation upon deserialization (cf. deserialization
//!     of `BlockmakerMetricsTimeSeries`)
//!   - An example for a soft invariant is an upper bound on the number of elements
//!     in a datastructure which maintains a sliding window of snapshots, where the
//!     actual number of snapshots does not affect correctness and we just want to make
//!     sure it doesn't grow indefinitely.
//! - We don't attempt to restore invariants or soft invariants upon deserializing
//!   as it could change the past.
//!
mod bitcoin;
pub mod canister_state;
pub(crate) mod hash;
pub mod metadata_state;
pub mod page_map;
pub mod replicated_state;
pub mod testing {
    pub use super::canister_state::system_state::testing::SystemStateTesting;
    pub use super::canister_state::testing::CanisterQueuesTesting;
    pub use super::replicated_state::testing::ReplicatedStateTesting;
}
pub use canister_state::{
    execution_state::Memory,
    num_bytes_try_from,
    system_state::{
        memory_required_to_push_request, CallContext, CallContextAction, CallContextManager,
        CallOrigin, CanisterMetrics, CanisterStatus, ExecutionTask, SystemState,
    },
    CanisterQueues, CanisterState, EmbedderCache, ExecutionState, ExportedFunctions, Global,
    NumWasmPages, SchedulerState,
};
pub use metadata_state::{
    IngressHistoryState, NetworkTopology, Stream, SubnetTopology, SystemMetadata,
};
pub use page_map::{PageIndex, PageMap};
pub use replicated_state::{InputQueueType, NextInputQueue, ReplicatedState, StateError};

/// Encapsulates metrics related to errors that can occur on checkpoint loading.
/// The intention is to pass an implementation of this trait along with the actual
/// struct to deserialize to the deserialization logic so that there is a way
/// to record metrics from there.
pub trait CheckpointLoadingMetrics {
    fn observe_broken_soft_invariant(&self, msg: String);
}
