func process(done chan<- bool) {
	fmt.Println(" processing ")
	done <- true
}

func main() {
	done := make(chan bool)
	fmt.Println("start")
	go process(done)
	result := <-done
	fmt.Println("done: ", result)
}
