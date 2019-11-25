// enqueued background jobs runner bin

use srv::bot::jobs::Environment;
use srv::db;

use diesel::r2d2;
use std::thread::sleep;
use std::time::Duration;

fn main() {
    println!(">>> Booting Background jobs runner");

    let db_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db_config = r2d2::Pool::builder().max_size(4);
    let db_pool = db::diesel_pool(db_url, db_config);

    let job_start_timeout = dotenv::var("BACKGROUND_JOB_TIMEOUT")
        .unwrap_or_else(|_| "30".into())
        .parse()
        .expect("Invalid value for `BACKGROUND_JOB_TIMEOUT`");

    let environment = Environment::new(db_pool.clone());

    let build_runner = || {
        swirl::Runner::builder(db_pool.clone(), environment.clone())
            .thread_count(2)
            .job_start_timeout(Duration::from_secs(job_start_timeout))
            .build()
    };
    let mut runner = build_runner();

    println!(">>> Runner booted, running jobs");

    let mut failure_count = 0;

    loop {
        if let Err(e) = runner.run_all_pending_jobs() {
            failure_count += 1;
            if failure_count < 5 {
                eprintln!(
                    ">>!! Error running jobs (n = {}) -- retrying: {:?}",
                    failure_count, e,
                );
                runner = build_runner();
            } else {
                panic!("!! Failed to run jobs 5 times. Restarting the process");
            }
        }
        sleep(Duration::from_secs(1));
    }
}
