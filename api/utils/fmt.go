package utils

import (
	"fmt"
	"reflect"
)

func IterateStruct(obj interface{}) {
	v := reflect.ValueOf(obj)
	typeOfS := v.Type()

	for i := 0; i < v.NumField(); i++ {
		fmt.Printf("Field: %s\tValue: %v\n", typeOfS.Field(i).Name, v.Field(i).Interface())
	}
}
