#[starknet::contract]
pub mod OrderBook {
    use core::hash::{HashStateExTrait, HashStateTrait};
    use core::num::traits::{Bounded, Zero};
    use core::{poseidon::PoseidonTrait};
    use openzeppelin_token::erc20::interface::IERC20;
    use starknet::storage::{
        Map, StoragePathEntry, StoragePointerReadAccess, StoragePointerWriteAccess,
    };
    use core::dict::Felt252Dict;
    use starknet::{ContractAddress, get_caller_address};

    #[storage]
    pub struct Storage {
        next_order_id: u256,
        order_list: Map::<u256, Order>,
        user_orders: Map::<(ContractAddress, u256), Order> //UserAddr + id => Order

    }

    #[event]
    #[derive(Copy, Drop, starknet::Event)]
    pub enum Event {
        OrderListed: OrderListed,
        OrderFullFill: OrderFullFill,
    }

    #[derive(Copy, Drop, starknet::Event)]
    pub struct OrderListed {
        #[key]
        order_id: u256,
        sender: ContractAddress,
    }
    #[derive(Copy, Drop, starknet::Event)]
    pub struct OrderFullFill {
        #[key]
        order_id: u256,
        buyer: ContractAddress,
    }

    #[derive(Copy, Drop, Serde, Hash, starknet::Store)]
    pub struct Order {
        id: u256,
        pub seller: ContractAddress,
        pub amount: u256,
        pub apy: u256,
        pub deadline: u256,
        pub fulfill: bool,
        // price: u256,// duration: u256,
    }

    #[constructor]
    fn constructor(ref self: ContractState) {}

    #[abi(embed_v0)]
    impl OrderBookImpl of thunder_contract::interfaces::contract1::IOrderBook<ContractState> {
        fn create_order(ref self: ContractState, caller: ContractAddress, order: Order) {
            let UserOrder = Order {
                id: self.next_order_id.read(),
                seller: caller,
                amount: order.amount,
                apy: order.apy,
                deadline: order.deadline,
                fulfill: false,
            };

            self.order_list.entry(self.next_order_id.read()).write(UserOrder);
            self.user_orders.entry((caller, self.next_order_id.read())).write(UserOrder);

            self.emit(OrderListed { order_id: self.next_order_id.read(), sender: caller });
            self.next_order_id.write(self.next_order_id.read() + 1);
        }

        fn fulfill_order(ref self: ContractState, order_id: u256) {
            let user_order = self.order_list.entry(order_id).read();
            let UserOrder = Order {
                id: order_id,
                seller: user_order.seller,
                amount: user_order.amount,
                apy: user_order.apy,
                deadline: user_order.deadline,
                fulfill: true,
            };

            self.order_list.entry(order_id).write(UserOrder);
        }


        fn compute_order_hash(ref self: ContractState, order: Order) -> felt252 {
            let poseidon_hash = PoseidonTrait::new().update_with(order).finalize();
            return poseidon_hash;
        }

        fn get_order_by_id(self: @ContractState, order_id: u256) -> Order {
            self.order_list.entry(order_id).read()
        }
    }
}
