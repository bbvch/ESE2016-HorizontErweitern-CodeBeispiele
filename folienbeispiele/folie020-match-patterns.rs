match x {
    1 => "eins",
    2 => "zwei",
    3 | 4 => "drei oder vier"
    10 ... 20 => "zwischen 10 und 20 inklusive",
    _ => "anderes",


    Point { x, .. } => format!("Point mit x={}", x),
    Ok(value) => format!("Option mit Wert {}", value),
    Err(_) => "Fehler!",
    Some(value) if value > 50 => "Optionaler Wert > 50"
}
