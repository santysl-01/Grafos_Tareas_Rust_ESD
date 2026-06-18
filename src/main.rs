mod grafo;
mod dfs;
mod entrada;

use grafo::PlanProyecto;
use entrada::{pausa, leer_linea, leer_numero};

fn analizar_plan(plan: &PlanProyecto) {
    plan.mostrar_plan();

    pausa("ahora corremos el algoritmo DFS sobre este plan");

    if dfs::tiene_ciclo(plan) {
        println!("\nresultado: el plan SI tiene un ciclo");
        println!("ninguna tarea puede ser la primera, el proyecto nunca podria iniciar");
    } else {
        println!("\nresultado: el plan no tiene ciclos");
        println!("se puede ordenar y completar todas las tareas sin problema");
    }
}

fn escenario_1() -> PlanProyecto {
    let mut plan = PlanProyecto::new();

    let comprar_pintura  = plan.agregar_tarea("Comprar pintura y materiales");
    let mover_muebles    = plan.agregar_tarea("Mover los muebles");
    let limpiar_paredes  = plan.agregar_tarea("Limpiar y lijar paredes");
    let pintar_pared     = plan.agregar_tarea("Pintar la pared");
    let acomodar_muebles = plan.agregar_tarea("Acomodar muebles de nuevo");

    plan.agregar_dependencia(comprar_pintura, pintar_pared);
    plan.agregar_dependencia(mover_muebles, limpiar_paredes);
    plan.agregar_dependencia(limpiar_paredes, pintar_pared);
    plan.agregar_dependencia(pintar_pared, acomodar_muebles);

    plan
}

fn escenario_2() -> PlanProyecto {
    let mut plan = PlanProyecto::new();

    let diseno   = plan.agregar_tarea("Disenar la base de datos");
    let backend  = plan.agregar_tarea("Construir el backend");
    let frontend = plan.agregar_tarea("Construir el frontend");
    let pruebas  = plan.agregar_tarea("Hacer pruebas de integracion");

    plan.agregar_dependencia(diseno, backend);
    plan.agregar_dependencia(backend, frontend);
    plan.agregar_dependencia(frontend, pruebas);

    plan.agregar_dependencia(pruebas, diseno);

    plan
}

fn construir_plan_personalizado() -> PlanProyecto {
    let mut plan = PlanProyecto::new();

    println!("\nvamos a construir tu propio plan de tareas");

    let total_tareas = leer_numero("cuantas tareas tendra tu plan? ");

    for i in 0..total_tareas {
        loop {
            let nombre = leer_linea(&format!("nombre de la tarea [{}]: ", i));

            if plan.nombre_existe(&nombre) {
                println!("ya existe una tarea con ese nombre, escribe otro");
                continue;
            }

            plan.agregar_tarea(&nombre);
            break;
        }
    }

    println!("\ntareas registradas:");
    for (i, nombre) in plan.nombres.iter().enumerate() {
        println!("[{}] {}", i, nombre);
    }

    println!("\nahora las dependencias (las flechas del grafo)");
    println!("una dependencia antes -> despues significa que antes debe completarse primero\n");

    let total_dependencias = leer_numero("cuantas dependencias quieres agregar? ");

    for i in 0..total_dependencias {
        println!("\ndependencia {} de {}", i + 1, total_dependencias);

        loop {
            let antes = leer_numero(&format!(
                "id de la tarea que va antes (0 a {}): ",
                total_tareas - 1
            ));
            let despues = leer_numero(&format!(
                "id de la tarea que va despues (0 a {}): ",
                total_tareas - 1
            ));

            if antes >= total_tareas || despues >= total_tareas {
                println!("ese id no existe, intenta otra vez");
                continue;
            }

            if plan.dependencia_existe(antes, despues) {
                println!("esa dependencia ya fue agregada antes, intenta otra vez");
                continue;
            }

            plan.agregar_dependencia(antes, despues);
            println!("agregado: [{}] {} -> [{}] {}", antes, plan.nombres[antes], despues, plan.nombres[despues]);
            break;
        }
    }

    plan
}

fn main() {
    println!("Planificador de tareas - deteccion de ciclos con DFS\n");

    loop {
        println!("\nMenu");
        println!("1) Ejemplo sin ciclo (pintar una habitacion)");
        println!("2) Ejemplo con ciclo (proyecto de software)");
        println!("3) Construir mi propio plan");
        println!("4) Salir");

        let opcion = leer_linea("\nElige una opcion: ");

        match opcion.as_str() {
            "1" => {
                let plan = escenario_1();
                analizar_plan(&plan);
            }
            "2" => {
                let plan = escenario_2();
                analizar_plan(&plan);
            }
            "3" => {
                let plan = construir_plan_personalizado();
                pausa("tu plan esta listo, vamos a analizarlo");
                analizar_plan(&plan);
            }
            "4" => {
                println!("\nhasta luego");
                break;
            }
            _ => {
                println!("\nesa opcion no existe, intenta otra vez");
            }
        }
    }
}