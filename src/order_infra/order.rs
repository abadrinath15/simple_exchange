use std::error;
use std::fmt;
use std::str;

/// A trade must have a direction (ie buy or sell); this enum present the two options.
pub enum OrderType {
    BUY,
    SELL,
}

///
pub struct SingleOrder {
    order_time: i32,
    participant_code: String,
    price: f32,
    size: i32,
    pub(super) direction: OrderType,
}

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

#[derive(Debug)]
struct InvalidOrderType<'a> {
    order_type_str: &'a str,
}

impl fmt::Display for InvalidOrderType<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} is not an order type; must be BUY OR SELL",
            self.order_type_str
        )
    }
}

impl error::Error for InvalidOrderType<'_> {}

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
/// # Examples
///
pub fn order_from_string(order_str: &'static String) -> Result<SingleOrder, Box<dyn error::Error>> {
    let mut order_iter = order_str.split_whitespace();
    let order_time = order_iter
        .next()
        .ok_or(ParamMissing {
            param_name: "Order time".to_string(),
        })?
        .parse::<i32>()?;
    let part_code = order_iter
        .next()
        .ok_or(ParamMissing {
            param_name: "Participant code".to_string(),
        })?
        .to_string();
    let price = order_iter
        .next()
        .ok_or(ParamMissing {
            param_name: "Price".to_string(),
        })?
        .parse::<f32>()?;
    let size = order_iter
        .next()
        .ok_or(ParamMissing {
            param_name: "Size".to_string(),
        })?
        .parse::<i32>()?;

    let direction_str = order_iter.next().ok_or(ParamMissing {
        param_name: "Order type".to_string(),
    })?;
    let direction = match direction_str {
        "BUY" => Ok(OrderType::BUY),
        "SELL" => Ok(OrderType::SELL),
        _ => Err(InvalidOrderType {
            order_type_str: direction_str,
        }),
    }?;
    Ok(SingleOrder {
        order_time: order_time,
        participant_code: part_code,
        price: price,
        size: size,
        direction: direction,
    })
}
