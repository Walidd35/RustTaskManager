use std::fs::OpenOptions;
use std::io::{self, BufReader};
use serde::{Serialize, Deserialize};

// Structure pour représenter une tâche
#[derive(Serialize, Deserialize)]
struct Task {
    description: String,
    completed: bool,
}

impl Task {
    fn new(description: String) -> Self {
        Task {
            description,
            completed: false,
        }
    }
}

// Ajouter une tâche
fn add_task(description: String) -> io::Result<()> {
    let mut tasks = load_tasks().unwrap_or_else(|_| Vec::new());
    tasks.push(Task::new(description));
    save_tasks(&tasks)
}

// Lister toutes les tâches
fn list_tasks() -> io::Result<()> {
    let tasks = load_tasks()?;
    for (i, task) in tasks.iter().enumerate() {
        println!("{}: {} [{}]", i + 1, task.description, if task.completed { "✓" } else { " " });
    }
    Ok(())
}

// Marquer une tâche comme terminée
fn mark_task_completed(index: usize) -> io::Result<()> {
    let mut tasks = load_tasks()?;
    if let Some(task) = tasks.get_mut(index) {
        task.completed = true;
        save_tasks(&tasks)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Task not found"))
    }
}

// Supprimer une tâche
fn delete_task(index: usize) -> io::Result<()> {
    let mut tasks = load_tasks()?;
    if index < tasks.len() {
        tasks.remove(index);
        save_tasks(&tasks)
    } else {
        Err(io::Error::new(io::ErrorKind::NotFound, "Task not found"))
    }
}

// Charger les tâches à partir du fichier JSON
fn load_tasks() -> Result<Vec<Task>, serde_json::Error> {
    let file = OpenOptions::new().read(true).open("task.json");
    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            serde_json::from_reader(reader)
        }
        Err(_) => Ok(Vec::new()), // Si le fichier n'existe pas, retourner une liste vide.
    }
}

// Sauvegarder les tâches dans le fichier JSON
fn save_tasks(tasks: &[Task]) -> io::Result<()> { // Changer tasks en task
    let file = OpenOptions::new().write(true).create(true).truncate(true).open("task.json")?;
    serde_json::to_writer(file, tasks)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: gestionnaire_de_taches <command> [<args>]");
        println!("Commands:");
        println!("  add <description>   Ajouter une nouvelle tâche");
        println!("  list                Lister toutes les tâches");
        println!("  complete <index>    Marquer une tâche comme terminée");
        println!("  delete <index>      Supprimer une tâche");
        return Ok(());
    }

    match args[1].as_str() {
        "add" => {
            let description = args[2..].join(" ");
            add_task(description)?;
        }
        "list" => list_tasks()?,
        "complete" => {
            let index: usize = args[2].parse().expect("Index doit être un nombre");
            mark_task_completed(index - 1)?;
        }
        "delete" => {
            let index: usize = args[2].parse().expect("Index doit être un nombre");
            delete_task(index - 1)?;
        }
        _ => {
            println!("Commande non reconnue.");
        }
    }
    Ok(())
}
