package m

import (
	"testing"
)

func TestUnpackFilesFromImageC(t *testing.T) {
	unpack := LoadRustFunc()

	image := []byte("maxwellflitton/nan-one\x00")
	directory := []byte("./cache/two\x00")

	resultPtr := unpack(&image[0], &directory[0])
	if resultPtr == nil {
		t.Fatal("unpack_files_from_image_c returned NULL")
	}
}
