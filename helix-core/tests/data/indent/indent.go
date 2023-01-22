package main

func main() {
	data := make(chan int)
	quit := make(chan int)

	select {
	case d <-data:
		fmt.Println(d)
		break;
	case <-quit:
		break;
	}
}
