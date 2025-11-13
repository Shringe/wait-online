mod cli;

use clap::Parser;
use cli::Args;
use std::process::exit;
use std::thread::sleep;
use std::time;
use trust_dns_resolver::Resolver;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};

/// Uses println!() in debug mode. Optimizies completely out of release builds
macro_rules! debug {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        println!($($arg)*);
    }};
}

fn main() {
    let args = Args::parse();
    debug!("{:#?}", args);

    let resolver = Resolver::new(ResolverConfig::default(), ResolverOpts::default())
        .expect("Failed to create resolver");
    let delay = time::Duration::from_millis(args.delay);

    println!("Waiting to connect to endpoint: {}", args.endpoint);
    for attempt in 1..=args.max_retries {
        debug!("Attempt {}/{}", attempt, args.max_retries);

        if resolver.lookup_ip(&args.endpoint).is_ok() {
            println!("Successfully reached enpoint");
            exit(0);
        }

        if attempt < args.max_retries {
            sleep(delay);
        }
    }

    eprintln!("Failed to reach endpoint");
    exit(1);
}
