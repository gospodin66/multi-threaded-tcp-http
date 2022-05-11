use mysql::*;
use mysql::prelude::*;
use chrono::{Local, NaiveDateTime};
use crate::lib::cstmconfig;

//#[derive(Queryable)]
#[derive(Debug, Clone)]
pub struct User {
    role_id: u64,
    username: String,
    email: String,
    password: String,
    config: String,
    active: bool,
    remember_token: String,
    avatar: String,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

//#[derive(Queryable)]
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Token {
    user_id: u64,
    token_type: String,
    access_token: String,
    refresh_token: String,
    token_expire: NaiveDateTime,
    created_at: NaiveDateTime,
    updated_at: NaiveDateTime
}

fn init_db() -> Result<Conn> {
    let db_config : cstmconfig::DbConfig = cstmconfig::DbConfig::new_cfg();
    let opts = OptsBuilder::new()
                .ip_or_hostname(Some(db_config.host))
                .tcp_port(db_config.port)
                .user(Some(db_config.user))
                .pass(Some(db_config.password))
                .db_name(Some(db_config.database));
    match Conn::new(opts) {
        Ok(connection) => {
            Ok(connection)
        },
        Err(e) => {
            let errmsg = format!("Error connecting to database: {}", e);
            println!("{}", &errmsg);
            return Err(e);
        }
    }
}



impl User {
    pub fn user_to_string(user: &User) -> String {
        format!(
            "\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n",
            user.role_id,
            user.username,
            user.email,
            user.password,
            user.config,
            user.active,
            user.remember_token,
            user.avatar,
            user.created_at,
            user.updated_at,
        )
    }

    pub fn select_all() -> Result<Vec<User>>{
        let mut conn : Conn;
        let selected_users : Vec<User>;

        match init_db() {
            Ok(connection) => {
                conn = connection;
            },
            Err(e) => {
                let errmsg = format!("Error connecting to db: {}", e);
                println!("{}", errmsg);
                return Err(e);
            }
        }
        let stmt = "SELECT
                        role_id,
                        username,
                        email,
                        password,
                        config,
                        active,
                        remember_token,
                        avatar,
                        created_at,
                        updated_at
                    FROM
                    users";
        let select_res = conn.query_map(
            stmt,
            |(role_id, username, email, password, config, active, remember_token, avatar, created_at, updated_at)|
            -> User {
                User {
                    role_id,
                    username,
                    email,
                    password,
                    config,
                    active,
                    remember_token,
                    avatar,
                    created_at,
                    updated_at
                }
            },
        );
        match select_res {
            Ok(users) => {
                selected_users = users;
            },
            Err(e) => {
                let errmsg = format!("Error selecting from db: {}", e);
                println!("{}", errmsg);
                return Err(e);
            }
        }
        Ok(selected_users)
    }

    #[allow(dead_code)]
    pub fn create_users_from_vec() -> Result<()>{
        let users = vec![
            User { 
                role_id: 2,
                username: String::from("test_user_123"),
                email: String::from("test-user-123@test.com"),
                password: String::from("Ajmooo"),
                config: String::from("{\"test1\": \"test11\", \"test22\": \"testval2\"}"),
                active: true,
                remember_token: String::from("apisdvv3uzz453b4"),
                avatar: String::from("/img/default/user-avatar.png"),
                created_at: Local::now().naive_local(),
                updated_at: Local::now().naive_local()
            },
        ];
        /********************************************/
        match insert(users) {
            Ok(()) => {
                println!("insert success!\n");
            },
            Err(err) => {
                let errmsg = format!("error on insert(): {}", err);
                println!("{}", &errmsg);
                return Err(err);
            }
        }
        /********************************************/
        fn insert(users: Vec<User>) -> Result<()> {
            let mut conn : Conn;
            match init_db() {
                Ok(connection) => {
                    conn = connection;
                },
                Err(e) => {
                    let errmsg = format!("Error connecting to db: {}", e);
                    println!("{}", errmsg);
                    return Err(e);
                }
            }
            let stmt =
            "INSERT INTO users
                (role_id,username,email,password,config,active,remember_token,avatar,created_at,updated_at)
            VALUES
                (:role_id,:username,:email,:password,:config,:active,:remember_token,:avatar,:created_at,:updated_at)";
            
            // Strings are passed by reference!
            let __params = users.iter().map(|u| params!{
                "role_id" => u.role_id,
                "username" => &u.username,
                "email" => &u.email,
                "password" => &u.password,
                "config" => &u.config,
                "active" => u.active,
                "remember_token" => &u.remember_token,
                "avatar" => &u.avatar,
                "created_at" => u.created_at,
                "updated_at" => u.updated_at
            });
            println!("params to insert:\n\n{:?}\n", __params);
            match conn.exec_batch(stmt, __params) {
                Ok(()) => {
                    println!("Successfuly inserted users!");
                }
                Err(err) => {
                    let errmsg = format!("error on insert(): {}", err);
                    println!("{}", &errmsg);
                    return Err(err);
                }
            }
            Ok(())
        }
        /********************************************/
        Ok(())
    }
}

impl Token {
    pub fn token_to_string(token: &Token) -> String {
        format!(
            "\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n{}\r\n",
            token.user_id,
            token.token_type,
            token.access_token,
            token.refresh_token,
            token.token_expire,
            token.created_at,
            token.updated_at
        )
    }

    pub fn select_all() -> Result<Vec<Token>> {
        let mut conn : Conn;
        let selected_tokens : Vec<Token>;

        match init_db() {
            Ok(connection) => {
                conn = connection;
            },
            Err(e) => {
                let errmsg = format!("Error connecting to db: {}", e);
                println!("{}", errmsg);
                return Err(e);
            }
        }
        let stmt = "SELECT
                        user_id,
                        token_type,
                        access_token,
                        refresh_token,
                        token_expire,
                        created_at,
                        updated_at
                    FROM
                    tokens";
        let select_res = conn.query_map(
            stmt,
            |(user_id,token_type,access_token,refresh_token,token_expire,created_at,updated_at)| -> Token {
                Token {
                    user_id,
                    token_type,
                    access_token,
                    refresh_token,
                    token_expire,
                    created_at,
                    updated_at
                }
            },
        );
        match select_res {
            Ok(tokens) => {
                selected_tokens = tokens;
            },
            Err(e) => {
                let errmsg = format!("Error selecting from db: {}", e);
                println!("{}", errmsg);
                return Err(e);
            }
        }
        Ok(selected_tokens)
    }

    #[allow(dead_code)]
    pub fn create_tokens_from_vec() -> Result<()>{
        let tokens = vec![
            Token { 
                user_id: 71,
                token_type: String::from("Bearer"),
                access_token: String::from("9jojOELU1YcWq1sh3dRHLdn+GjA7e/Hn"),
                refresh_token: String::from("OGaQHohcJ4skNBulc5KPCMywyNB4JB7UvSS8isvsMTo="),
                token_expire: Local::now().naive_local(),
                created_at: Local::now().naive_local(),
                updated_at: Local::now().naive_local()
            },
        ];
        /********************************************/
        match insert(tokens) {
            Ok(()) => {
                println!("insert success!\n");
            },
            Err(err) => {
                let errmsg = format!("error on insert(): {}", err);
                println!("{}", &errmsg);
                return Err(err);
            }
        }
        /********************************************/
        fn insert(tokens: Vec<Token>) -> Result<()> {
            let mut conn : Conn;
            match init_db() {
                Ok(connection) => {
                    conn = connection;
                },
                Err(e) => {
                    let errmsg = format!("Error connecting to db: {}", e);
                    println!("{}", errmsg);
                    return Err(e);
                }
            }
            let stmt =
            "INSERT INTO tokens
                (user_id,token_type,access_token,refresh_token,token_expire,created_at,updated_at)
            VALUES
                (:user_id,:token_type,:access_token,:refresh_token,:token_expire,:created_at,:updated_at)";
                
            // Strings are passed by reference!
            let __params = tokens.iter().map(|t| params! {
                "user_id" =>  t.user_id,
                "token_type" =>  &t.token_type,
                "access_token" =>  &t.access_token,
                "refresh_token" =>  &t.refresh_token,
                "token_expire" =>  t.token_expire,
                "created_at" =>  t.created_at,
                "updated_at" =>  t.updated_at
            });
            println!("params to insert:\n\n{:?}\n", __params);
            match conn.exec_batch(stmt, __params) {
                Ok(()) => {
                    println!("Successfuly inserted tokens!");
                }
                Err(err) => {
                    let errmsg = format!("error on insert(): {}", err);
                    println!("{}", &errmsg);
                    return Err(err);
                }
            }
            Ok(())
        }
        /********************************************/
        Ok(())
    }
}