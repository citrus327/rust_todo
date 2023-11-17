mod auto_increment_id;
mod dir;
mod storage;

#[cfg(test)]
mod tests {
    #[test]
    fn test_print_table() {
        println!(
            "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
            "total", "blanks", "comments", "code"
        );
        println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", 0, 0, 0, 0);
        println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", 77, 0, 3, 74);
        println!("{0: <10} | {1: <10} | {2: <10} | {3: <10}", 112, 0, 6, 106);
        println!(
            "{0: <10} | {1: <10} | {2: <10} | {3: <10}",
            460, 0, 10, 1371
        );
    }

    #[test]
    fn print_envs() {
        macro_rules! print_env {
            ($name: expr) => {
                println!("{}={}", $name, env!($name))
            };
        }

        print_env!("CARGO_PKG_VERSION_MAJOR");
        print_env!("CARGO_MANIFEST_DIR");
        print_env!("CARGO_PKG_AUTHORS");
        print_env!("CARGO_PKG_DESCRIPTION");
        print_env!("CARGO_PKG_HOMEPAGE");
        print_env!("CARGO_PKG_NAME");
        print_env!("CARGO_PKG_REPOSITORY");
        print_env!("CARGO_PKG_VERSION");
        print_env!("CARGO_PKG_VERSION_MAJOR");
        print_env!("CARGO_PKG_VERSION_MINOR");
        print_env!("CARGO_PKG_VERSION_PATCH");
        print_env!("CARGO_PKG_VERSION_PRE");
    }
}
