package cmd

import (
	"fmt"

	"github.com/fly-hiring/48318/tester/util"
	"github.com/spf13/cobra"
)

func CreateBalancerCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "balancer",
		Short: "Check if proxy supports load balancing",
		Args:  cobra.ExactArgs(1),
		Run:   handleBalancerCmd,
	}

	cmd.Flags().
		StringSliceP("data", "d", []string{}, "Comma separated data to send to proxy")
	cmd.Flags().
		StringSliceP("expect", "e", []string{}, "Comma separated data to expect from proxy")
	cmd.MarkFlagRequired("data")
	cmd.MarkFlagRequired("expect")
	return cmd
}

func handleBalancerCmd(cmd *cobra.Command, args []string) {
	address := args[0]
	data, _ := cmd.Flags().GetStringSlice("data")
	expect, _ := cmd.Flags().GetStringSlice("expect")

	if len(data) != len(expect) {
		fmt.Println("Field `data` must be of same length with `expect`")
		return
	}

	var reports []*util.GenericReport
	for i := 0; i < len(data); i++ {
		reply, err := util.TcpSendAndReceive(address, data[i])
		if err != nil {
			fmt.Println(err.Error())
			return
		}

		actual := expect[i]
		if string(reply.Response) != actual {
			fmt.Printf(
				"Received response doesn't match expected (%s != %s)\n",
				reply.Response, actual,
			)
			return
		}

		reports = append(reports, reply)
	}

	report := util.PrepareAggregatedReport(reports)
	report.PrettyPrint()
}
