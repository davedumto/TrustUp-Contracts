use soroban_sdk::{symbol_short, Address, Env, Symbol};

use crate::types::ProtocolParameters;

pub const ADMIN_KEY: Symbol = symbol_short!("ADMIN");
pub const PARAMS_KEY: Symbol = symbol_short!("PARAMS");

pub fn has_admin(env: &Env) -> bool {
    env.storage().instance().has(&ADMIN_KEY)
}

pub fn get_admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&ADMIN_KEY)
        .expect("parameters admin not set")
}

pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&ADMIN_KEY, admin);
}

pub fn get_parameters(env: &Env) -> ProtocolParameters {
    env.storage()
        .instance()
        .get(&PARAMS_KEY)
        .expect("parameters not set")
}

pub fn set_parameters(env: &Env, params: &ProtocolParameters) {
    env.storage().instance().set(&PARAMS_KEY, params);
}
