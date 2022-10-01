package utils

import "github.com/zrwaite/Insomnizac/graph/model"

type SortType interface {
	int | *model.Skill
}

func GenericBinarySearch[V SortType](list []V, item V, gt func(a, b V) bool, lt func(a, b V) bool) (found bool, index int) {
	startIndex := 0
	endIndex := len(list) - 1
	if len(list) == 0 {
		return false, 0
	}
	for {
		middleIndex := (startIndex + endIndex) / 2
		middleItem := list[middleIndex]
		// fmt.Println("Looking for " + item + ", found " + middleItem + " at " + strconv.Itoa(middleIndex) + " between " + strconv.Itoa(startIndex) + " and " + strconv.Itoa(endIndex))
		if lt(middleItem, item) {
			if startIndex == endIndex {
				return false, startIndex + 1
			}
			startIndex = middleIndex + 1
		} else if gt(middleItem, item) {
			if startIndex == endIndex {
				return false, startIndex
			}
			endIndex = middleIndex - 1
		} else {
			return true, middleIndex
		}
	}
}

func CompareSkillGT(a, b *model.Skill) bool {
	return a.ID > b.ID
}

func CompareSkillLT(a, b *model.Skill) bool {
	return a.ID < b.ID
}

func SkillBinarySearch(list []*model.Skill, skill *model.Skill) (found bool, index int) {
	return GenericBinarySearch(list, skill, CompareSkillGT, CompareSkillLT)
}
