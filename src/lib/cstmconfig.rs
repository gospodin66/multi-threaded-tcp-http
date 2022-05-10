#[derive(Debug)]
pub struct ServerConfig {
    pub host: String,
    pub port1: u16,
    pub port2: u16,
    pub request_methods: [&'static str; 4],
    pub connections: Vec<std::net::TcpStream>,
}

pub struct DbConfig {
    pub host: String,
    pub port: u16,
    pub user: String,
    pub password: String,
    pub database: String,
}

pub struct BaseConfig {
    pub http_protocol: String,
}
pub struct AssetsConfig {
    pub html_base_path: String,
    pub log_path: String,
}

pub struct AppConfig {
    pub base: BaseConfig,
    pub server: ServerConfig,
    pub database: DbConfig,
    pub assets: AssetsConfig,
}


#[allow(dead_code)]
impl AppConfig {
    pub fn new_cfg() -> AppConfig {
        AppConfig {
            server: ServerConfig::new_cfg(),
            database: DbConfig::new_cfg(),
            base: BaseConfig::new_cfg(),
            assets: AssetsConfig::new_cfg(),
        }
    }
}


impl BaseConfig {
    pub fn new_cfg() -> BaseConfig {
        match dotenv::dotenv().ok() {
            Some(_envpath) => {},
            None => {
                println!("Error loading env vars!");
                //ret error
            }
        }
        let proto : String = dotenv::var("SERVER.HTTP_PROTOCOL").unwrap();
        BaseConfig {
            http_protocol: proto
        }
    }
}

impl AssetsConfig {
    pub fn new_cfg() -> AssetsConfig {
        match dotenv::dotenv().ok() {
            Some(_envpath) => {},
            None => {
                println!("Error loading env vars!");
                //ret error
            }
        }
        let base_path : String = dotenv::var("APP.HTML_BASE_PATH").unwrap();
        let log_path : String = dotenv::var("APP.LOG_PATH").unwrap();
        AssetsConfig {
            html_base_path: base_path,
            log_path: log_path,
        }
    }
}

impl ServerConfig {
    pub fn new_cfg() -> ServerConfig {
        match dotenv::dotenv().ok() {
            Some(_envpath) => {},
            None => {
                println!("Error loading env vars!");
                //ret error
            }
        }
        let port1_str: String = dotenv::var("SERVER.PORT1").unwrap();
        let port2_str: String = dotenv::var("SERVER.PORT2").unwrap();
        let port1: u16 = port1_str.trim().parse::<u16>().unwrap();
        let port2: u16 = port2_str.trim().parse::<u16>().unwrap();
        ServerConfig {
            host: dotenv::var("SERVER.HOST").unwrap(),
            port1: port1,
            port2: port2,
            request_methods: ["GET","POST","OPTIONS","HEAD"],
            connections: Vec::new(),
        }
    }
}

impl DbConfig {
    pub fn new_cfg() -> DbConfig {
        match dotenv::dotenv().ok() {
            Some(_envpath) => {},
            None => {
                println!("Error loading env vars!");
                //ret error
            }
        }
        let db_port_str : String = dotenv::var("DATABASE.PORT").unwrap();
        let db_port : u16 = db_port_str.trim().parse::<u16>().unwrap();
        DbConfig {
            host: dotenv::var("DATABASE.HOST").unwrap(),
            port: db_port,
            user: dotenv::var("DATABASE.USER").unwrap(),
            password: dotenv::var("DATABASE.PASSWORD").unwrap(),
            database: dotenv::var("DATABASE.DATABASE").unwrap(),
        }
    }
}