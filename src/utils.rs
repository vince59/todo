use std::env;

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