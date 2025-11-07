#![no_std]
use soroban_sdk::{contractimpl, contracttype, symbol_short, Address, Env, Symbol, Map};
use soroban_token_sdk::{token, TokenClient};

pub struct Vault;

#[contracttype]
#[derive(Clone)]
pub struct VaultMetadata {
    pub token_address: Address,
    pub total_shares: i128,
}

#[contractimpl]
impl Vault {
    pub fn init(env: Env, token_address: Address) {
        let metadata = VaultMetadata {
            token_address,
            total_shares: 0,
        };
        env.storage().set(symbol_short!("metadata"), metadata);
        env.storage().set(symbol_short!("balances"), Map::<Address, i128>::new(&env));
        env.storage().set(symbol_short!("claimed_profit"), Map::<Address, i128>::new(&env));
        env.storage().set(symbol_short!("total_profit"), 0i128);
    }

    pub fn deposit(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let mut metadata: VaultMetadata = env.storage().get(symbol_short!("metadata")).unwrap();
        let mut balances: Map<Address, i128> = env
            .storage()
            .get(symbol_short!("balances"))
            .unwrap_or(Map::new(&env));

        let token = TokenClient::new(&env, &metadata.token_address);
        token.transfer_from(&from, &env.current_contract_address(), &amount);

        let prev_balance = balances.get(from.clone()).unwrap_or(0);
        balances.set(from.clone(), prev_balance + amount);
        metadata.total_shares += amount;

        env.storage().set(symbol_short!("balances"), balances);
        env.storage().set(symbol_short!("metadata"), metadata);
    }

    pub fn withdraw(env: Env, to: Address, share_amount: i128) {
        to.require_auth();

        let mut metadata: VaultMetadata = env.storage().get(symbol_short!("metadata")).unwrap();
        let mut balances: Map<Address, i128> = env
            .storage()
            .get(symbol_short!("balances"))
            .unwrap_or(Map::new(&env));

        let user_balance = balances.get(to.clone()).unwrap_or(0);
        if user_balance < share_amount {
            panic!("Not enough shares");
        }

        balances.set(to.clone(), user_balance - share_amount);
        metadata.total_shares -= share_amount;

        let token = TokenClient::new(&env, &metadata.token_address);
        token.transfer(&to, &share_amount);

        env.storage().set(symbol_short!("balances"), balances);
        env.storage().set(symbol_short!("metadata"), metadata);
    }

    /// ✅ Admin injecte du rendement dans le vault
    pub fn inject_profit(env: Env, admin: Address, amount: i128) {
        admin.require_auth();

        let metadata: VaultMetadata = env.storage().get(symbol_short!("metadata")).unwrap();
        let token = TokenClient::new(&env, &metadata.token_address);
        token.transfer_from(&admin, &env.current_contract_address(), &amount);

        let current_profit = env
            .storage()
            .get::<Symbol, i128>(symbol_short!("total_profit"))
            .unwrap_or(0);
        env.storage().set(symbol_short!("total_profit"), current_profit + amount);
    }

    /// ✅ L’utilisateur réclame son rendement proportionnel à ses parts
    pub fn claim_rewards(env: Env, user: Address) {
        user.require_auth();

        let metadata: VaultMetadata = env.storage().get(symbol_short!("metadata")).unwrap();
        let balances: Map<Address, i128> = env
            .storage()
            .get(symbol_short!("balances"))
            .unwrap_or(Map::new(&env));

        let user_shares = balances.get(user.clone()).unwrap_or(0);
        if user_shares == 0 {
            panic!("No shares");
        }

        let total_profit = env
            .storage()
            .get::<Symbol, i128>(symbol_short!("total_profit"))
            .unwrap_or(0);

        let mut claimed_map: Map<Address, i128> = env
            .storage()
            .get(symbol_short!("claimed_profit"))
            .unwrap_or(Map::new(&env));

        let already_claimed = claimed_map.get(user.clone()).unwrap_or(0);

        let user_total_earnable = (total_profit * user_shares) / metadata.total_shares;
        let to_claim = user_total_earnable - already_claimed;

        if to_claim <= 0 {
            panic!("Nothing to claim");
        }

        // Payer l'utilisateur
        let token = TokenClient::new(&env, &metadata.token_address);
        token.transfer(&user, &to_claim);

        // Mettre à jour les rendements réclamés
        claimed_map.set(user.clone(), user_total_earnable);
        env.storage().set(symbol_short!("claimed_profit"), claimed_map);
    }

    // Getters (balance, shares, total profit)
    pub fn get_share_balance(env: Env, user: Address) -> i128 {
        let balances: Map<Address, i128> = env
            .storage()
            .get(symbol_short!("balances"))
            .unwrap_or(Map::new(&env));
        balances.get(user).unwrap_or(0)
    }

    pub fn get_total_shares(env: Env) -> i128 {
        let metadata: VaultMetadata = env.storage().get(symbol_short!("metadata")).unwrap();
        metadata.total_shares
    }

    pub fn get_total_profit(env: Env) -> i128 {
        env.storage()
            .get(symbol_short!("total_profit"))
            .unwrap_or(0)
    }

    pub fn get_claimed(env: Env, user: Address) -> i128 {
        let claimed: Map<Address, i128> = env
            .storage()
            .get(symbol_short!("claimed_profit"))
            .unwrap_or(Map::new(&env));
        claimed.get(user).unwrap_or(0)
    }
}