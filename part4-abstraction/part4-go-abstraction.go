package main

import "fmt"
import "math"

type form interface {
	fläche() float64
}

type rechteck struct {
	breite, höhe float64
}

func (r *rechteck) fläche() float64 {
	return r.breite * r.höhe
}

type kreis struct {
	radius float64
}

func (k *kreis) fläche() float64 {
	return math.Pi * k.radius * k.radius
}

func fläche_ausgeben(f form) {
	fmt.Println("Form ", f, " hat Fläche ", f.fläche())
}

func main() {
	r := rechteck{breite: 10, höhe: 5}
	k := kreis{radius: 4}
	fläche_ausgeben(&r)
	fläche_ausgeben(&k)
}
