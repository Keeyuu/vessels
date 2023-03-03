package main

import (
	"fmt"
	"net"
)

var Cconn net.Conn

func main() {
	client_write()
}

func client_write() {
	conn, err := net.Dial("tcp", "127.0.0.1:3737")
	if err != nil {
		fmt.Println(err)
		return
	}
	for {
		buf := ""
		fmt.Scan(&buf)
		fmt.Println("has write", buf)
		conn.Write([]byte(buf))
		if err != nil {
			return
		}
	}
}

func client_read() {
	conn, err := net.Dial("tcp", "127.0.0.1:3737")
	if err != nil {
		fmt.Println(err)
		return
	}
	defer conn.Close()
	buf := make([]byte, 1024)
	for {
		n, err := conn.Read(buf)
		if err != nil {
			return
		}
		fmt.Println(string(buf[:n]))
	}

}

func server() {
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
		if Cconn == nil {
			Cconn = conn
			fmt.Println("get a init")
		} else {
			fmt.Println("not a init")
			buf := make([]byte, 1024)
			for {
				n, err := conn.Read(buf)
				if err != nil {
					Cconn.Close()
					conn.Close()
					Cconn = nil
					break
				}
				Cconn.Write(buf[:n])
			}
		}

	}
}
