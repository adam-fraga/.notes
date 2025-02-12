# Utilisation avancée du package `bufio` en Go

Le package `bufio` en Go permet d'effectuer des opérations d'entrée/sortie de manière efficace grâce à l'utilisation de buffers. Il est particulièrement utile pour la lecture et l'écriture de fichiers volumineux ou de flux de données.

## 📌 Points clés sur `bufio`

- `bufio.Scanner` est idéal pour lire ligne par ligne ou avec des séparateurs personnalisés.
- `bufio.Writer` réduit les appels système en regroupant les écritures en mémoire tampon.
- Toujours appeler `Flush()` après l'écriture pour s'assurer que toutes les données sont bien enregistrées.
- On peut personnaliser la taille des buffers pour optimiser les performances.
- La gestion des erreurs est cruciale pour éviter des lectures ou écritures partielles.

## 🔍 Exemples d'utilisation

### 1️⃣ Lecture d'un fichier ligne par ligne

```go
file, err := os.Open("sample.txt")
if err != nil {
    fmt.Println("Erreur lors de l'ouverture du fichier:", err)
    return
}
defer file.Close()

scanner := bufio.NewScanner(file)
for scanner.Scan() {
    fmt.Println(scanner.Text()) // Affiche chaque ligne
}
if err := scanner.Err(); err != nil {
    fmt.Println("Erreur de lecture:", err)
}
```

### 2️⃣ Lecture depuis l'entrée standard

```go
reader := bufio.NewReader(os.Stdin)
fmt.Print("Entrez du texte: ")
text, err := reader.ReadString('\n')
if err != nil {
    fmt.Println("Erreur de lecture:", err)
}
fmt.Printf("Vous avez saisi: %s", text)
```

### 3️⃣ Scanner avec un séparateur personnalisé (par mots)

```go
input := strings.NewReader("Ceci est un exemple de lecture mot par mot")
scanner := bufio.NewScanner(input)
scanner.Split(bufio.ScanWords) // Lecture par mots

for scanner.Scan() {
    fmt.Printf("Mot: %s\n", scanner.Text())
}
```

### 4️⃣ Écriture avec `bufio.Writer`

```go
outputFile, err := os.Create("output.txt")
if err != nil {
    fmt.Println("Erreur lors de la création du fichier:", err)
    return
}
defer outputFile.Close()

writer := bufio.NewWriter(outputFile)
_, err = writer.WriteString("Écriture optimisée avec buffer.\n")
if err != nil {
    fmt.Println("Erreur d'écriture:", err)
    return
}
writer.Flush() // Nécessaire pour écrire le buffer dans le fichier
```

### 5️⃣ Scanner avec un buffer personnalisé

```go
largeInput := strings.NewReader("Texte volumineux...")
scanner := bufio.NewScanner(largeInput)
scanner.Buffer(make([]byte, 64*1024), 1024*1024) // 64 KB de buffer, max 1 MB

for scanner.Scan() {
    fmt.Println(scanner.Text())
}
```

### 6️⃣ Lecture jusqu'à un délimiteur spécifique (`ReadBytes`)

```go
func readUntilDelimiter(reader *bufio.Reader, delimiter byte) {
    data, err := reader.ReadBytes(delimiter)
    if err != nil {
        fmt.Println("Erreur de lecture:", err)
        return
    }
    fmt.Printf("Lu jusqu'au délimiteur: %s\n", string(data))
}
```

### 7️⃣ Prévisualisation des prochains octets (`Peek`)

```go
func peekNextBytes(reader *bufio.Reader, n int) {
    bytes, err := reader.Peek(n)
    if err != nil {
        fmt.Println("Erreur de Peek:", err)
        return
    }
    fmt.Printf("Prochains %d octets: %s\n", n, string(bytes))
}
```

## 🆚 Comparaison `bufio` vs `ioutil` / `os`

| Méthode           | Avantages                                         | Inconvénients                        |
| ----------------- | ------------------------------------------------- | ------------------------------------ |
| `bufio.Scanner`   | Lecture ligne par ligne, faible empreinte mémoire | Limité à 64 KB par défaut            |
| `bufio.Reader`    | Lecture efficace de gros fichiers                 | Doit gérer explicitement les buffers |
| `ioutil.ReadFile` | Facile à utiliser, lit tout le fichier en mémoire | Problématique pour les gros fichiers |
| `os.Read`         | Lecture bas niveau, performant                    | Nécessite plus de gestion de code    |

## 🚀 Optimisation avec `sync.Pool`

Si tu travailles avec de très gros fichiers ou des flux de données continus, utiliser `sync.Pool` pour recycler les buffers peut améliorer les performances.

```go
var bufferPool = sync.Pool{
    New: func() interface{} {
        return make([]byte, 64*1024) // 64 KB
    },
}

func readLargeFile(filename string) {
    file, err := os.Open(filename)
    if err != nil {
        fmt.Println("Erreur d'ouverture:", err)
        return
    }
    defer file.Close()

    reader := bufio.NewReader(file)
    buf := bufferPool.Get().([]byte)
    defer bufferPool.Put(buf)

    for {
        n, err := reader.Read(buf)
        if err != nil {
            if err == io.EOF {
                break
            }
            fmt.Println("Erreur de lecture:", err)
            return
        }
        fmt.Print(string(buf[:n]))
    }
}
```

---

Avec ces techniques, tu peux optimiser efficacement la lecture et l'écriture des fichiers en Go ! 🚀🔥
