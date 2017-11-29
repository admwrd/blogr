use rocket::{Data, Request, Response};
use std::str::FromStr;
use rocket::fairing::{Fairing, Info, Kind};
use std::net::{IpAddr, SocketAddr};

// https://github.com/lukaspustina/ifconfig-rs/blob/master/src/fairings.rs

#[derive(Default)]
pub struct IpLog {
    pub visitors: 
}

impl Fairing for IpLog {
    fn info(&self) -> Info {
        Info {
            name: "Log the client's ip address",
            kind: Kind::Request | Kind::Response,
        }
    }

    fn on_request(&self, request: &mut Request, _: &Data) {
        let new_remote = if let Some(xfr) = request.headers().get_one("X-Forwarded-For") {
            if let Some(remote) = request.remote() {
                if let Ok(ip) = IpAddr::from_str(xfr) {
                    Some(SocketAddr::new(ip, remote.port()))
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };
        if let Some(remote) = new_remote {
            request.set_remote(remote);
        }
    }

    fn on_response(&self, _: &Request, _: &mut Response) {
        return;
    }
}