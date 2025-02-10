#### INIT MODULE

```sh
    go mod init adam/hello
```

**(This will create the module file and a go.mod file which is equivalent to package.json, you can then create
your .go file and in top of it specify the package, each go project must start with a main function by convention
we call the package containing this function "main")**

#### COMPILE OR EXECUTE GO FILE(S)

```sh
go build .
go build file.go

#Optimize Build
go build -trimpath -ldflags "-w -s" -o "mybinary"
CGO_ENABLED=0 go build -trimpath -ldflags="-w -s" -o mybinary && upx --best --lzma mybinary

go run .
go run file.go

```

#### COMPILE FOR SPECIFIC OS

```sh
GOOS=windows GOARCH=amd64 go build .
```

#### MANAGE DEPENDENCIES

```sh
go get (Auto fetch dependencies using in project)
go add "my_dep"
```
