pub struct PlanProyecto {
    pub nombres: Vec<String>,
    pub requisitos: Vec<Vec<usize>>,
}

impl PlanProyecto {
    pub fn new() -> Self {
        PlanProyecto {
            nombres: Vec::new(),
            requisitos: Vec::new(),
        }
    }

    pub fn agregar_tarea(&mut self, nombre: &str) -> usize {
        self.nombres.push(nombre.to_string());
        self.requisitos.push(Vec::new());
        self.nombres.len() - 1
    }

    pub fn agregar_dependencia(&mut self, antes: usize, despues: usize) {
        self.requisitos[antes].push(despues);
    }

    pub fn nombre_existe(&self, nombre: &str) -> bool {
        self.nombres.iter().any(|n| n.eq_ignore_ascii_case(nombre))
    }

    pub fn dependencia_existe(&self, antes: usize, despues: usize) -> bool {
        self.requisitos[antes].contains(&despues)
    }

    pub fn mostrar_plan(&self) {
        println!("\nPlan de tareas del proyecto:");
        for tarea in 0..self.nombres.len() {
            let nombre_tarea = &self.nombres[tarea];

            if self.requisitos[tarea].is_empty() {
                println!("[{}] {} -> no es requisito de ninguna otra", tarea, nombre_tarea);
            } else {
                let mut siguientes = Vec::new();
                for &siguiente in &self.requisitos[tarea] {
                    siguientes.push(format!("[{}] {}", siguiente, self.nombres[siguiente]));
                }
                println!("[{}] {} -> antes de: {}", tarea, nombre_tarea, siguientes.join(", "));
            }
        }
    }
}