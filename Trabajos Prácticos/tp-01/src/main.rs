use std::io;

fn hilocomunicados(s1: String, s2: String) -> f64 {
    let distancia_total = distancia_recorrida(s1);
    let (soluciones_totales, soluciones_validas) = hilocomunicados_bt(distancia_total, (0.0,0.0), s2, 0);

    soluciones_validas/soluciones_totales
}

fn hilocomunicados_bt(d: i32, mut sp: (f64, f64), mut s: String, dt: i32) -> (f64, f64) {
    if s.is_empty() {
        if d == dt {
            sp.1 += 1.0
        }

        sp.0 += 1.0;

        return sp
    }

    let distancia_restante = (d - dt).abs();
    let movimientos_restantes = s.len() as i32;

    if distancia_restante > movimientos_restantes {
        sp.0 += 1.0;
        return sp;
    }

    if (movimientos_restantes - distancia_restante) % 2 != 0 {
        sp.0 += 1.0;
        return sp;
    }

    match s.pop().unwrap() {
        '+' => hilocomunicados_bt(d, sp, s, dt + 1),
        '-' => hilocomunicados_bt(d, sp, s, dt - 1),
        '?' => {
            let a = hilocomunicados_bt(d, sp, s.clone(), dt + 1);
            let b = hilocomunicados_bt(d, sp, s, dt - 1);
            sp.0 = a.0 + b.0;
            sp.1 = a.1 + b.1;

            sp
        },
        _ => panic!("input mal formateado: s2")
    }
}

fn distancia_recorrida(s: String) -> i32 {
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
