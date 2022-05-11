use crate::lib::database;

pub fn validate_http_request(buffer: &str) -> Result<Vec<&str>, String> {
    match buffer.split_once("\r\n") {
        Some(httprequest) => {
            let http_req : Vec<&str> = httprequest.0.split(' ').collect();
            Ok(http_req)
        }, 
        None => {
            // ???????
            let errmsg = "Input does not consist of any newlines - not a HTTP request - skipping..";
            return Err(String::from(errmsg))
        }
    }
}

pub fn process_request(request_method: &str, route: &str, routes: &[&str; 3]) -> Result<String, String>{
    let mut response_data : String = String::new();
    // SELECT on GET | INSERT on POST
    if request_method == "POST" {
        if route == routes[1] {
            match database::User::create_users_from_vec() {
                Ok(()) => {},
                Err(e) => {
                    let errmsg = format!("Error inserting users: {}", e);
                    println!("{}", &errmsg);
                    return Err(errmsg);
                }
            }
        }
        else if route == routes[2] {
            match database::Token::create_tokens_from_vec() {
                Ok(()) => {},
                Err(e) => {
                    let errmsg = format!("Error inserting tokens: {}", e);
                    println!("{}", &errmsg);
                    return Err(errmsg);
                }
            }
        }
        else if route == routes[0] { /*** (default route '/') */
            response_data = String::from("Default route - default response :3")
        }
        else {
            return Err(String::from(format!("Invalid route {}", route)))
        }
    }
    else if request_method == "GET" {
        if route == routes[1] {
            match database::User::select_all() {
                Ok(u) => {
                    let mut usersstr = String::new();
                    for user in u.iter() {
                        usersstr.push_str(database::User::user_to_string(&user).as_str());
                    }
                    response_data.push_str(usersstr.as_str());
                },
                Err(e) => {
                    println!("Error selecting users: {:?}", e);
                }
            }
        }
        else if route == routes[2] {
            match database::Token::select_all() {
                Ok(t) => {
                    let mut tokensstr = String::new();
                    for token in t.iter() {
                        tokensstr.push_str(database::Token::token_to_string(&token).as_str());
                    }
                    response_data.push_str(tokensstr.as_str());
                },
                Err(e) => {
                    println!("Error selecting tokens: {:?}", e);
                }
            }
        }
        else if route == routes[0] { /*** (default route '/') */
            response_data = String::from("Default route - default response :3")
        }
        else {
            return Err(String::from(format!("Invalid route {}", route)))
        }
    }
    else {
        return Err(String::from(format!("Invalid request method [{}]: supporting only GET|POST", request_method)))
    }

    Ok(response_data)
}