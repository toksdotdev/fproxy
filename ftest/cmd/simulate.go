package cmd

import (
	"fmt"
	"sync"

	"github.com/fly-hiring/48318/tester/util"
	"github.com/spf13/cobra"
)

func CreateSimulateCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "simulate",
		Short: "Simulate requests to proxy",
		Args:  cobra.ExactArgs(1),
		Run:   handleLoadCmd,
	}

	cmd.Flags().IntP("concurrency", "c", 50, "Number of concurrent request")
	cmd.Flags().
		StringP("data", "d", "hello world", "Data to send for each request")
	return cmd
}

func handleLoadCmd(cmd *cobra.Command, args []string) {
	address := args[0]
	data, _ := cmd.Flags().GetString("data")
	concurrency, _ := cmd.Flags().GetInt("concurrency")

	var wg sync.WaitGroup
	channel := make(chan util.Result[*util.GenericReport], concurrency)
	for i := 0; i < concurrency; i++ {
		wg.Add(1)
		go sendAndReceive(&wg, channel, address, data)
	}

	wg.Wait()
	close(channel)

	var reports []*util.GenericReport
	for msg := range channel {
		if msg.Err != nil {
			fmt.Println(msg.Err.Error())
			return
		}
		reports = append(reports, msg.Data)
	}

	util.PrepareAggregatedReport(reports).PrettyPrint()
}

func sendAndReceive(
	wg *sync.WaitGroup,
	channel chan util.Result[*util.GenericReport],
	address string,
	data string,
) {
	defer wg.Done()
	reply, err := util.TcpSendAndReceive(address, data)

	if err != nil {
		channel <- util.Result[*util.GenericReport]{Err: err}
		return
	}

	channel <- util.Result[*util.GenericReport]{Data: reply}
}
