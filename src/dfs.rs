use crate::grafo::PlanProyecto;
use crate::entrada::pausa;

const BLANCO: u8 = 0;
const GRIS: u8 = 1;
const NEGRO: u8 = 2;

fn nombre_color(c: u8) -> &'static str {
    match c {
        BLANCO => "blanco (sin revisar)",
        GRIS => "gris (revisandose ahora)",
        NEGRO => "negro (ya revisada)",
        _ => "?",
    }
}

pub fn tiene_ciclo(plan: &PlanProyecto) -> bool {
    let total_tareas = plan.nombres.len();
    let mut color = vec![BLANCO; total_tareas];

    println!("\nEmpezando DFS para buscar ciclos");
    println!("al inicio todas las tareas estan en blanco\n");

    for tarea_inicial in 0..total_tareas {
        if color[tarea_inicial] == BLANCO {
            pausa(&format!(
                "vamos a explorar desde [{}] {}",
                tarea_inicial, plan.nombres[tarea_inicial]
            ));

            if explorar(plan, tarea_inicial, &mut color) {
                return true;
            }
        }
    }

    false
}

fn explorar(plan: &PlanProyecto, tarea_actual: usize, color: &mut Vec<u8>) -> bool {
    color[tarea_actual] = GRIS;
    println!(
        "[{}] {} ahora es {}",
        tarea_actual, plan.nombres[tarea_actual], nombre_color(GRIS)
    );

    for &siguiente in &plan.requisitos[tarea_actual] {
        println!(
            "  revisando [{}] {} -> [{}] {} (color: {})",
            tarea_actual, plan.nombres[tarea_actual],
            siguiente, plan.nombres[siguiente],
            nombre_color(color[siguiente])
        );

        if color[siguiente] == GRIS {
            println!("\nla tarea [{}] {} ya esta en gris", siguiente, plan.nombres[siguiente]);
            println!("eso significa que ya estaba en el camino actual, hay un ciclo");
            println!("ciclo encontrado: {} depende (indirectamente) de {}",
                plan.nombres[tarea_actual], plan.nombres[siguiente]);
            return true;
        }

        if color[siguiente] == BLANCO {
            pausa(&format!(
                "la tarea [{}] {} esta blanca, toca explorarla",
                siguiente, plan.nombres[siguiente]
            ));

            if explorar(plan, siguiente, color) {
                return true;
            }
        } else {
            println!("  [{}] {} ya esta negra, no hay problema", siguiente, plan.nombres[siguiente]);
        }
    }

    color[tarea_actual] = NEGRO;
    println!(
        "[{}] {} ahora es {}",
        tarea_actual, plan.nombres[tarea_actual], nombre_color(NEGRO)
    );
    false
}