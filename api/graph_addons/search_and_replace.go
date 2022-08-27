package main

import (
	"bytes"
	"fmt"
	"io/ioutil"
)

func main() {
	input, err := ioutil.ReadFile("graph/model/models_gen.go")
	if err != nil {
		fmt.Println(err)
		return
	}
	output := bytes.Replace(input, []byte("`json:"), []byte("`bson:"), -1)

	if err = ioutil.WriteFile("graph/model/models_gen.go", output, 0666); err != nil {
		fmt.Println(err)
		return
	}
}
