pub mod atomic;
pub mod condition_variable;
pub mod functional;
pub mod memory;
pub mod mutex;
pub mod string;
pub mod vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn string() {
        let _ = env_logger::builder().is_test(true).try_init();

        let mut string = string::String::new();
        info!("empty: {:?} ({:?})", string.data(), string);
        assert_eq!("", string.data());

        string.assign("Hello, world!");
        info!("small: {:?} ({:?})", string.data(), string);
        assert_eq!("Hello, world!", string.data());

        string.assign("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        info!("large: {:?} ({:?})", string.data(), string);
        assert_eq!("ABCDEFGHIJKLMNOPQRSTUVWXYZ", string.data());
    }
}
