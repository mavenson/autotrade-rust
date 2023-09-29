use serde::{Serialize, Deserialize};
use serde_json::{json, Result, Value};
use tungstenite::{connect, Message};
use rust_decimal::Decimal;
use std::str::FromStr;
use time::prelude::*;
use redis::Commands;
use crate::calc::dec_sum;
use rust_decimal_macros::*;
use crate::data;

#[derive(Serialize, Deserialize, Debug)]
pub struct TickStruct {
    pub time: String,
    pub price: String,
    pub side: String,
    pub last_size: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ParsedTick {
    pub stream: String,
    pub time: time::PrimitiveDateTime,
    pub side: bool,
    pub price: Decimal,
    pub size: Decimal,
}

pub fn oldest_msg(stream: String, con: redis::Connection) -> redis::RedisResult<ParsedTick> {
    let args: redis::Cmd = crate::data::redis_args(
	&["XRANGE", &stream, "*", "-", "+", "COUNT", "1"]).unwrap();
    let oldest: TickStruct = fmt_tick(rd_cmd_no_re!(args, con));
    let result: redis::RedisResult<ParsedTick> = parse_tick(stream, oldest);
    Ok(result.unwrap())
}

pub fn newest_msg(stream: String, con: redis::Connection) -> redis::RedisResult<ParsedTick> {
    let args: redis::Cmd = crate::data::redis_args(
	&["XREVRANGE", &stream, "*", "-", "+", "COUNT", "1"]).unwrap();
    let newest: TickStruct = fmt_tick(rd_cmd_no_re!(args, con));
    let result: redis::RedisResult<ParsedTick> = parse_tick(stream, newest);
    Ok(result.unwrap())
}

pub fn parse_tick(stream: String, tick: TickStruct) -> redis::RedisResult<ParsedTick> {
    let parsed = ParsedTick{stream: stream,
			     time: fmt_unix_ts(tick.time),
			     side: chk_side(tick.side).unwrap(),
			     price: Decimal::from_str(&tick.price).unwrap(),
			     size: Decimal::from_str(&tick.last_size).unwrap(),
    };
    Ok(parsed)
}

pub fn parsed_to_json(parsed: ParsedTick) -> Result<String> {
    let parsed_json = serde_json::to_string(&parsed).unwrap();
    Ok(parsed_json)
}

pub fn fmt_tick(tick: Message) -> TickStruct {
    let tick_str = Message::to_text(&tick).unwrap();
    let tick_struct: TickStruct = serde_json::from_str(tick_str).unwrap();
    tick_struct
}

pub fn json_to_str(req_msg: String, url: String) -> String {
    let r = serde_json::to_string(&req_msg).unwrap();
    r
}

// pub trait TimeStamp {
//     fn make_ts(&self, known_ts: TsType) -> TimeStruct;
//     fn modify_ts(&self, redis_ts: Time, exch_ts: Time, rust_ts: Time);
//     fn calc_fields(&self);
//     fn redis_to_rust(&self,v) -> Time;
//     fn exch_to_rust(&self,v) -> Time;
//     fn rust_to_redis(&self,v) ->String;
//     fn rust_to_exch(&self,v) -> String;
// }

// pub enum TsType{
// 	Redis(Time),
// 	Exch(Time),
// 	Rust(Time),
//     }
pub struct TimeStruct{
     rd: Option<time::PrimitiveDateTime>,
     exch: Option<time::PrimitiveDateTime>,
}

// impl TimeStamp for TimeStruct {
//     fn modify_ts(&self, redis_ts, exch_ts, rust_ts) {
// 	self.redis_ts = redis_ts;
// 	self.exch_ts = exch_ts;
// 	self.rust_ts= rust_ts;
//     }
//     fn make_ts(&self, known_ts: TsType) -> TimeStruct {
// 	match known_ts {
// 	    TsType::Redis<Time> => modify_ts(Some(known_ts), None, None),
// 	    TsType::Exch<Time> => modify_ts(None, Some(known_ts), None),
// 	    TsType::Rust<Time> => modify_ts(None, None, Some(known_ts)),
// 	}
//     }
//     fn calc_fields(&self) {
// 	match self.redis_ts {
// 	    Some(v) => self.redis_to_rust(v),
// 	    None => None,
// 	}
// 	match self.exch_ts {
// 	    Some(v) => self.exch_to_rust(v),
// 	    None => None,
// 	}
// 	match self.rust_ts {
// 	    Some(v) => self.rust_to_redis(v),
// 	    None => None,
// 	}
// 	match self.redis_ts {
// 	    Some(v) => self.redis_to_rust(v),
// 	    None => None,
// 	}
//     }
// }
	
// pub fn get_ts(cmd: String, stream: String,  start: String, end: String) ->
// 	                                                 Vec<Vec<Vec<String>>> {
// 	redis_cmd_full!(Vec<Vec<Vec<String>>>,cmd, (stream, start, end))
// }

// pub fn get_exch_ts(cmd: &str, args:  &[&str]) -> Vec<Vec<Vec<String>>>{
//     let result = redis_cmd_full!(Vec<Vec<Vec<String>>>, cmd, args).unwrap();
//     result
// }

pub fn get_exch_ts(cmd: &str, args:  &[&str]) -> redis::RedisResult<Vec<Vec<String>>>{
    let result = redis_cmd_full!(Vec<Vec<String>>>, cmd, args);
    Ok(result)
}


// pub fn get_redis_ts(cmd: &str, args:  &[&str]) -> Vec<Vec<String>>{
//     let result = redis_cmd_full!(Vec<Vec<String>>, cmd, args).unwrap();
//     result
// }



//pub fn new_ts(cmd: &str, args: &[&str]) -> time::PrimitiveDateTime {
    
    // let redis_ts = get_redis_ts(cmd, args);
    // let redis_ts_ref: &str = &redis_ts.unwrap()[0];
    // let rd_ts_ob: time::PrimitiveDateTime = fmt_unix_ts(String::from(redis_ts_ref));
    // rd_ts_ob
//}

pub fn chk_side(side: String) -> Option<bool> {
    match &side[..] {
	"buy" => Some(true),
	"sell" => Some(false),
	_ => None,
    }
}
	
    // let exch_ts = get_exch_ts("XRANGE", &["BTC-USD", "-", "+", "COUNT", "1"]);
    // let exch_ts_ref: &str = &exch_ts[0][0][1];
    // let exch_ts_ob: Option<time::PrimitiveDateTime> = Some(fmt_exch_ts(String::from(exch_ts_ref)));
    // match exch_ts_ob{
    // 	redis_ts_ob => true,
    // 	_ => false,
    // }
    // }

pub struct Ymd{
    year: i32,
    month: u8,
    day: u8,
}

pub struct Hms{
    hour: u8,
    minute: u8,
    second: u8,
}
    
pub fn fmt_exch_ts(exch_ts: String) -> time::PrimitiveDateTime {
    let mut dt_arr: Vec<&str> = exch_ts.split("T").collect();
    let ts_time_raw = dt_arr.pop().unwrap();
    let mut ts_arr: Vec<&str> = ts_time_raw.split(".").collect();
    let ms_time = ts_arr.pop().unwrap();
    let mut ms: Vec<&str> = ms_time.split("Z").collect();
    let ms_time = ms.pop().unwrap();
    let ms_time = ms.pop().unwrap();
    let ts_time = ts_arr.pop().unwrap();
    let ts_date = dt_arr.pop().unwrap();
    let mut ymd: Vec<&str> = ts_date.split("-").collect();
    let mut hms: Vec<&str> = ts_time.split(":").collect();
    let date_s = Ymd {year: ymd[0].parse::<i32>().unwrap(),
		      month: ymd[1].parse::<u8>().unwrap(),
		      day: ymd[2].parse::<u8>().unwrap()};
    let time_s = Hms {hour: hms[0].parse::<u8>().unwrap(),
		      minute: hms[1].parse::<u8>().unwrap(),
		      second: hms[2].parse::<u8>().unwrap()};
    let date_ob = time::Date::try_from_ymd(date_s.year,
					   date_s.month, date_s.day).unwrap();
    let time_ob = time::Time::try_from_hms(time_s.hour,
					   time_s.minute, time_s.second).unwrap();
    let dt = time::PrimitiveDateTime::new(date_ob, time_ob);
    dt
}
    
pub fn fmt_unix_ts(milli: String) -> time::PrimitiveDateTime {
    let ms_i64 = milli[0..10].parse::<i64>().unwrap();
    let prim_datetime: time::PrimitiveDateTime = time::PrimitiveDateTime::from_unix_timestamp(ms_i64);
    prim_datetime
}


// pub fn batch_price() -> () {
//     let con = crate::data::redis_con().unwrap();
//     let cmd = crate::data::redis_args(&["XRANGE", "btc-usd_ticker", "-", "+"]).unwrap();
//     let payload: Vec<Vec<Vec<String>>> = redis_cmd!(Vec<Vec<Vec<String>>>, cmd, con);
//     let mut batch_vec: Vec<Decimal> = Vec::new();
//     for e in &payload {
// 	let s: &str = &e[0][1];
// 	let d = Decimal::from_str(&s).unwrap();
// 	batch_vec.push(d);
//     }
//     let total: Decimal = crate::calc::dec_sum(batch_vec);
// 	println!("{}", total);
// }


// pub fn format_ts(packet: Vec<Vec<Vec<String>) {
//     println!("{}", packet);
// }

// pub fn stream_duration(stream: String, start: Time, end: Time) -> Duration {
    


//     // 1589765621447-0 - 1589766128800-0
