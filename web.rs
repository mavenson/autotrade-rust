use crate::parse::{TickStruct, json_to_str, newest_msg, oldest_msg, parse_tick, fmt_tick};
use rust_decimal::Decimal; 
use rust_decimal_macros::*; 
use tungstenite::{Message, connect};
use url::Url;
use redis::Commands;
use std::fmt::Debug;
use std::ops::Deref;

pub fn launch_ws() -> () {
    let request_message = String::from(r#"
{
"type": "subscribe",
"product_ids": ["BTC-USD"],
"channels": ["ticker"]
}"#);

    
    let (mut socket, _) =
	connect(Url::parse("wss://ws-feed.pro.coinbase.com").unwrap()).
	expect("Can't connect");
    socket.write_message(Message::Text(request_message.into())).unwrap();

 	let pl = socket.read_message().unwrap();
    let pl = socket.read_message().unwrap();

    let  con:  redis::Connection =  crate::data::redis_con().unwrap();
    crate::data::prc_fifo(String::from("btc-usd_r"));
    
    loop{
	let pl = socket.read_message().unwrap();
	let con: redis::Connection = rd_conn!().unwrap();
	let pl_str: TickStruct = crate::parse::fmt_tick(pl);
	let add_raw: redis::Cmd = crate::data::redis_args(&[
	    "XADD", "btc-usd_r",
	    "*", "time", &pl_str.time,
	    "price", &pl_str.price,
	    "side", &pl_str.side,
	    "size",  &pl_str.last_size]).unwrap();
	rd_cmd_no_re!(add_raw, con);
	
	println!("{} {} {} {}", &pl_str.time, &pl_str.price, &pl_str.side, &pl_str.last_size);
    }
    socket.close(None);
}
