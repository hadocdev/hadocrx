package main

import (
	"os"
	"log"
	"strings"
	"path/filepath"
)

func relativePath(path string) string {
	ownpath, _ := os.Executable()
	return filepath.Join(filepath.Dir(ownpath), path)
}

func makeDir(path string) {
	err := os.Mkdir(path, 0755)
	if err != nil { log.Fatal(err) }
}

func listFiles(path string) []string {
	var files []string
	filepath.Walk(path, func(path string, info os.FileInfo, err error) error {
		if err != nil { return err }
		if !info.IsDir() { files = append(files, path) }
		return nil
	})
	return files
}

func listFilesByExt(path string, ext string) []string {
	var files []string
	filepath.Walk(path, func(path string, info os.FileInfo, err error) error {
		if err != nil { return err }
		if !info.IsDir() && strings.HasSuffix(path, ext) { 
			files = append(files, path) 
		}
		return nil
	})
	return files
}

func directoryExists(path string) bool {
	info, err := os.Stat(path)
	if err != nil { return false }
	return info.IsDir()
}