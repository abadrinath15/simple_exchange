use super::order;
use ordered_float;
use std::cmp::{Eq, Ord, PartialEq, PartialOrd, Reverse};
use std::collections::{BinaryHeap, HashMap};
use std::hash;

#[derive(Debug, Clone)]
struct OrderNotInBookError {}

trait SecurityOrderBook<T> {
    /// Adds an order to the book and returns an order ID which can retrieve the order from the same
    /// book.
    /// # Examples
    fn add_order(&mut self, new_order: order::SingleOrder) -> T;

    /// Removes an order from the book as identified by an order number, returning the order if it
    /// is found in the book or an error if not found.
    /// # Examples
    fn remove_order(&mut self, order_number: T) -> Result<order::SingleOrder, OrderNotInBookError>;
}
#[derive(PartialEq, Eq, PartialOrd, Ord, hash::Hash)]
struct OrderTime {
    price: ordered_float::NotNan<f32>,
    time: i32,
}

struct HeadMapBook {
    order_heap: BinaryHeap<Reverse<OrderTime>>,
    order_map: HashMap<OrderTime, order::SingleOrder>,
}

impl SecurityOrderBook<OrderTime> for HeadMapBook {
    fn add_order(&mut self, new_order: order::SingleOrder) -> OrderTime {
        let order_as_ot = OrderTime {
            price: new_order.price,
            time: new_order.order_time,
        };
        self.order_map.insert(order_as_ot, new_order);
        self.order_heap.push(Reverse(order_as_ot));
        return order_as_ot;
    }
}
