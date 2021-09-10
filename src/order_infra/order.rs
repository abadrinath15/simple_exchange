use ordered_float;
use std::error;
use std::fmt;
use std::str;

/// A trade must have a direction (ie buy or sell); this enum present the two options.
#[derive(PartialEq, Debug)]
pub enum OrderType {
    BUY,
    SELL,
}

/// The basic structure of an order, containing fields an order book would require.
#[derive(PartialEq, Debug)]
pub struct SingleOrder {
    pub order_time: i32,
    participant_code: String,
    security_name: String,
    pub price: ordered_float::NotNan<f32>,
    size: i32,
    pub(super) direction: OrderType,
}

/// An error type for parsing the order string, used if the string doesn't contain all the
/// requisite fields for an order.
#[derive(Debug)]
struct ParamMissing {
    param_name: String,
}

impl fmt::Display for ParamMissing {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} is missing", self.param_name)
    }
}

impl error::Error for ParamMissing {}

/// An error if the direction paramter isn't "BUY" or "SELL", since that's really all orders
/// can have...
#[derive(Debug)]
struct InvalidOrderType {
    order_type_str: String,
}

impl fmt::Display for InvalidOrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{} is not an order type; must be BUY OR SELL",
            self.order_type_str
        )
    }
}

impl error::Error for InvalidOrderType {}

#[doc(hidden)]
fn iter_name_error<'a>(
    order_iter: &'a mut str::SplitWhitespace,
    param_name: &str,
) -> Result<&'a str, ParamMissing> {
    order_iter.next().ok_or(ParamMissing {
        param_name: param_name.to_string(),
    })
}
/// Creates a `SingleOrder` from a `String` representation of an order. This requires order fields
/// to be seperated by spaces.
///
/// # Arguments
/// `order_str` - A string that holds the details of the order, with the parameters seperated by
/// spaces.
/// # Examples
/// ```
/// let buy_ord_str = "1 BOFASEC 50.0 100 BUY".to_string();
/// let buy_ord = order_from_string(buy_ord_str).unwrap();
/// ```
/// ```
/// let sell_ord_str = "1 BOFASEC FB 75.0 100 SELL".to_string();
/// let sell_ord = order_from_string(sell_ord_str).unwrap();
/// ```
///
pub fn order_from_string(order_str: String) -> Result<SingleOrder, Box<dyn error::Error>> {
    let mut order_iter = order_str.split_whitespace();
    let order_time = iter_name_error(&mut order_iter, "Order time")?.parse::<i32>()?;
    let part_code = iter_name_error(&mut order_iter, "Participant code")?.to_string();
    let sec_name = iter_name_error(&mut order_iter, "Security name")?.to_string();
    let price =
        ordered_float::NotNan::new(iter_name_error(&mut order_iter, "Price")?.parse::<f32>()?)?;
    let size = iter_name_error(&mut order_iter, "Size")?.parse::<i32>()?;
    let direction_str = iter_name_error(&mut order_iter, "Order type")?;
    let direction = match direction_str {
        "BUY" => Ok(OrderType::BUY),
        "SELL" => Ok(OrderType::SELL),
        _ => Err(InvalidOrderType {
            order_type_str: direction_str.to_string(),
        }),
    }?;
    Ok(SingleOrder {
        order_time: order_time,
        participant_code: part_code,
        security_name: sec_name,
        price: price,
        size: size,
        direction: direction,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn parse_basic_buy_order() {
        let buy_ord_str = "1 BOFASEC AAPL 50.0 100 BUY".to_string();
        let buy_ord_check = SingleOrder {
            order_time: 1,
            direction: OrderType::BUY,
            security_name: "AAPL".to_string(),
            price: ordered_float::NotNan::new_unchecked(50.0),
            size: 100,
            participant_code: "BOFASEC".to_string(),
        };
        let buy_ord = order_from_string(buy_ord_str).unwrap();
        assert_eq!(buy_ord, buy_ord_check)
    }
    #[test]
    fn parse_basic_sell_order() {
        let sell_ord_str = "1 BOFASEC FB 75.0 100 SELL".to_string();
        let sell_ord_check = SingleOrder {
            order_time: 1,
            direction: OrderType::SELL,
            security_name: "FB".to_string(),
            price: ordered_float::NotNan::new_unchecked(75.0),
            size: 100,
            participant_code: "BOFASEC".to_string(),
        };
        let sell_ord = order_from_string(sell_ord_str).unwrap();
        assert_eq!(sell_ord, sell_ord_check)
    }
    #[test]
    fn parse_price_as_int() {
        let sell_ord_str = "1 BOFASEC FB 75 100 SELL".to_string();
        let sell_ord_check = SingleOrder {
            order_time: 1,
            direction: OrderType::SELL,
            security_name: "FB".to_string(),
            price: ordered_float::NotNan::new_unchecked(75.0),
            size: 100,
            participant_code: "BOFASEC".to_string(),
        };
        let sell_ord = order_from_string(sell_ord_str).unwrap();
        assert_eq!(sell_ord, sell_ord_check)
    }
}
