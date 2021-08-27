pub enum OrderType {
    BUY,
    SELL,
}
pub struct SingleOrder {
    participant_code: String,
    price: f32,
    size: i32,
    pub(super) direction: OrderType,
    order_time: i64,
}
