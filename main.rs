mod web;
mod parse;
//use crate::parse::{batch_price, new_ts, prc_fifo};
//use crate::calc::fmt_unix_ts;
//use crate::autotrade::data::;
use rust_decimal::Decimal;
use rust_decimal_macros::*;
use time::*;

fn main() {
//    parse::batch_price();
    web::launch_ws();
//    let ts: time::PrimitiveDateTime = parse::new_ts("btc-usd_r", x);
//    println!("{}", ts);
}

