pub use pallet_bridge_dispatch::Instance1 as WithDarwiniaDispatch;

// --- paritytech ---
use bp_messages::{LaneId, MessageNonce};
use frame_support::traits::Contains;
use pallet_bridge_dispatch::Config;
// --- darwinia-network ---
use crate::*;
use darwinia_messages::FromDarwiniaEncodedCall;

pub struct S2sCallFilter;
impl Contains<Call> for S2sCallFilter {
	fn contains(c: &Call) -> bool {
		matches!(
			c,
			// Call::Substrate2SubstrateBacking(to_substrate_backing::Call::unlock_from_remote(..))
			Call::System(frame_system::Call::remark(_))
		)
	}
}

impl Config<WithDarwiniaDispatch> for Runtime {
	type Event = Event;
	type BridgeMessageId = (LaneId, MessageNonce);
	type Call = Call;
	type CallFilter = S2sCallFilter;
	type EncodedCall = FromDarwiniaEncodedCall;
	type SourceChainAccountId = AccountId;
	type TargetChainAccountPublic = AccountPublic;
	type TargetChainSignature = Signature;
	type AccountIdConverter = AccountIdConverter;
}