use std::{io, collections::HashMap};

fn obtener_aulas (cantidad_filas: usize, cantidad_columnas: usize) -> Vec<Vec<i32>> {
    let mut aulas: Vec<Vec<i32>> = Vec::new();

    for n in 0.. cantidad_filas {
        let mut fila = String::new();

        io::stdin().read_line(&mut fila).expect("error leyendo fila");

        let fila_aulas: Vec<i32> = fila.trim().split(" ")
            .map(|a| a.parse().expect("error parseando aula"))
            .collect();

        if fila_aulas.len() != cantidad_columnas {
            panic!("error parseando fila {:}", n);
        } else {
            aulas.push(fila_aulas)
        }
    }

    aulas
}

fn cambio_de_aula(grilla: Vec<Vec<i32>>) -> String {
    let n = grilla.len();
    let m = grilla[0].len();

    if (n + m - 1) % 2 != 0 {
        return String::from("NO");
    }

    let mut memo: HashMap<(usize, usize, i32), bool> = HashMap::new();

    if cambio_de_aula_top_down(&grilla, &mut memo, 0, 0, grilla[0][0]) {
        return String::from("YES")
    }

    String::from("NO")
}

fn cambio_de_aula_top_down(grilla: &Vec<Vec<i32>>, memo: &mut HashMap<(usize, usize, i32), bool>, i: usize, j: usize, diff: i32) -> bool {
    let n = grilla.len();
    let m = grilla[0].len();

    if i == n - 1 && j == m - 1 {
        return diff == 0;
    }

    if let Some(&existe_camino_memo) = memo.get(&(i, j, diff)) {
        return existe_camino_memo;
    }

    let mut existe_camino = false;

    if i < n - 1 {
        existe_camino = existe_camino || cambio_de_aula_top_down(grilla, memo, i + 1, j, diff + grilla[i + 1][j]);

        if existe_camino {
            return true;
        }
    }

    if j < m - 1 {
        existe_camino = existe_camino || cambio_de_aula_top_down(grilla, memo, i, j + 1, diff + grilla[i][j + 1]);

        if existe_camino {
            return true;
        }
    }

    memo.insert((i, j, diff), existe_camino);

    existe_camino
}
 
fn main() {
    let mut input_casos_prueba = String::new();

    io::stdin().read_line(&mut input_casos_prueba).expect("error leyendo cantidad de casos de prueba");
    let casos_prueba: i32 = input_casos_prueba.trim().parse().expect("error parseando cantidad de casos de prueba");

    for _ in 0.. casos_prueba {
        let mut input_dimensiones = String::new();

        io::stdin().read_line(&mut input_dimensiones).expect("error leyendo dimensiones de facultad");

        let dimensiones: Vec<usize> = input_dimensiones.trim().split(" ")
            .map(|d| d.parse().expect("error parseando dimensiones de la facultad"))
            .collect();

        if dimensiones.len() == 2 {
            let grilla: Vec<Vec<i32>> = obtener_aulas(dimensiones[0], dimensiones[1]);

            println!("{:}", cambio_de_aula(grilla));
        } else {
            panic!("error leyendo dimensiones");
        }
    }
}

