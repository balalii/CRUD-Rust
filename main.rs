use std::collections::HashMap;
use std::io;

struct Patient {
    id: u32,
    name: String,
    age: u32,
    diagnosis: String,
}

struct Hospital {
    patients: HashMap<u32, Patient>,
    next_id: u32,
}

impl Hospital {
    fn new() -> Hospital {
        Hospital {
            patients: HashMap::new(),
            next_id: 1,
        }
    }

    fn add_patient(&mut self, name: String, age: u32, diagnosis: String) {
        let patient = Patient {
            id: self.next_id,
            name,
            age,
            diagnosis,
        };

        self.patients.insert(patient.id, patient);
        self.next_id += 1;
    }

    fn view_patients(&self) {
        println!("{:<5} {:<20} {:<5} {}", "ID", "Name", "Age", "Diagnosis");

        for (_, patient) in &self.patients {
            println!("{:<5} {:<20} {:<5} {}", patient.id, patient.name, patient.age, patient.diagnosis);
        }
    }

    fn edit_patient(&mut self, id: u32, new_diagnosis: String) {
        if let Some(patient) = self.patients.get_mut(&id) {
            patient.diagnosis = new_diagnosis;
            println!("Patient {} updated.", id);
        } else {
            println!("Patient with ID {} not found.", id);
        }
    }

    fn delete_patient(&mut self, id: u32) {
        if let Some(_) = self.patients.remove(&id) {
            println!("Patient {} deleted.", id);
        } else {
            println!("Patient with ID {} not found.", id);
        }
    }
}

fn main() {
    let mut hospital = Hospital::new();

    loop {
        println!("1. Add Patient");
        println!("2. View Patients");
        println!("3. Edit Patient");
        println!("4. Delete Patient");
        println!("5. Exit");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        match choice {
            1 => {
                println!("Enter patient name:");
                let mut name = String::new();
                io::stdin().read_line(&mut name).expect("Failed to read line");

                println!("Enter patient age:");
                let mut age = String::new();
                io::stdin().read_line(&mut age).expect("Failed to read line");
                let age: u32 = match age.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("Enter patient diagnosis:");
                let mut diagnosis = String::new();
                io::stdin().read_line(&mut diagnosis).expect("Failed to read line");

                hospital.add_patient(name.trim().to_string(), age, diagnosis.trim().to_string());
            }
            2 => hospital.view_patients(),
            3 => {
                println!("Enter patient ID to edit:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                println!("Enter new diagnosis:");
                let mut new_diagnosis = String::new();
                io::stdin()
                    .read_line(&mut new_diagnosis)
                    .expect("Failed to read line");

                hospital.edit_patient(id, new_diagnosis.trim().to_string());
            }
            4 => {
                println!("Enter patient ID to delete:");
                let mut id = String::new();
                io::stdin().read_line(&mut id).expect("Failed to read line");
                let id: u32 = match id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => continue,
                };

                hospital.delete_patient(id);
            }
            5 => {
                println!("Exiting program.");
                break;
            }
            _ => continue,
        }
    }
}
