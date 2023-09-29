#![macro_export]

#[macro_use]

macro_rules! rd_cmd_re {   
    ($type:path, $cmd:expr, $con:expr)  => {
	{
	    fn gen_cmd(con: redis::Connection, cmd: redis::Cmd) -> redis::RedisResult<$type> {
		let mut conn: redis::Connection = con; 
		let result: $type = cmd.query(&mut conn);
		Ok(result)
	    }
	}
    }
}

#[macro_use]

macro_rules! rd_cmd_no_re {
    ($cmd:expr, $con:expr) => {
	{
	    fn gen_cmd(con: redis::Connection, cmd: redis::Cmd) -> redis::RedisResult<()> {
		let mut conn: redis::Connection = con; 
		let _: redis::RedisResult<()> = cmd.query(&mut conn);
		Ok(())
	    }
	}
    }
}

#[macro_use]

macro_rules! rd_conn {
    () => {
	{
	    fn new_conn() -> redis::RedisResult<redis::Connection> {
		let client = redis::Client::open("redis://127.0.0.1/")?;
		let mut con = client.get_connection()?;
		Ok(con)
	    }
	}
    }
}

#[macro_use]

macro_rules! rd_args {
    ($cmd:expr) => {
	{
	    fn mk_cmd(cmd)
