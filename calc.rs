use rust_decimal::Decimal;
use rust_decimal_macros::*;
use std::str::FromStr;
use std::iter::Sum;
use crate::data::{redis_args, redis_con};
use time::prelude::*;


pub fn calc_profit(initial: Decimal, current: Decimal) -> () {
    let r = (current - initial, (current - initial) / initial);
}


pub fn dec_sum(batch: Vec<Decimal>) -> Decimal {
    let batch_iter = batch.iter();
    let mut total: Decimal = dec!(0);
    for e in batch_iter {
	total += e;
    }
    total
}

// Calculate Average 

