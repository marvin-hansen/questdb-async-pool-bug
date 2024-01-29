# questdb-async-pool-bug

Minimal reproducible rust app of the QuestDB async pool bug 

Install Rust, if its not an the system. 

1. Clone this repo: Git clone https://github.com/marvin-hansen/questdb-async-pool-bug.git
2. Change into the project root: cd questdb-async-pool-bug
3. Run the app: cargo run

Results in:

Failed to get a client from the pool: Backend(Error { kind: Connect, cause: Some(Os { code: 61, kind: ConnectionRefused, message: "Connection refused" }) })

Run with stacktrace:  RUST_BACKTRACE=1  cargo run