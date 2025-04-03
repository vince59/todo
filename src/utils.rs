
use std::env;
use chrono::{DateTime, Local};
pub trait ToStr {
    fn to_str(&self) -> String;
}

impl ToStr for Option<DateTime<Local>> {
    fn to_str(&self) -> String {
        match self {
            Some(date) => date.format("%Y-%m-%d %H:%M:%S").to_string(),
            None => "".to_string(),
        }
    }
}

#[macro_export]
macro_rules! enum_with_strings {
    ($name:ident { $($variant:ident => $string:expr),* $(,)? }) => {
        #[repr(u8)]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
        pub enum $name {
            $($variant),*
        }

        impl ToString for $name {
            fn to_string(&self) -> String {
                match self {
                    $(Self::$variant => $string.to_string()),*
                }
            }
        }

        impl $name {
            pub fn all() -> Vec<($name, String)> {
                vec![
                    $(($name::$variant, $string.to_string())),*
                ]
            }
        }

        impl ToStr for $name {
            fn to_str(&self) -> String {
                (*self as u8).to_string()
            }
        }
    };
}

pub fn print_usage(){
    println!("Usage :");
    println!("todo [-p port]");
    println!("Ex :");
    println!("todo -p 8080");
}

pub fn get_port_from_args() -> Result<u16, String> {
    let default_port = 3000;
    let args: Vec<String> = env::args().collect();
    
    if let Some(pos) = args.iter().position(|arg| arg == "--port" || arg == "-p") {
        if let Some(port_str) = args.get(pos + 1) {
            match port_str.parse::<u16>() {
                Ok(port) if port > 79 => Ok(port),
                Ok(_) => Err("Error : port must be greater than 79.".to_string()),
                Err(_) => Err("Erreur : invalid port.".to_string()),
            }
        } else {
            Err("Erreur : no port given after --port or -p.".to_string())
        }
    } else {
        Ok(default_port)
    }
}