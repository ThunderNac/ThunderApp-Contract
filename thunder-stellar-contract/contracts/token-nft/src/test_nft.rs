#[cfg(test)]
mod test {
    use soroban_sdk::{Env, Address, Bytes, testutils::Address as _};
    use crate::{NFTContract, NFTContractClient};

    #[test]
    fn test_mint_and_uri() {
        let env = Env::default();

        // Étape 1: déployer le contrat
        let contract_id = env.register_contract(None, NFTContract);

        // Étape 2: créer le client du contrat
        let client = NFTContractClient::new(&env, &contract_id);

        // Étape 3: préparer les valeurs
        let user = Address::generate(&env);
        let token_id = 42;
        let uri = Bytes::from_slice(&env, b"https://example.com/nft/42");

        // Étape 4: appeler les fonctions via le client
        client.mint(&user, &token_id, &uri);

        let fetched_owner = client.owner_of(&token_id);
        let fetched_uri = client.token_uri(&token_id);

        // Étape 5: assertions
        assert_eq!(fetched_owner, user);
        assert_eq!(fetched_uri, uri);
    }
}