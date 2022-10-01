package services

import (
	"fmt"

	"github.com/zrwaite/Insomnizac/db"
	"github.com/zrwaite/Insomnizac/graph/model"
)

func GetSkillArgs(skill *model.Skill) []interface{} {
	filler := ""
	return []interface{}{&filler, &skill.Name, &skill.Image, &filler, &filler}
}

func GetSkills() (skills []*model.Skill, status int) {
	cacheKey := "skills"
	found := db.GetJsonCache(cacheKey, &skills)
	if found {
		return skills, 200
	}

	rows, err := db.DB.Query("SELECT * FROM skills")
	if err != nil {
		return nil, 400
	}
	defer rows.Close()
	for rows.Next() {
		skill := new(model.Skill)
		err = rows.Scan(GetSkillArgs(skill)...)
		if err != nil {
			fmt.Println(err)
			return nil, 400
		}
		skills = append(skills, skill)
	}
	db.SetJsonCache(cacheKey, skills)
	return skills, 200
}
