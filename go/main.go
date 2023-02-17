package main

import (
	"fmt"
	"net"
)

func main() {
	l, err := net.Listen("tcp", ":3737")
	if err != nil {
		panic(err)
	}
	for {
		fmt.Println("start listen")
		conn, err := l.Accept()
		if err != nil {
			fmt.Println("get a err")
			fmt.Println(err)
			continue
		}
		fmt.Printf("get a conn :%s \n", conn.RemoteAddr())
	}
}

func server() {
	
}
