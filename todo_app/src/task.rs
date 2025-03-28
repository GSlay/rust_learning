pub struct Task {
	_text: String,
	_done: bool,
}

impl Task {
	pub fn new_task(text:String) -> Task {
		return Task {
			_text: text,
			_done: false,
		};
	}
	pub fn complete_task(mut task:Task) -> () {
		task._done = true;
	}
}