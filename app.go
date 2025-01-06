package main

import (
	"context"
	"fmt"
)

// App struct
type App struct {
	ctx context.Context
}

// NewApp creates a new App application struct
func NewApp() *App {
	return &App{}
}

// startup is called when the app starts. The context is saved
// so we can call the runtime methods
func (a *App) startup(ctx context.Context) {
	a.ctx = ctx
}

// Greet returns a greeting for the given name
func (a *App) Greet(name string) {
	fmt.Printf("Hello %s, It's show time!\n", name)
}

func (app *App) LoadProfile() []Profile {
	profilesDir := relativePath("./profiles")
	var profiles []Profile
	
	if directoryExists(profilesDir) {
		jsonFiles := listFilesByExt(profilesDir, ".json")
		for _, file := range jsonFiles {
			profiles = append(profiles, LoadFromFile(file))
		}
	} else {
		makeDir(profilesDir)
	}
	return profiles
}

func (app *App) CreateProfile(name, leftinfo, rightinfo, bottominfo string) bool {
	profile := NewProfile(name, leftinfo, rightinfo, bottominfo)
	err := profile.SaveToFile()
	if err != nil { return false }
	return true
}
