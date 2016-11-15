func process() {
	fmt.Println("processing")
}

func main() {
	fmt.Println("start")
	go process()
	time.Sleep(time.Millisecond * 10)
	fmt.Println("done")
}
