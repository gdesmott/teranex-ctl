#[macro_use]
extern crate failure;
extern crate structopt;
#[macro_use]
extern crate log;
extern crate env_logger;

use failure::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;
use structopt::StructOpt;

const PORT: u16 = 9800;

#[derive(StructOpt, Debug)]
#[structopt(name = "teranex-ctl")]
struct Opt {
    #[structopt(short = "h", long = "host", help = "Teranex address")]
    host: String,
}

#[derive(Debug)]
struct Connection {
    stream: TcpStream,
}

impl Connection {
    fn new(host: &str) -> Result<Connection, Error> {
        info!("Connecting to {}", host);
        let stream = TcpStream::connect((host, PORT))?;
        stream.set_read_timeout(Some(Duration::from_millis(2000)))?;

        Ok(Connection { stream })
    }

    fn read(&mut self) -> Result<String, Error> {
        let mut buf = String::new();

        let res = self.stream.read_to_string(&mut buf);
        match res {
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                trace!("read:\n{}", buf);
                Ok(buf)
            }
            Err(e) => Err(format_err!("Reading failed: {}", e)),
            _ => Err(format_err!("Connection dropped")),
        }
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let opt = Opt::from_args();

    let mut conn = Connection::new(&opt.host)?;
    // Need to read the dump from Teranex before being able to send commands
    conn.read()?;

    Ok(())
}
