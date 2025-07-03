package m

import (
	"fmt"
	"log"
	"runtime"

	"github.com/ebitengine/purego"
)

func LoadRustFunc() func(*byte, *byte) *byte {
	var target string

	goos := runtime.GOOS
	goarch := runtime.GOARCH

	switch {
	case goos == "linux" && goarch == "amd64":
		target = "x86_64-unknown-linux-gnu"
	case goos == "linux" && goarch == "arm64":
		target = "aarch64-unknown-linux-gnu"
	case goos == "darwin" && goarch == "arm64":
		target = "aarch64-apple-darwin"
	default:
		panic(fmt.Sprintf("unsupported platform: %s/%s. Only Linux ARM/AMD64 and darwin are supported", goos, goarch))
	}

	libPath := fmt.Sprintf("../../../target/%s/release/libcoredockpack.so", target)

	rustlib, err := purego.Dlopen(libPath, purego.RTLD_NOW|purego.RTLD_GLOBAL)
	if err != nil {
		log.Fatalf("failed to load library: %v", err)
	}
	// defer purego.Dlclose(rustlib)

	var unpackFiles func(*byte, *byte) *byte
	purego.RegisterLibFunc(&unpackFiles, rustlib, "unpack_files_from_image_c")
	return unpackFiles
}
