use clap::ArgMatches;
use std::net::IpAddr;
use std::str::FromStr;

#[derive(Debug)]
pub struct Opts {
    pub n: u16,
    pub ip: IpAddr,
    pub lower: u16,
    pub upper: u16,
}

impl Opts {
    pub fn from(cmd: &ArgMatches) -> Result<Opts, &'static str> {
        let threads = cmd
            .value_of("v")
            .unwrap_or("1000")
            .to_string()
            .parse::<u16>()
            .unwrap();
        let ip_str = cmd.value_of("IP").unwrap();
        let lower = cmd
            .value_of("min")
            .unwrap_or("0")
            .to_string()
            .parse::<u16>()
            .unwrap();
        let uppser = cmd
            .value_of("max")
            .unwrap_or("65535")
            .to_string()
            .parse::<u16>()
            .unwrap();

        if let Ok(ip_addr) = IpAddr::from_str(&ip_str) {
            Ok(Opts {
                n: threads,
                ip: ip_addr,
                lower: lower,
                upper: uppser,
            })
        } else {
            Err("Invalid IPv4 Address entered")
        }
    }
}
