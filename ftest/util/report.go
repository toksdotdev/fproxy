package util

import (
	"fmt"
	"time"
)

type Result[T any] struct {
	Data T
	Err  error
}

type GenericReport struct {
	Response      []byte
	ResponseTime  time.Duration
	DialDuration  time.Duration
	WriteDuration time.Duration
	ReadDuration  time.Duration
}

func (c *GenericReport) PrettyPrint() {
	fmt.Printf("Response\t: %s\n", string(c.Response))
	fmt.Printf("Dial time\t: %s\n", c.DialDuration)
	fmt.Printf("Write time\t: %s\n", c.WriteDuration)
	fmt.Printf("Read time\t: %s\n", c.ReadDuration)
	fmt.Printf("Response time\t: %s\n", c.ResponseTime)
}

type AggregatedReport struct {
	TotalRequests   int
	AvgResponseTime time.Duration
	AvgDialTime     time.Duration
	AvgWriteTime    time.Duration
	AvgReadTime     time.Duration
}

func (c *AggregatedReport) PrettyPrint() {
	fmt.Printf("Request count\t\t: %d\n", c.TotalRequests)
	fmt.Printf("Avg. dial time\t\t: %s\n", c.AvgDialTime)
	fmt.Printf("Avg. write time\t\t: %s\n", c.AvgWriteTime)
	fmt.Printf("Avg. read time\t\t: %s\n", c.AvgReadTime)
	fmt.Printf("Avg. response time\t: %s\n", c.AvgResponseTime)
}

func PrepareAggregatedReport(reports []*GenericReport) *AggregatedReport {
	count := len(reports)
	total := &AggregatedReport{TotalRequests: count}

	for _, report := range reports {
		total.AvgDialTime += divideDuration(report.DialDuration, count)
		total.AvgReadTime += divideDuration(report.ReadDuration, count)
		total.AvgResponseTime += divideDuration(report.ResponseTime, count)
		total.AvgWriteTime += divideDuration(report.WriteDuration, count)
	}

	return total
}

func divideDuration(duration time.Duration, value int) time.Duration {
	return time.Duration(duration.Nanoseconds() / int64(value))
}
