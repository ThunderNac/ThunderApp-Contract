#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Bytes};

#[contract]
pub struct NFTContract;

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Owner(u128),
    TokenURI(u128),
}

#[contractimpl]
impl NFTContract {
    pub fn mint(env: Env, to: Address, token_id: u128, uri: Bytes) {
        let key = DataKey::Owner(token_id);

        if env.storage().instance().has(&key) {
            panic!("Token already minted");
        }

        env.storage().instance().set(&key, &to);
        env.storage().instance().set(&DataKey::TokenURI(token_id), &uri);
    }

    pub fn owner_of(env: Env, token_id: u128) -> Address {
        let key = DataKey::Owner(token_id);
        match env.storage().instance().get::<_, Address>(&key) {
            Some(owner) => owner,
            None => panic!("Token does not exist"),
        }
    }

    pub fn token_uri(env: Env, token_id: u128) -> Bytes {
        let key = DataKey::TokenURI(token_id);
        match env.storage().instance().get::<_, Bytes>(&key) {
            Some(uri) => uri,
            None => panic!("URI not found"),
        }
    }
}

mod test_nft;