mod storage;
mod task;

use clap::{Parser , Subcommand};
use storage::{load_tasks, save_tasks};
use task::Task;

#[derive(Parser)]
#[command( name="todo")]
#[command(about = "simple cli to app", long_about = None)]
struct Cli{
	#[command(subcommand)]
	command: Commands,
}

#[derive(Subcommand)]
enum Commands{
	Add {
		title: String,
	},
	List,
	Done{
		id: u32,
	},
	Delete{
		id:u32,
	},
}

fn main(){
	let cli = Cli::parse();
	let mut tasks = load_tasks();
	
	match cli.command{
		Commands::Add { title }=> {
			let next_id = tasks.iter().map(|t| t.id).max().unwrap_or(0)+1;
			let task= Task::new(next_id, title);
			tasks.push(task);
			save_tasks(&tasks);
			println!("task added");
		}
		
		Commands::List => {
			if tasks.is_empty() {
				println!("No tasks found");
			}else {
				for task in tasks {
					let status = if task.completed { "[x]"} else {"[ ]"};
					println!("{} {}: {}", status,task.id,task.title);
				}
			}
		}
		Commands::Done{ id } => {
			match tasks.iter_mut().find(|t| t.id == id){
				Some(task)=>{
					task.completed = true;
					save_tasks(&tasks);
					println!("Task {} marked as done", id);
				}
				None=> println!("task doesnt exist"),
			}
		}
		
		Commands::Delete {id} =>{
			let len_before = tasks.len();
			tasks.retain(|t| t.id !=id);
			
			if tasks.len() == len_before{
				println!("task dont exist bitch");
			}else{
				save_tasks(&tasks);
				println!("task deleted successfully");
			}
		}
	}
}
