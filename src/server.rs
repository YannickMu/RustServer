#[path = "lib/threadpool.rs"] mod threadpool;
use threadpool::ThreadPool;

pub struct Server {
	addr: String,
	threadpool: ThreadPool,
}

impl Server {
	pub fn new(addr: String, threads: usize) -> Server {
		let pool: ThreadPool = ThreadPool::new(threads);
		return Server { addr, threadpool: pool };
	}
}
