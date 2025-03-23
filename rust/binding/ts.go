// #include <stddef.h>
// #include <stdlib.h>

package main

import (
	"C"
	"fmt"
	"os"
	"strings"
	"unsafe"

	"github.com/microsoft/typescript-go/internal/ast"
	"github.com/microsoft/typescript-go/internal/binder"
	"github.com/microsoft/typescript-go/internal/bundled"
	ts "github.com/microsoft/typescript-go/internal/compiler"
	"github.com/microsoft/typescript-go/internal/core"
	"github.com/microsoft/typescript-go/internal/diagnosticwriter"
	"github.com/microsoft/typescript-go/internal/parser"
	"github.com/microsoft/typescript-go/internal/printer"
	"github.com/microsoft/typescript-go/internal/scanner"
	"github.com/microsoft/typescript-go/internal/transformers"
	"github.com/microsoft/typescript-go/internal/tspath"
	"github.com/microsoft/typescript-go/internal/vfs/osvfs"
)

//export RunProject
func RunProject(project *C.char, out_diagnostics **C.void, out_length *C.size_t) {
	currentDirectory, err := os.Getwd()
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error getting current directory: %v\n", err)
		return
	}
	configFileName := tspath.ResolvePath(currentDirectory, C.GoString(project))
	fs := bundled.WrapFS(osvfs.FS())
	defaultLibraryPath := bundled.LibPath()
	compilerOptions := &core.CompilerOptions{
		ConfigFilePath: configFileName,
	}
	host := ts.NewCompilerHost(compilerOptions, currentDirectory, fs, defaultLibraryPath)
	program := ts.NewProgram(ts.ProgramOptions{
		ConfigFileName: configFileName,
		Options:        compilerOptions,
		SingleThreaded: false,
		Host:           host,
	})
	compilerOptions = program.Options()
	diagnostics := program.GetConfigFileParsingDiagnostics()
	if len(diagnostics) != 0 {
		printDiagnostics(diagnostics, host, compilerOptions.Pretty.IsFalse())
		return
	}
	result := program.Emit(ts.EmitOptions{})
	if len(result.Diagnostics) != 0 {
		for i, diag := range result.Diagnostics {
			*(**C.void)(unsafe.Pointer(uintptr(unsafe.Pointer(out_diagnostics)) + uintptr(i)*unsafe.Sizeof(uintptr(0)))) =
				(*C.void)(unsafe.Pointer(uintptr(unsafe.Pointer(diag))))
		}
		*out_length = C.size_t(len(result.Diagnostics))
		return
	}
}

//export TranspileModule
func TranspileModule(input *C.char, filename *C.char) *C.char {
	fileName := C.GoString(filename)
	file := parser.ParseSourceFile(fileName, tspath.Path(fileName), C.GoString(input), core.ScriptTargetESNext, scanner.JSDocParsingModeParseNone)

	// Get the Go object from the handle
	compilerOptions := &core.CompilerOptions{
		Target:     core.ScriptTargetLatest,
		ModuleKind: core.ModuleKindESNext,
	}

	ast.SetParentInChildren(file.AsNode())
	emitContext := printer.NewEmitContext()
	resolver := binder.NewReferenceResolver(compilerOptions, binder.ReferenceResolverHooks{})
	file = transformers.NewTypeEraserTransformer(emitContext, compilerOptions).TransformSourceFile(file)
	file = transformers.NewRuntimeSyntaxTransformer(emitContext, compilerOptions, resolver).TransformSourceFile(file)
	// file = transformers.NewESModuleTransformer(emitContext, compilerOptions, resolver).TransformSourceFile(file)
	printer := printer.NewPrinter(
		printer.PrinterOptions{
			NewLine: core.NewLineKindLF,
		},
		printer.PrintHandlers{},
		emitContext,
	)
	text := printer.EmitSourceFile(file)
	return C.CString(text)
}

func printDiagnostics(diagnostics []*ast.Diagnostic, host ts.CompilerHost, pretty bool) {
	formatOpts := getFormatOpts(host)
	if pretty {
		diagnosticwriter.FormatDiagnosticsWithColorAndContext(os.Stdout, diagnostics, formatOpts)
		fmt.Fprintln(os.Stdout)
		diagnosticwriter.WriteErrorSummaryText(os.Stdout, diagnostics, formatOpts)
	} else {
		for _, diagnostic := range diagnostics {
			printDiagnostic(diagnostic, 0, formatOpts.ComparePathsOptions)
		}
	}
}

func printDiagnostic(d *ast.Diagnostic, level int, comparePathOptions tspath.ComparePathsOptions) {
	file := d.File()
	if file != nil {
		p := tspath.ConvertToRelativePath(file.FileName(), comparePathOptions)
		line, character := scanner.GetLineAndCharacterOfPosition(file, d.Loc().Pos())
		fmt.Printf("%v%v(%v,%v): error TS%v: %v\n", strings.Repeat(" ", level*2), p, line+1, character+1, d.Code(), d.Message())
	} else {
		fmt.Printf("%verror TS%v: %v\n", strings.Repeat(" ", level*2), d.Code(), d.Message())
	}
	printMessageChain(d.MessageChain(), level+1)
	for _, r := range d.RelatedInformation() {
		printDiagnostic(r, level+1, comparePathOptions)
	}
}

func printMessageChain(messageChain []*ast.Diagnostic, level int) {
	for _, c := range messageChain {
		fmt.Printf("%v%v\n", strings.Repeat(" ", level*2), c.Message())
		printMessageChain(c.MessageChain(), level+1)
	}
}

func getFormatOpts(host ts.CompilerHost) *diagnosticwriter.FormattingOptions {
	return &diagnosticwriter.FormattingOptions{
		NewLine: host.NewLine(),
		ComparePathsOptions: tspath.ComparePathsOptions{
			CurrentDirectory:          host.GetCurrentDirectory(),
			UseCaseSensitiveFileNames: host.FS().UseCaseSensitiveFileNames(),
		},
	}
}

func main() {}
