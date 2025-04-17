use std::{io, collections::HashSet};

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

    let mut memo: HashSet<(usize, usize, i32)> = HashSet::new();

    if cambio_de_aula_top_down(&grilla, &mut memo, 0, 0, grilla[0][0]) {
        return String::from("YES")
    }

    String::from("NO")
}

fn cambio_de_aula_top_down(grilla: &Vec<Vec<i32>>, memo: &mut HashSet<(usize, usize,i32)>, i: usize, j: usize, diff: i32) -> bool {
    let n = grilla.len();
    let m = grilla[0].len();

    let pasos_restantes = (n + m - 1 - i - j) as i32;

    if diff.abs() > pasos_restantes {
        return false;
    }

    if i == n - 1 && j == m - 1 {
        return diff == 0;
    }

    if !memo.insert((i, j, diff)) {
        return false;
    }

    if i < n - 1 {
        if cambio_de_aula_top_down(grilla, memo, i + 1, j, diff + grilla[i + 1][j]) {
            return true
        }
    }

    if j < m - 1 {
        if cambio_de_aula_top_down(grilla, memo, i, j + 1, diff + grilla[i][j + 1]) {
            return true
        }
    }

    false
}


fn cambio_de_aula_bottom_up(grilla: Vec<Vec<i32>>) -> String {
    let n = grilla.len();
    let m = grilla[0].len();
    let max_diff = (n + m) as i32;

    let mut dp = vec![vec![HashSet::new(); m]; n];
    dp[0][0].insert(grilla[0][0]);

    for i in 0..n {
        for j in 0..m {
            if i == 0 && j == 0 {
                continue; // ya inicializado
            }

            let mut posibles = HashSet::new();
            let actual = grilla[i][j];

            if i > 0 {
                for &prev in &dp[i - 1][j] {
                    let nuevo = prev + actual;
                    if nuevo.abs() <= max_diff {
                        posibles.insert(nuevo);
                    }
                }
            }

            if j > 0 {
                for &prev in &dp[i][j - 1] {
                    let nuevo = prev + actual;
                    if nuevo.abs() <= max_diff {
                        posibles.insert(nuevo);
                    }
                }
            }

            dp[i][j] = posibles;
        }
    }

    if dp[n - 1][m - 1].contains(&0) {
        "YES".to_string()
    } else {
        "NO".to_string()
    }
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

            // println!("{:}", cambio_de_aula(grilla));
            println!("{:}", cambio_de_aula_bottom_up(grilla));
        } else {
            panic!("error leyendo dimensiones");
        }
    }
}

/*
fn hilocomunicados(s1: String, s2: String) -> f64 {
    let distancia_total = distancia_recorrida(&s1);
    let soluciones_totales = soluciones_totales(&s2);
    let soluciones_validas = hilocomunicados_bt(distancia_total, s2, 0);

    soluciones_validas/soluciones_totales
}

fn soluciones_totales(s: &String) -> f64 {
    let cantidad_ramas: f64 = s.chars().fold(0.0, |acc, c| if c == '?' { acc + 1.0 } else { acc });
    
    2.0f64.powf(cantidad_ramas)
}

fn hilocomunicados_bt(d: i32, mut s: String, dt: i32) -> f64 {
    if s.is_empty() {
        if d == dt {
            return 1.0;
        }

        return 0.0;
    }

    let distancia_restante = (d - dt).abs();
    let movimientos_restantes = s.len() as i32;

    if distancia_restante > movimientos_restantes {
        return 0.0;
    }

    if (movimientos_restantes - distancia_restante) % 2 != 0 {
        return 0.0;
    }

    match s.pop().unwrap() {
        '+' => hilocomunicados_bt(d, s, dt + 1),
        '-' => hilocomunicados_bt(d, s, dt - 1),
        '?' => {
            let a = hilocomunicados_bt(d, s.clone(), dt + 1);
            let b = hilocomunicados_bt(d, s, dt - 1);

            a + b
        },
        _ => panic!("input mal formateado: s2")
    }
}

fn distancia_recorrida(s: &String) -> i32 {
    s.chars().fold(0, |distancia, c| match c {
        '+' => distancia + 1,
        '-' => distancia - 1,
        _ => panic!("input mal formateado: s1")
    })
}

fn main() {
    let mut i1 = String::new();
    let mut i2 = String::new();

    io::stdin().read_line(&mut i1).expect("error leyendo s1");
    io::stdin().read_line(&mut i2).expect("error leyendo s2");

    let s1 = i1.trim().to_string();
    let s2 = i2.trim().to_string();

    println!("{:.12}", hilocomunicados(s1, s2));
}
*/
