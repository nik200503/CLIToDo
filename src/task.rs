use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task{
	pub id: u8,
	pub title: string,
	pub completed: bool,
}

impl Task{
	pub fn new(id: u8,title: string)->self{
		self{
			id,
			title,
			completed: false
		}
	}
}
