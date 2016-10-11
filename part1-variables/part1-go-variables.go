package main

import "fmt"

func main() {
	const pi = 3.1415926

	var a = 4 // Typ-inferenz
	var b int
	var c string = "hello" // Explizite Typ-Angabe
	d := "world"           // Kurzversion von 'var' name [type] = wert
	fmt.Println(pi, c, d, a, b)
}
