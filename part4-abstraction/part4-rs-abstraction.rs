#![feature(non_ascii_idents)]
#![allow(unused_variables)]

const PI: f64 = 3.1415926;

trait Form {
    fn fläche(&self) -> f64;
}

struct Rechteck {
    breite: f64,
    höhe: f64
}

impl Form for Rechteck {
    fn fläche(&self) -> f64 {
        self.breite * self.höhe
    }
}

struct Kreis {
    radius: f64
}

impl Form for Kreis {
    fn fläche(&self) -> f64 {
        PI * self.radius * self.radius
    }
}

// dynamic dispatch über Trait objects
fn fläche_ausgeben(f : &Form) {
    println!("Form hat Fläche {}", f.fläche())
}

// static dispatch über Trait bounds
fn fläche_ausgeben2<T: Form>(f: &T) {
    println!("Form hat Fläche {}", f.fläche())
}

fn main() {
    let r = Rechteck {breite: 10.0, höhe: 5.0};
    let k = Kreis { radius: 4.0 };
    fläche_ausgeben(&r);
    fläche_ausgeben(&k);
    fläche_ausgeben2(&r);
    fläche_ausgeben2(&k);
}
