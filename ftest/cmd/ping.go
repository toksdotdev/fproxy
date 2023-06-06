package cmd

import (
	"fmt"

	"github.com/fly-hiring/48318/tester/util"
	"github.com/spf13/cobra"
)

func CreatePingCmd() *cobra.Command {
	cmd := &cobra.Command{
		Use:   "ping",
		Short: "Send ping data to proxy (supports IPv4 and IPv6)",
		Args:  cobra.ExactArgs(1),
		Run:   handlePingCmd,
	}

	cmd.Flags().StringP("data", "d", "", "Data to send to proxy")
	return cmd
}

func handlePingCmd(cmd *cobra.Command, args []string) {
	address := args[0]
	data, err := cmd.Flags().GetString("data")
	if err != nil {
		fmt.Println(err.Error())
		return
	}

	reply, err := util.TcpSendAndReceive(address, data)
	if err != nil {
		fmt.Println(err.Error())
		return
	}

	reply.PrettyPrint()
}
