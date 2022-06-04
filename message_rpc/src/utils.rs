use std::env;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

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
    fn test_service_port_is_string() {
        temp_env::with_var::<&str, &str, _>("PIPE_PORT", Some("AAAA"), || {
            super::get_service_port();
        });
    }

    #[test]
    #[should_panic]
    fn test_service_port_is_not_u16() {
        temp_env::with_var::<&str, &str, _>("PIPE_PORT", Some("65537"), || {
            super::get_service_port();
        });
    }

    #[test]
    fn test_service_port_is_ok() {
        temp_env::with_var::<&str, &str, _>("PIPE_PORT", Some("65535"), || {
            assert_eq!(super::get_service_port(), 65535);
        });
    }

    #[test]
    #[should_panic]
    fn test_discord_bot_token_not_set() {
        temp_env::with_var::<&str, &str, _>("DISCORD_BOT_TOKEN", None, || {
            super::get_discord_bot_token();
        });
    }

    #[test]
    fn test_discord_bot_token_ok() {
        temp_env::with_var::<&str, &str, _>("DISCORD_BOT_TOKEN", Some("ABC"), || {
            assert_eq!(super::get_discord_bot_token(), "ABC".to_string());
        });
    }

    #[test]
    #[should_panic]
    fn test_channel_id_is_string() {
        temp_env::with_var::<&str, &str, _>("DISCORD_CHANNEL_ID", None, || {
            super::get_channel_id();
        });
    }

    #[test]
    fn test_channel_id_is_ok() {
        temp_env::with_var::<&str, &str, _>("DISCORD_CHANNEL_ID", Some("1234567890"), || {
            assert_eq!(super::get_channel_id(), 1234567890);
        });
    }
}