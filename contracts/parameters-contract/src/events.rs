use soroban_sdk::{symbol_short, Address, Env, Symbol};

use crate::types::ProtocolParameters;

const PARAMS_UPDATED: Symbol = symbol_short!("PARMUPDT");
const ADMIN_UPDATED: Symbol = symbol_short!("PARMADMN");

pub fn emit_parameters_updated(env: &Env, admin: &Address, params: &ProtocolParameters) {
    env.events().publish(
        (PARAMS_UPDATED, admin),
        (
            params.min_guarantee_percent,
            params.min_reputation_threshold,
            params.full_repayment_reward,
            params.default_penalty,
            params.large_loan_threshold,
            params.large_loan_default_penalty,
            params.base_interest_bps,
        ),
    );
}

pub fn emit_admin_updated(env: &Env, old_admin: &Address, new_admin: &Address) {
    env.events()
        .publish((ADMIN_UPDATED, old_admin), new_admin.clone());
}
