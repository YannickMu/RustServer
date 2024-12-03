#[path = "lib/threadpool.rs"] mod threadpool;
use std::sync::Arc;

use threadpool::ThreadPool;

pub struct Route {
	path: String,
	protected: bool,
	methodes: Vec<Job>
}

impl Route {
	pub fn execute<F>(&self, f: Job, pool: ThreadPool)
		where
			F: Fn(String) -> String,
		{
			pool.execute(|| f);
		}
}

type Job = Arc<dyn Fn(String) -> String>;
