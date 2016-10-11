#![feature(non_ascii_idents)]
#![allow(unused_variables)]

struct Rechteck {
    breite: f64,
    höhe: f64
}

fn main() {
    let a = 4;
    let b: i32;
    let c: String = "hello".to_string();
    // c = "Hallo".to_string();     // c ist immutable --> Fehler
    let mut d : String = "welt".to_string();
    d = "world".to_string();
    println!("{} {} {}", c, d, a);
    //println!("{}", b); // b ist nicht initialisiert --> Fehler

    let r = Rechteck {breite: 2.4, höhe: 5.0};
    let moved_r = r;
    //let another_r = r; // r steckt bereits in moved_r --> Fehler

    let mut r2 =  Rechteck {breite: 4.4, höhe: 5.0};
    {
        let borrowed_r2 = &r2;
        // da r2 nun von borrowed_r2 borrowed ist, kann es in 
        // der folgenden Zeile nicht mehr verändert werden --> Fehler
        //r2.breite = 3.5;
    }
    // Die Lebenszeit von borrowed_r2 und damit des borrows von
    // r2 ist mit der geschweiften Klammer beendet und r2 kann
    // wieder geändert werden
    r2.breite = 10.0;
}
