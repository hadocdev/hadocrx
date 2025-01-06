package main

import (
	"fmt"
	"os"
	"strings"
	"encoding/json"
)

type Profile struct {
	Name string `json:"name"`
	Leftinfo string `json:"leftinfo"`
	Rightinfo string `json:"rightinfo"`
	Bottominfo string `json:"bottominfo"`
}

func NewProfile(name, leftinfo, rightinfo, bottominfo string) Profile {
	return Profile{
		Name: name,
		Leftinfo: leftinfo,
		Rightinfo: rightinfo,
		Bottominfo: bottominfo,
	}
}

func LoadFromFile(filename string) Profile {
	var profile Profile
	file, err := os.Open(filename)
	if err != nil { 
		fmt.Errorf("Error opening %s: %v\n", filename, err)
		return profile 
	}
	defer file.Close()

	decoder := json.NewDecoder(file)
	err = decoder.Decode(&profile)
	if err != nil { 
		fmt.Errorf("Error decoding profile: %v\n", err)
		return profile
	}
	return profile
}

func (profile *Profile) SaveToFile() error {
	filename := fmt.Sprintf(
		"profiles/profile-%s.json", 
		strings.ReplaceAll(profile.Name, " ", "-"))
	file, err := os.Create(relativePath(filename))
	if err != nil { return err }
	defer file.Close()

	encoder := json.NewEncoder(file)
	err = encoder.Encode(profile)
	if err != nil { return err }
	return nil
}