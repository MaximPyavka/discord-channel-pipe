use std::env;

use std::net::{SocketAddr, Ipv4Addr, IpAddr};

use tonic::transport::Endpoint;

pub fn get_service_port() -> u16 {
    env::var("PIPE_PORT")
    .expect("Please provide TCP port for Discord Pipe service.")
    .parse()
    .expect("Please provide valid registered port number.")
}

pub fn get_service_socket() -> SocketAddr {
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), get_service_port())
}

pub fn get_client_endpoint() -> Endpoint {
    Endpoint::from_shared(format!("http://{:?}", get_service_socket())).unwrap()
}

pub fn get_channel_id() -> u64 {
    env::var("DISCORD_CHANNEL_ID")
    .expect("Please specify DISCORD_CHANNEL_ID environment variable.")
    .parse()
    .expect("Please provide DISCORD_CHANNEL_ID as u64.")
}

pub fn get_discord_bot_token() -> String {
    env::var("DISCORD_BOT_TOKEN").expect("Please specify env variable DISCORD_BOT_TOKEN")
}


#[cfg(test)]
mod tests {

    #[test]
    #[should_panic]
    fn test_service_port_not_set() {
        // super::env::set_var()
        super::get_service_port();
    }

    #[test]
    #[should_panic]
    fn test_service_port_is_string() {
        super::env::set_var("PIPE_PORT", "AAAAA");
        super::get_service_port();
    }

    #[test]
    #[should_panic]
    fn test_service_port_is_not_u16() {
        super::env::set_var("PIPE_PORT", "65537");
        super::get_service_port();
    }

    #[test]
    fn test_service_port_is_ok() {
        super::env::set_var("PIPE_PORT", "65535");
        let port = super::get_service_port();
        assert_eq!(port, 65535)
    }

}