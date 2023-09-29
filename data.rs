use redis::Commands;
use tungstenite::{connect, Message};
use crate::parse::{ParsedTick, TickStruct, fmt_tick, oldest_msg, newest_msg, parse_tick};


pub fn prc_fifo(stream: String) -> redis::RedisResult<()> {
    let mut con = redis_con().unwrap();
    let mut oldest = crate::parse::oldest_msg(stream, con).unwrap();
    let mut newest = crate::parse::newest_msg(stream, con).unwrap();
    while &newest.time > &oldest.time {
	let mut newest = crate::parse::newest_msg(stream, con);
	let mut parsed_t = serde_json::to_string(&newest).unwrap();
	let cmd: redis::Cmd = redis_args(&[]).unwrap();
	let _: () = rd_cmd_no_re!(cmd, con);
	println!("{:?}", newest);
    }
    Ok(())
}


