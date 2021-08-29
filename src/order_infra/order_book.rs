use super::order;

#[derive(Debug, Clone)]
struct OrderNotInBookError {}

trait SecurityOrderBook<T> {
    fn add_order(new_order: order::SingleOrder) -> T;
    //fn cancel_order(order_ref: T) -> Result((), )
}
