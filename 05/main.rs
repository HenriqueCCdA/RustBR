use std::io;


fn main() {
    let mut s = String::new();
    println!("Digite um texto");

    io::stdin().read_line(&mut s).expect("Error readgin console");

    println!("Você digitou {s}");

}


// fn main() {

    // let l0 = 'b';
    // let l1 = 'r';
    // let l2 = 'u';
    // let l3 = 'n';
    // let l4 = 'o';

    // println!("{l0}{l1}{l2}{l3}{l4}");

    // let letter = "A"; // 0100 0001 ->xO f 4241

    // let nome: &str = "Bruno";

    // String Dinâmica
    // let mut s = String::new();

    // s.push('b');
    // s.push('r');
    // s.push('u');
    // s.push('n');
    // s.push('o');

    // s.push_str("Bruno");

    // println!("{s}")
// }