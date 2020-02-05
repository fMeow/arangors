use std::env;
pub const ARANGODB_HOST: &str = "http://localhost:8529/";

pub const ROOT_USERNAME: &str = "root";
pub const ROOT_PASSWORD: &str = "KWNngteTps7XjrNv";

pub const NORMAL_USERNAME: &str = "username";
pub const NORMAL_PASSWORD: &str = "password";

pub fn get_root_user() -> String {
    let root_user = env::var("ARANGO_ROOT_USER");

    match root_user {
        Err(_e) => ROOT_USERNAME.to_owned(),
        Ok(s) => s,
    }
}

pub fn get_root_password() -> String {
    let password = env::var("ARANGO_ROOT_PASSWORD");

    match password {
        Err(_e) => ROOT_PASSWORD.to_owned(),
        Ok(s) => s,
    }
}

pub fn get_normal_user() -> String {
    let user = env::var("ARANGO_USER");

    match user {
        Err(_e) => NORMAL_USERNAME.to_owned(),
        Ok(s) => s,
    }
}

pub fn get_normal_password() -> String {
    let password = env::var("ARANGO_PASSWORD");

    match password {
        Err(_e) => NORMAL_PASSWORD.to_owned(),
        Ok(s) => s,
    }
}

pub fn get_arangodb_host() -> String {
    let host = env::var("ARANGODB_HOST");

    match host {
        Err(_e) => ARANGODB_HOST.to_owned(),
        Ok(s) => format!("http://{}", s),
    }
}

#[test]
pub fn test_setup() {
    match env_logger::Builder::from_default_env()
        .is_test(true)
        .try_init()
    {
        _ => {}
    }
}

pub fn test_root_and_normal<T>(test: T) -> ()
where
    T: Fn(&str, &str) -> (),
{
    test(get_root_user().as_ref(), get_root_password().as_ref());
    test(get_normal_user().as_ref(), get_normal_password().as_ref());
}
