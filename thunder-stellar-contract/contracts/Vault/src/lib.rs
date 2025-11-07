#![no_std]

use soroban_sdk::{contractimpl,contract, contracttype, symbol_short, Address, Env, Symbol, Map};
use soroban_sdk::token::Client as TokenClient;


#[contracttype]
#[derive(Clone)]
pub struct VaultMetadata {
    pub token_address: Address,
    pub total_shares: i128,
}

#[contractimpl]
impl Vault {
    /// Initialise le Vault avec le token utilisé
    pub fn init(env: Env, token_address: Address) {
        let metadata = VaultMetadata {
            token_address,
            total_shares: 0,
        };

        env.storage().persistent().set(&symbol_short!("meta"), &metadata);
        env.storage().persistent().set(&symbol_short!("bal"), &Map::<Address, i128>::new(&env));
        env.storage().persistent().set(&symbol_short!("claimed"), &Map::<Address, i128>::new(&env));
        env.storage().persistent().set(&symbol_short!("profit"), &0i128);
    }

    /// Dépôt dans le vault — prend les tokens de l’utilisateur
    pub fn deposit(env: Env, from: Address, amount: i128) {
        from.require_auth();

        let mut metadata: VaultMetadata = env
            .storage()
            .persistent()
            .get(&symbol_short!("meta"))
            .unwrap();

        let mut balances: Map<Address, i128> = env
            .storage()
            .persistent()
            .get(&symbol_short!("bal"))
            .unwrap_or(Map::new(&env));

        let token = TokenClient::new(&env, &metadata.token_address);

        // 🔹 Prendre les tokens du compte utilisateur vers le contrat
        token.transfer_from(&from, &from, &env.current_contract_address(), &amount);

        // 🔹 Met à jour la balance de l’utilisateur
        let prev_balance = balances.get(from.clone()).unwrap_or(0);
        balances.set(from.clone(), prev_balance + amount);
        metadata.total_shares += amount;

        // 🔹 Sauvegarde
        env.storage().persistent().set(&symbol_short!("bal"), &balances);
        env.storage().persistent().set(&symbol_short!("meta"), &metadata);
    }

    /// Retrait du vault — brûle les parts et renvoie les tokens
    pub fn withdraw(env: Env, to: Address, share_amount: i128) {
        to.require_auth();

        let mut metadata: VaultMetadata = env
            .storage()
            .persistent()
            .get(&symbol_short!("meta"))
            .unwrap();

        let mut balances: Map<Address, i128> = env
            .storage()
            .persistent()
            .get(&symbol_short!("bal"))
            .unwrap_or(Map::new(&env));

        let user_balance = balances.get(to.clone()).unwrap_or(0);
        if user_balance < share_amount {
            panic!("Not enough shares");
        }

        balances.set(to.clone(), user_balance - share_amount);
        metadata.total_shares -= share_amount;

        let token = TokenClient::new(&env, &metadata.token_address);

        // 🔹 Envoie les tokens à l’utilisateur
        token.transfer(&env.current_contract_address(), &to, &share_amount);

        env.storage().persistent().set(&symbol_short!("bal"), &balances);
        env.storage().persistent().set(&symbol_short!("meta"), &metadata);
    }

    /// L’admin injecte un rendement dans le vault
    pub fn inject_profit(env: Env, admin: Address, amount: i128) {
        admin.require_auth();

        let metadata: VaultMetadata = env
            .storage()
            .persistent()
            .get(&symbol_short!("meta"))
            .unwrap();

        let token = TokenClient::new(&env, &metadata.token_address);

        // 🔹 L’admin transfère ses tokens vers le contrat
        token.transfer_from(&admin, &admin, &env.current_contract_address(), &amount);

        // 🔹 Mise à jour du profit total
        let current_profit = env
            .storage()
            .persistent()
            .get::<Symbol, i128>(&symbol_short!("profit"))
            .unwrap_or(0);

        env.storage()
            .persistent()
            .set(&symbol_short!("profit"), &(current_profit + amount));
    }

    /// L’utilisateur réclame sa part du rendement
    pub fn claim_rewards(env: Env, user: Address) {
        user.require_auth();

        let metadata: VaultMetadata = env
            .storage()
            .persistent()
            .get(&symbol_short!("meta"))
            .unwrap();

        let balances: Map<Address, i128> = env
            .storage()
            .persistent()
            .get(&symbol_short!("bal"))
            .unwrap_or(Map::new(&env));

        let user_shares = balances.get(user.clone()).unwrap_or(0);
        if user_shares == 0 {
            panic!("No shares");
        }

        let total_profit = env
            .storage()
            .persistent()
            .get::<Symbol, i128>(&symbol_short!("profit"))
            .unwrap_or(0);

        let mut claimed_map: Map<Address, i128> = env
            .storage()
            .persistent()
            .get(&symbol_short!("claimed"))
            .unwrap_or(Map::new(&env));

        let already_claimed = claimed_map.get(user.clone()).unwrap_or(0);

        let user_total_earnable = (total_profit * user_shares) / metadata.total_shares;
        let to_claim = user_total_earnable - already_claimed;

        if to_claim <= 0 {
            panic!("Nothing to claim");
        }

        let token = TokenClient::new(&env, &metadata.token_address);

        // 🔹 Versement de la récompense à l’utilisateur
        token.transfer(&env.current_contract_address(), &user, &to_claim);

        // 🔹 Met à jour les profits réclamés
        claimed_map.set(user.clone(), user_total_earnable);
        env.storage().persistent().set(&symbol_short!("claimed"), &claimed_map);
    }

    /// Getter: balance en parts de l’utilisateur
    pub fn get_share_balance(env: Env, user: Address) -> i128 {
        let balances: Map<Address, i128> = env
            .storage()
            .persistent()
            .get(&symbol_short!("bal"))
            .unwrap_or(Map::new(&env));
        balances.get(user).unwrap_or(0)
    }

    /// Getter: total des parts du vault
    pub fn get_total_shares(env: Env) -> i128 {
        let metadata: VaultMetadata = env
            .storage()
            .persistent()
            .get(&symbol_short!("meta"))
            .unwrap();
        metadata.total_shares
    }

    /// Getter: profit total
    pub fn get_total_profit(env: Env) -> i128 {
        env.storage()
            .persistent()
            .get::<Symbol, i128>(&symbol_short!("profit"))
            .unwrap_or(0)
    }

    /// Getter: montant déjà réclamé par un utilisateur
    pub fn get_claimed(env: Env, user: Address) -> i128 {
        let claimed: Map<Address, i128> = env
            .storage()
            .persistent()
            .get(&symbol_short!("claimed"))
            .unwrap_or(Map::new(&env));
        claimed.get(user).unwrap_or(0)
    }
}

#[contract]
pub struct Vault;
