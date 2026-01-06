use std::fs;
use std::path::Path;
use crate::task::Task;

const FILE_NAME: &str ="tasks.json";
pub fn load_data()->Vec<Task>{
	if !Path::new(FILE_NAME).exists(){
		Vec::new()
	}
	let data= fs::read_to_string(FILE_NAME).unwrap_or_default;
	serde_json::from_str(&data).unwrap_or_default()
}

pub fn save_tasks(tasks: &Vec<Task>){
	let data= serde_json::to_string_pretty(tasks).expect("failed to serialize tasks");
	fs::wite(FILE_NAME, data).expect("failed to write tasks");
}


