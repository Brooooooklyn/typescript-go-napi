// tsgo is a test bed for the Go port of TypeScript.
package main

import (
	"C"
	"fmt"
	"os"

	"github.com/microsoft/typescript-go/internal/tspath"
)

//export ResolveTsconfig
func ResolveTsconfig(project *C.char) *C.char {
	currentDirectory, err := os.Getwd()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error getting current directory: %v\n", err)
		return nil
	}
	configFileName := tspath.ResolvePath(currentDirectory, C.GoString(project))
	return C.CString(configFileName)
}

func main() { }