package util

import (
	"net"
	"time"
)

func TcpSendAndReceive(address string, data string) (*GenericReport, error) {
	dialedAtStart := time.Now()
	conn, err := net.Dial("tcp", address)
	dialedAtEnd := time.Now()
	if err != nil {
		return nil, err
	}

	defer conn.Close()

	// Write message
	dataByte := []byte(data)
	writeAtStart := time.Now()
	_, err = conn.Write(dataByte)
	writeAtEnd := time.Now()
	if err != nil {
		return nil, err
	}

	// Read response
	response := make([]byte, len(dataByte))
	readAtStart := time.Now()
	_, err = conn.Read(response)
	readAtEnd := time.Now()
	if err != nil {
		return nil, err
	}

	dialDuration := dialedAtEnd.Sub(dialedAtStart)
	writeDuration := writeAtEnd.Sub(writeAtStart)
	readDuration := readAtEnd.Sub(readAtStart)
	responseTime := dialDuration + writeDuration + readDuration

	return &GenericReport{
		Response:      response,
		ResponseTime:  responseTime,
		DialDuration:  dialDuration,
		WriteDuration: writeDuration,
		ReadDuration:  readDuration,
	}, nil
}
