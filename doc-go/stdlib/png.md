Je ne peux plus utiliser le canvas dans cette conversation. Mais voici ta documentation en Markdown :

# Documentation: Utilisation du package `image/png` en Go

Ce document illustre comment utiliser le package `image/png` de la bibliothèque standard de Go pour créer, lire et manipuler des images PNG.

---

## 📌 Table des matières

1. [Création et sauvegarde d'une image PNG](#1-création-et-sauvegarde-dune-image-png)
2. [Lecture d'un fichier PNG](#2-lecture-dun-fichier-png)
3. [Encodage PNG avec des paramètres personnalisés](#3-encodage-png-avec-des-paramètres-personnalisés)
4. [Lecture des métadonnées PNG](#4-lecture-des-métadonnées-png)
5. [Création d'un PNG avec transparence](#5-création-dun-png-avec-transparence)
6. [Fonctions utilitaires](#6-fonctions-utilitaires)

---

## 1️⃣ Creation et sauvegarde d'une image PNG

Nous créons une image PNG en utilisant `image.NewRGBA` et dessinons un dégradé basé sur la position des pixels.

```go
width, height := 200, 200
img := image.NewRGBA(image.Rect(0, 0, width, height))

for y := 0; y < height; y++ {
    for x := 0; x < width; x++ {
        img.Set(x, y, color.RGBA{
            R: uint8(x),
            G: uint8(y),
            B: 100,
            A: 255,
        })
    }
}
```

Ensuite, nous enregistrons l'image dans un fichier :

```go
outFile, err := os.Create("gradient.png")
if err != nil {
    log.Fatal("Erreur de création du fichier:", err)
}
defer outFile.Close()

err = png.Encode(outFile, img)
if err != nil {
    log.Fatal("Erreur d'encodage PNG:", err)
}
```

---

## 2️⃣ Lecture d'un fichier PNG

Nous lisons un fichier PNG et affichons ses dimensions :

```go
inFile, err := os.Open("gradient.png")
if err != nil {
    log.Fatal("Erreur d'ouverture du fichier:", err)
}
defer inFile.Close()

decodedImg, err := png.Decode(inFile)
if err != nil {
    log.Fatal("Erreur de décodage PNG:", err)
}

bounds := decodedImg.Bounds()
fmt.Printf("Dimensions de l'image: %dx%d\n", bounds.Max.X, bounds.Max.Y)
```

---

## 3️⃣ Encodage d'une image PNG avec des données personnalisé

Nous utilisons un encodeur avec un niveau de compression spécifique :

```go
customFile, err := os.Create("custom_compression.png")
if err != nil {
    log.Fatal("Erreur de création du fichier:", err)
}
defer customFile.Close()

encoder := &png.Encoder{
    CompressionLevel: png.BestCompression,
}

err = encoder.Encode(customFile, img)
if err != nil {
    log.Fatal("Erreur d'encodage avec compression:", err)
}
```

---

## 4️⃣ Lecture des metadonnées PNG

Nous utilisons un décodeur pour extraire les métadonnées PNG :

```go
metadataFile, err := os.Open("gradient.png")
if err != nil {
    log.Fatal("Erreur d'ouverture du fichier:", err)
}
defer metadataFile.Close()

decoder := png.Decoder{
    ChunkDecoder: png.DefaultChunkDecoder(),
}

_, err = decoder.Decode(metadataFile)
if err != nil {
    log.Fatal("Erreur de décodage des métadonnées:", err)
}
```

---

## 5 Création d'un PNG avec transparence

Nous créons une image transparente avec un motif en damier :

```go
transpImg := image.NewRGBA(image.Rect(0, 0, 100, 100))

for y := 0; y < 100; y++ {
    for x := 0; x < 100; x++ {
        alpha := uint8(255)
        if (x+y)%2 == 0 {
            alpha = 0
        }
        transpImg.Set(x, y, color.RGBA{
            R: 255,
            G: 0,
            B: 0,
            A: alpha,
        })
    }
}

transFile, err := os.Create("transparent.png")
if err != nil {
    log.Fatal("Erreur de création du fichier:", err)
}
defer transFile.Close()

err = png.Encode(transFile, transpImg)
if err != nil {
    log.Fatal("Erreur d'encodage PNG:", err)
}
```

---

## 6️⃣ Fonctions utilitaires

### 🔍 Lire la couleur d'un pixel spécifique

```go
func readPixelAt(img image.Image, x, y int) color.Color {
    return img.At(x, y)
}
```

### 🖼️ Vérifier si un fichier est un PNG

```go
func isPNG(filename string) (bool, error) {
    file, err := os.Open(filename)
    if err != nil {
        return false, err
    }
    defer file.Close()

    signature := make([]byte, 8)
    _, err = file.Read(signature)
    if err != nil {
        return false, err
    }

    return string(signature) == "\x89PNG\r\n\x1a\n", nil
}
```

---

## 🔥 Conclusion

Ce document couvre les bases de l'utilisation du package `image/png` en Go. Il est possible d'étendre ces exemples en manipulant les images de manière plus avancée, comme appliquer des filtres ou convertir des formats.

---
