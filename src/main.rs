#[macro_use]
extern crate failure;
extern crate structopt;
#[macro_use]
extern crate log;
extern crate env_logger;

use failure::Error;
use std::io::prelude::*;
use std::net::TcpStream;
use std::str::FromStr;
use std::time::Duration;
use structopt::StructOpt;

const PORT: u16 = 9800;

/* See 'Format Conversion Tables' in Teranex Manual */
#[derive(Debug, Clone, PartialEq)]
#[allow(clippy::enum_variant_names)]
enum VideoMode {
    Mode525i59_94,
    Mode625i50,
    Mode720p50,
    Mode720p59_94,
    Mode720p60,
    Mode1080p23_98,
    Mode1080PsF23_98,
    Mode1080p24,
    Mode1080PsF24,
    Mode1080p25,
    Mode1080PsF25,
    Mode1080p29_97,
    Mode1080PsF29_97,
    Mode1080p30,
    Mode1080PsF30,
    Mode1080i50,
    Mode1080p50,
    Mode1080i59_94,
    Mode1080p59_94,
    Mode1080i60,
    Mode1080p60,
    Mode2KDCI23_98p,
    Mode2KDCI23_98PsF,
    Mode2KDCI24p,
    Mode2KDCI24PsF,
    Mode2160p23_98,
    Mode2160p24,
    Mode2160p25,
    Mode2160p29_97,
    Mode2160p30,
    Mode2160p50,
    Mode2160p59_94,
    Mode2160p60,
}

static VIDEO_MODES: &'static [VideoMode] = &[
    VideoMode::Mode525i59_94,
    VideoMode::Mode625i50,
    VideoMode::Mode720p50,
    VideoMode::Mode720p59_94,
    VideoMode::Mode720p60,
    VideoMode::Mode1080p23_98,
    VideoMode::Mode1080PsF23_98,
    VideoMode::Mode1080p24,
    VideoMode::Mode1080PsF24,
    VideoMode::Mode1080p25,
    VideoMode::Mode1080PsF25,
    VideoMode::Mode1080p29_97,
    VideoMode::Mode1080PsF29_97,
    VideoMode::Mode1080p30,
    VideoMode::Mode1080PsF30,
    VideoMode::Mode1080i50,
    VideoMode::Mode1080p50,
    VideoMode::Mode1080i59_94,
    VideoMode::Mode1080p59_94,
    VideoMode::Mode1080i60,
    VideoMode::Mode1080p60,
    VideoMode::Mode2KDCI23_98p,
    VideoMode::Mode2KDCI23_98PsF,
    VideoMode::Mode2KDCI24p,
    VideoMode::Mode2KDCI24PsF,
    VideoMode::Mode2160p23_98,
    VideoMode::Mode2160p24,
    VideoMode::Mode2160p25,
    VideoMode::Mode2160p29_97,
    VideoMode::Mode2160p30,
    VideoMode::Mode2160p50,
    VideoMode::Mode2160p59_94,
    VideoMode::Mode2160p60,
];

impl VideoMode {
    fn variants() -> [&'static str; 33] {
        let mut array = [""; 33];
        VIDEO_MODES.iter().enumerate().for_each(|(i, mode)| {
            array[i] = mode.name().unwrap();
        });
        array
    }

    fn name(&self) -> Result<&'static str, Error> {
        match self {
            VideoMode::Mode525i59_94 => Ok("525i59.94_NTSC"),
            VideoMode::Mode625i50 => Ok("625i50"),
            VideoMode::Mode720p50 => Ok("720p50"),
            VideoMode::Mode720p59_94 => Ok("720p59.94"),
            VideoMode::Mode720p60 => Ok("720p60"),
            VideoMode::Mode1080p23_98 => Ok("1080p23.98"),
            VideoMode::Mode1080PsF23_98 => Ok("1080PsF23.98"),
            VideoMode::Mode1080p24 => Ok("1080p24"),
            VideoMode::Mode1080PsF24 => Ok("1080PsF24"),
            VideoMode::Mode1080p25 => Ok("1080p25"),
            VideoMode::Mode1080PsF25 => Ok("1080PsF25"),
            VideoMode::Mode1080p29_97 => Ok("1080p29.97"),
            VideoMode::Mode1080PsF29_97 => Ok("1080PsF29.97"),
            VideoMode::Mode1080p30 => Ok("1080p30"),
            VideoMode::Mode1080PsF30 => Ok("1080PsF30"),
            VideoMode::Mode1080i50 => Ok("1080i50"),
            VideoMode::Mode1080p50 => Ok("1080p50"),
            VideoMode::Mode1080i59_94 => Ok("1080i59.94"),
            VideoMode::Mode1080p59_94 => Ok("1080p59.94"),
            VideoMode::Mode1080i60 => Ok("1080i60"),
            VideoMode::Mode1080p60 => Ok("1080p60"),
            VideoMode::Mode2KDCI23_98p => Ok("2K-DCI-23.98p"),
            VideoMode::Mode2KDCI23_98PsF => Ok("2K-DCI-23.98PsF"),
            VideoMode::Mode2KDCI24p => Ok("2K-DCI-24p"),
            VideoMode::Mode2KDCI24PsF => Ok("2K-DCI-24PsF"),
            VideoMode::Mode2160p23_98 => Ok("2160p23.98"),
            VideoMode::Mode2160p24 => Ok("2160p24"),
            VideoMode::Mode2160p25 => Ok("2160p25"),
            VideoMode::Mode2160p29_97 => Ok("2160p29.97"),
            VideoMode::Mode2160p30 => Ok("2160p30"),
            VideoMode::Mode2160p50 => Ok("2160p50"),
            VideoMode::Mode2160p59_94 => Ok("2160p59.94"),
            VideoMode::Mode2160p60 => Ok("2160p60"),
        }
    }

    fn protocol_name(&self) -> Result<String, Error> {
        Ok(str::replace(self.name()?, "_", " "))
    }
}

impl FromStr for VideoMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match VIDEO_MODES.iter().find(|mode| mode.name().unwrap() == s) {
            Some(mode) => Ok((*mode).clone()),
            _ => Err(String::from("[invalid value]")),
        }
    }
}

#[derive(StructOpt, Debug)]
#[structopt(name = "command")]
enum Command {
    #[structopt(name = "set-video-mode")]
    SetVideoMode {
        #[structopt(raw(possible_values = "&VideoMode::variants()", case_insensitive = "true"))]
        mode: VideoMode,
    },
}

#[derive(StructOpt, Debug)]
#[structopt(name = "teranex-ctl")]
struct Opt {
    #[structopt(short = "h", long = "host", help = "Teranex address")]
    host: String,
    #[structopt(subcommand)]
    cmd: Command,
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

    fn write(&mut self, buf: &str) -> Result<(), Error> {
        trace!("write: {}", buf);
        writeln!(self.stream, "{}", buf)?;
        Ok(())
    }

    fn check_reply(&mut self) -> Result<(), Error> {
        if self.read()?.starts_with("ACK\n") {
            Ok(())
        } else {
            bail!("Request failed")
        }
    }

    fn set_video_mode(&mut self, mode: &VideoMode) -> Result<(), Error> {
        let name = mode.protocol_name()?;
        info!("Setting mode to {}", name);
        let video_mode = format!("Video mode: {}", name);
        self.write("VIDEO OUTPUT:")?;
        self.write(&video_mode)?;
        self.write("")?;

        self.check_reply()
    }
}

fn main() -> Result<(), Error> {
    env_logger::init();
    let opt = Opt::from_args();

    let mut conn = Connection::new(&opt.host)?;
    // Need to read the dump from Teranex before being able to send commands
    conn.read()?;

    match opt.cmd {
        Command::SetVideoMode { mode } => conn.set_video_mode(&mode)?,
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[test]
    fn set_video_mode() {
        env_logger::init();

        let host = env::var("TERANEX_HOST").expect("define TERANEXT_HOST to run tests");
        let mut conn = Connection::new(&host).unwrap();
        conn.read().unwrap();

        VIDEO_MODES
            .iter()
            .for_each(|mode| conn.set_video_mode(&mode).unwrap());
    }
}
