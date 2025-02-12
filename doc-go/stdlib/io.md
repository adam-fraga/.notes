# Documentation : Utilisation du package `io` en Go

## Introduction

Ce document explore les différentes manières d'utiliser le package `io` de la bibliothèque standard de Go. L'objectif est de montrer des exemples pratiques pour manipuler des entrées/sorties efficacement sans utiliser le package `ioutil` désormais obsolète.

---

## 1. Opérations de base sur les fichiers

### 🔗 Écriture dans un fichier

```go
// Écriture dans un fichier
data := []byte("Hello, IO Package!")
err := os.WriteFile("test.txt", data, 0644)
if err != nil {
    log.Fatal(err)
}
```

### 🔗 Lecture d'un fichier

```go
// Lecture d'un fichier
content, err := os.ReadFile("test.txt")
if err != nil {
    log.Fatal(err)
}
fmt.Printf("Contenu du fichier : %s\n", content)
```

---

## 2. Utilisation des interfaces `io.Reader` et `io.Writer`

```go
// Création d'un lecteur de chaîne
reader := strings.NewReader("Hello, Reader!")

// Création d'un buffer pour l'écriture
var buffer bytes.Buffer

// Copie du lecteur vers le buffer
ios.Copy(&buffer, reader)
fmt.Printf("Buffer : %s\n", buffer.String())
```

---

## 3. Combinaison de plusieurs lecteurs et écrivains

### 🔗 `io.MultiReader`

```go
reader1 := strings.NewReader("Hello ")
reader2 := strings.NewReader("World!")
multiReader := io.MultiReader(reader1, reader2)
```

### 🔗 `io.MultiWriter`

```go
var buffer1, buffer2 bytes.Buffer
multiWriter := io.MultiWriter(&buffer1, &buffer2)
ios.Copy(multiWriter, multiReader)
```

---

## 4. Utilisation de `io.Pipe`

```go
pipeReader, pipeWriter := io.Pipe()

go func() {
    defer pipeWriter.Close()
    data := []byte("Données via pipe!")
    pipeWriter.Write(data)
}()

buffer.Reset()
ios.Copy(&buffer, pipeReader)
fmt.Printf("Contenu du pipe : %s\n", buffer.String())
```

---

## 5. Lecture limitée avec `io.LimitReader`

```go
bigReader := strings.NewReader("Ceci est une longue chaîne de texte.")
limitedReader := io.LimitReader(bigReader, 10)
limitedData, _ := io.ReadAll(limitedReader)
fmt.Printf("Lecture limitée : %s\n", limitedData)
```

---

## 6. Lecteur et écrivain personnalisés avec comptage

```go
type CountingReader struct {
    reader    io.Reader
    bytesRead int64
}

func (cr *CountingReader) Read(p []byte) (n int, err error) {
    n, err = cr.reader.Read(p)
    cr.bytesRead += int64(n)
    return n, err
}
```

---

## 7. Utilisation de `io.TeeReader`

```go
source := strings.NewReader("TeeReader test")
var teeBuffer bytes.Buffer
teeReader := io.TeeReader(source, &teeBuffer)

finalBuffer := make([]byte, 100)
n, _ := teeReader.Read(finalBuffer)
fmt.Printf("TeeReader : %s\n", finalBuffer[:n])
fmt.Printf("Buffer Tee : %s\n", teeBuffer.String())
```

---

## Conclusion

Le package `io` de Go offre des outils puissants pour manipuler les entrées/sorties de manière flexible et efficace. En comprenant ces méthodes, vous pouvez construire des applications plus performantes et mieux optimisées pour la gestion des flux de données.
