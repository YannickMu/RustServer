#[path = "lib/threadpool.rs"] mod threadpool;
use std::sync::Arc;

use threadpool::ThreadPool;

pub struct Route<'a> {
	path: &'a str,
	protected: bool,
	pub GET: Arc<dyn Fn(&str) -> String>
}

impl Route<'_> {
	pub fn new(path: &str, protected: bool, GET: Arc<dyn Fn(&str) -> String>) -> Route {
		Route { path, protected, GET }
	}
}

type Job = Box<(dyn FnOnce() -> String + Send + 'static)>;
