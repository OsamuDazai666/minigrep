use std::env;

pub struct Args {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Args {
    pub fn build(args: &[String]) -> Result<Args, &'static str> {
        if args.len() < 3 {
            return Err("Not Enough Arguments...");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Args {
            query,
            file_path,
            ignore_case,
        })
    }
}
