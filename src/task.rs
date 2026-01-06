use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Task{
	pub id: u32,
	pub title: String,
	pub completed: bool,
}

impl Task{
	pub fn new(id: u32,title: String)->Self{
		Self{
			id,
			title,
			completed: false
		}
	}
}
