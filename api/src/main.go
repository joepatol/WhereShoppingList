package main;

const CONN_URL = "postgres://postgresuser:postgrespwd@localhost:5432/supermarkt";

type MyStruct struct {
	a float32
	b float32
	c float32
}

func structMethod(s MyStruct) float32 {
	return s.c
}

func main() {
	var x float32 = 3.0

	var y float32 = 3.3

	var z = add(x, y)
	
	var theStruct = MyStruct{a: x, b: y, c: z}

	println(structMethod(theStruct))
}

func add(a float32, b float32) float32 {
	return a + b
}