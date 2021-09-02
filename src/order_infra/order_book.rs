use super::order;

#[derive(Debug, Clone)]
struct OrderNotInBookError {}

trait SecurityOrderBook<T> {
    /// Adds an order to the book and returns an order ID which can retrieve the order from the same
    /// book.
    /// # Examples
    fn add_order(&mut self, new_order: &order::SingleOrder) -> T;

    /// Removes an order from the book as identified by an order number, returning the order if it
    /// is found in the book or an error if not found.
    /// # Examples
    fn remove_order(&mut self, order_number: T) -> Result<order::SingleOrder, OrderNotInBookError>;
}
