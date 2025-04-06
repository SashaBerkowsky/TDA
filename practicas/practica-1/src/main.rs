// Backtracking:
// 1
fn existe_suma_subconjuntos_bt(c: &Vec<i32>, k: i32, i: usize, p: &mut Vec<i32>) -> bool{
    if i == 0 {
        if k == 0 {
            println!("Subconjunto encontrado: {:?}", p);
            return true;
        } else {
            return false;
        }
    }

    // reglas de factibilidad
    if k < 0 {
        return false;
    }
    if k == 0 {
        println!("Subconjunto encontrado: {:?}", p);
        return true;
    }

    // incluir el elemento c[i]
    p.push(c[i]);
    if existe_suma_subconjuntos_bt(c, k - c[i], i - 1, p) {
        return true;
    }
    p.pop(); 

    // no incluir el elemento c[i]
    if existe_suma_subconjuntos_bt(c, k, i - 1, p) {
        return true;
    }

    false
}

fn all_suma_subconjuntos_bt(c: &Vec<i32>, k: i32, i: usize, p: &mut Vec<i32>, s: &mut Vec<Vec<i32>>) {
    if k < 0 {
        return;
    }

    if k == 0 {
        s.push(p.to_vec());
        println!("{:?}", p);
        return;
    }

    if i == 0 {
        if k - c[i] == 0 {
            p.push(c[i]);
            s.push(p.to_vec());
            println!("{:?}", p);
            p.pop();
        }

        return
    }

    if k - c[i] >= 0 {
        p.push(c[i]);
        all_suma_subconjuntos_bt(c, k - c[i], i - 1, p, s);
        p.pop();
    }

    all_suma_subconjuntos_bt(c, k, i - 1, p, s);
}

fn main() {
    let c = Vec::from([6,12,6]);
    let k: i32 = 12;
    let mut p: Vec<i32> = Vec::new();
    let mut q: Vec<i32> = Vec::new();
    let mut sol: Vec<Vec<i32>> = Vec::new();

    // busca 1 subconjunto, muestra el primero que cumple
    println!("hay subconjunto: {:}", existe_suma_subconjuntos_bt(&c, k, c.len() - 1, &mut p));

    // muestra todos los subconjuntos
    all_suma_subconjuntos_bt(&c, k, c.len() - 1, &mut q, &mut sol);
    println!("{:?}", sol);
}
