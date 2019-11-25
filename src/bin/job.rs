// put background jobs to queue

use srv::errors::{SrvError, SrvResult};
use srv::{db, bot::tasks};
use swirl::Job;

fn main() -> SrvResult<()> {
    let conn = db::connect_now()?;
    tasks::spider_items()
        .enqueue(&conn)
        .map_err(|e| SrvError::from_std_error(e))?;

    Ok(())
}
