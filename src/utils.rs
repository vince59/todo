use std::env;
use chrono::NaiveDate;
use minijinja::{value::Value, Error};

// Macro permettant de déclarer un enum automatiquement et d'implémenter les traits pour sql
// et pour avoir un texte associé à l'enum

#[macro_export]
macro_rules! enum_with_strings {
    ($name:ident { $($variant:ident => $string:expr),* $(,)? }) => {
        #[repr(u8)]
        #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize)]
        pub enum $name {
            $($variant),*
        }

        // retourne le texte associé à la valeur de l'enum (pour l'affichage dans les vues)
        impl ToString for $name {
            fn to_string(&self) -> String {
                match self {
                    $(Self::$variant => $string.to_string()),*
                }
            }
        }

        // converti la valeur entière de sql dans la bonne occurence de de l'enum 
        impl FromSql for $name {
            fn column_result(value: ValueRef<'_>) -> Result<$name, FromSqlError> {
                match value.as_i64()? as u8 {
                    $(
                        x if x == $name::$variant as u8 => Ok($name::$variant),
                    )*
                    _ => Err(FromSqlError::Other(
                        format!("Invalid {} value", stringify!($name)).into()
                    )),
                }
            }
        }

        // Ajoute une fonction qui retourne un vecteur avec pour chaque occurence de l'enum un tuple
        // comprenant l'occurence de l'enum et son texte (pour affichage dans les vues)
        impl $name {
            #[allow(dead_code)]
            pub fn all() -> Vec<($name, String)> {
                vec![
                    $(($name::$variant, $string.to_string())),*
                ]
            }
        }

        impl ToSql for $name {
            fn to_sql(&self) -> Result<ToSqlOutput<'_>> {
                Ok(ToSqlOutput::from(*self as u8))
            }
        }
    };
}

pub fn parse_optional_date(s: &str) -> Result<Option<NaiveDate>, chrono::ParseError> {
    if s.trim().is_empty() {
        Ok(None)
    } else {
        Ok(Some(NaiveDate::parse_from_str(s, "%Y-%m-%d")?))
    }
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

// filtre de template pour afficher les dates en jj/mm/aa dans la template en laissant le format AAA-MM-JJ dans la bdd

pub fn format_date(value: Value) -> Result<Value, Error> {
    if let Some(date_str) = value.as_str() {
        let date_opt = NaiveDate::parse_from_str(date_str, "%Y-%m-%d").ok();
        match date_opt {
            Some(date) => Ok(Value::from(date.format("%d/%m/%y").to_string())),
            None => Ok(Value::from("")),
        }
    } else {
        Ok(Value::from(""))
    }
}