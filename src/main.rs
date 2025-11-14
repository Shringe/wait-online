mod cli;

use clap::Parser;
use cli::Args;
use std::process::exit;
use std::thread::sleep;
use std::time::Duration;
use trust_dns_resolver::Resolver;

/// Uses println!() in debug mode. Optimizies completely out of release builds
macro_rules! debug {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        println!($($arg)*);
    }};
}

/// Uses eprintln!() in debug mode. Optimizies completely out of release builds
macro_rules! edebug {
    ($($arg:tt)*) => {{
        #[cfg(debug_assertions)]
        eprintln!($($arg)*);
    }};
}

fn main() {
    let args = Args::parse();
    debug!("Args: {:#?}", args);

    let delay = Duration::from_millis(args.delay);

    println!("Waiting to connect to endpoint: {}", args.endpoint);
    for attempt in 1..=args.max_retries {
        debug!("Attempt {}/{}", attempt, args.max_retries);

        let resolver = match Resolver::from_system_conf() {
            Ok(resolver) => resolver,
            Err(e) => {
                edebug!("Failed to create resolver: {}", e);
                continue;
            }
        };

        match resolver.lookup_ip(&args.endpoint) {
            Ok(lookup) => {
                debug!("Lookup result: {:#?}", lookup);
                println!("Successfully reached enpoint");
                exit(0);
            }
            Err(e) => {
                edebug!("Failed to reach endpoint: {}", e);
            }
        }

        if attempt < args.max_retries {
            sleep(delay);
        }
    }

    eprintln!("Failed to reach endpoint");
    exit(1);
}
