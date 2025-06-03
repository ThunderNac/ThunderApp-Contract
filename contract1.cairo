#[starknet::contract]
mod Contarct1 {
    use starknet::storage::{Map,StoragePathEntry,StoragePointerReadAccess,StoragePointerWriteAccess};
    #[storage]
    struct Storage {
    }
    #[event]
    #[derive(Drop, starknet::Event)]
    enum Event {
    }

    #[abi(embed_v0)]
}
