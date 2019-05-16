pub const ROOT_USERNAME: &str = "root";
pub const ROOT_PASSWORD: &str = "KWNngteTps7XjrNv";

pub const NORMAL_USERNAME: &str = "username";
pub const NORMAL_PASSWORD: &str = "password";

#[test]
pub fn test_setup() {
    env_logger::init();
}

pub fn test_root_and_normal<T>(test: T) -> ()
where
    T: Fn(&str, &str) -> (),
{
    test(ROOT_USERNAME, ROOT_PASSWORD);
    //    test(NORMAL_USERNAME, NORMAL_PASSWORD);
}
