use starknet::ContractAddress;
use thunder_contract::core::contract1::OrderBook::{Order};
#[starknet::interface]
pub trait IOrderBook<TContractState> {
    fn create_order(ref self: TContractState, caller: ContractAddress, order: Order);
    fn compute_order_hash(ref self: TContractState, order: Order) -> felt252;
    fn get_order_by_id(self: @TContractState, order_id: u256) -> Order;
    fn fulfill_order(ref self: TContractState, order_id: u256);
    // fn buy_order_market(ref self: TContractState);
// fn get_orders_length(self: @TContractState) -> u256;
// fn get_user_order(self: @TContractState, user: ContractAddress, id: u256) -> Order;
// fn exist(self: @TContractState, order_id: u256) -> bool;
}
