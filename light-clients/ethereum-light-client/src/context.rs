use ethereum_verifier::context::LightClientContext as TLightClientContext;
use unionlabs::{
    ethereum::config::ChainSpec,
    ibc::lightclients::ethereum::{
        client_state::ClientState, fork_parameters::ForkParameters, sync_committee::SyncCommittee,
        trusted_sync_committee::ActiveSyncCommittee,
    },
};

use crate::consensus_state::TrustedConsensusState;

pub struct LightClientContext<'a, C: ChainSpec> {
    pub client_state: &'a ClientState,
    pub trusted_consensus_state: TrustedConsensusState<C>,
}

impl<'a, C: ChainSpec> LightClientContext<'a, C> {
    pub fn new(
        client_state: &'a ClientState,
        trusted_consensus_state: TrustedConsensusState<C>,
    ) -> Self {
        Self {
            client_state,
            trusted_consensus_state,
        }
    }
}

impl<'a, C: ChainSpec> TLightClientContext for LightClientContext<'a, C> {
    type ChainSpec = C;

    fn finalized_slot(&self) -> u64 {
        self.trusted_consensus_state.state.slot
    }

    fn current_sync_committee(&self) -> Option<&SyncCommittee<C>> {
        if let ActiveSyncCommittee::Current(committee) =
            &self.trusted_consensus_state.sync_committee
        {
            Some(committee)
        } else {
            None
        }
    }

    fn next_sync_committee(&self) -> Option<&SyncCommittee<C>> {
        if let ActiveSyncCommittee::Next(committee) = &self.trusted_consensus_state.sync_committee {
            Some(committee)
        } else {
            None
        }
    }

    fn fork_parameters(&self) -> &ForkParameters {
        &self.client_state.fork_parameters
    }
}
