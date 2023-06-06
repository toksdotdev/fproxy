package cmd

import (
	"os"

	"github.com/spf13/cobra"
)

func Execute() {
	rootCmd := &cobra.Command{
		Use:   "ftest",
		Short: "Testing tool for fproxy",
	}

	rootCmd.AddCommand(CreatePingCmd())
	rootCmd.AddCommand(CreateBalancerCmd())
	rootCmd.AddCommand(CreateSimulateCmd())
	err := rootCmd.Execute()

	if err != nil {
		os.Exit(1)
	}
}
