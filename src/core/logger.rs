pub struct Log;

impl Log {
    pub fn info(msg: &str) {
        println!("INFO: {}", msg);
    }

    pub fn error(msg: &str) {
        eprintln!("ERROR: {}", msg);
    }

    pub fn success(msg: &str) {
        println!("SUCCESS: {}", msg);
    }
}
