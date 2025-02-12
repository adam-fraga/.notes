D'accord, voici ton exemple formaté en Markdown :

````md
# Go Standard Library: HTML Package and Templates

Ce document illustre l'utilisation du package `html` et de ses sous-packages `html/template` et `html/parser` dans la bibliothèque standard de Go.

## 📌 Sommaire

1. [Échappement et déséchappement HTML](#échappement-et-déséchappement-html)
2. [Templates avec `html/template`](#templates-avec-htmltemplate)
3. [Utilisation de fichiers de templates multiples](#utilisation-de-fichiers-de-templates-multiples)
4. [Contenu HTML sûr et non sécurisé](#contenu-html-sûr-et-non-sécurisé)
5. [Serveur HTTP avec templates](#serveur-http-avec-templates)
6. [Échappement d'URL](#échappement-durl)
7. [Héritage de templates](#héritage-de-templates)

---

## 1️⃣ Échappement et déséchappement HTML

Le package `html` permet d'échapper et de déséchapper les chaînes HTML.

```go
package main

import (
	"fmt"
	"html"
)

func main() {
	rawString := `<script>alert("XSS attack!");</script>`
	escapedString := html.EscapeString(rawString)
	fmt.Printf("Original: %s\n", rawString)
	fmt.Printf("Escaped:  %s\n", escapedString)

	escapedText := "&lt;div&gt;Hello, World!&lt;/div&gt;"
	unescapedText := html.UnescapeString(escapedText)
	fmt.Printf("Escaped:   %s\n", escapedText)
	fmt.Printf("Unescaped: %s\n", unescapedText)
}
```
````

---

## 2️⃣ Templates avec `html/template`

Le package `html/template` est utilisé pour générer du HTML sécurisé.

```go
package main

import (
	"html/template"
	"log"
	"os"
	"strings"
)

type PageData struct {
	Title       string
	Message     string
	Items       []string
	HTMLContent string
}

// Définition des fonctions personnalisées pour le template
var funcMap = template.FuncMap{
	"lower": strings.ToLower,
	"upper": strings.ToUpper,
}

const templateString = `
<!DOCTYPE html>
<html>
<head>
    <title>{{.Title}}</title>
</head>
<body>
    <h1>{{.Title}}</h1>
    <p>{{.Message}}</p>
    <ul>
    {{range .Items}}
        <li>{{.}}</li>
    {{end}}
    </ul>
    {{if .HTMLContent}}
        <div>{{.HTMLContent}}</div>
    {{end}}
</body>
</html>`

func main() {
	tmpl, err := template.New("page").Funcs(funcMap).Parse(templateString)
	if err != nil {
		log.Fatal(err)
	}

	data := PageData{
		Title:       "HTML Template Demo",
		Message:     "Bienvenue dans Go Templates!",
		Items:       []string{"Item 1", "Item 2", "Item 3"},
		HTMLContent: "<p>Ce HTML sera échappé</p>",
	}

	file, err := os.Create("output.html")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	err = tmpl.Execute(file, data)
	if err != nil {
		log.Fatal(err)
	}
}
```

---

## 3️⃣ Utilisation de fichiers de templates multiples

Il est possible de structurer ses templates en plusieurs fichiers.

```go
package main

import (
	"html/template"
	"log"
)

func main() {
	templates := template.New("templates")
	templates = template.Must(templates.Parse(`
		{{define "header"}}<header><h1>{{.}}</h1></header>{{end}}
		{{define "footer"}}<footer><p>{{.}}</p></footer>{{end}}
	`))
}
```

---

## 4️⃣ Contenu HTML sûr et non sécurisé

L'utilisation du type `template.HTML` permet de stocker du contenu HTML sûr.

```go
package main

import (
	"fmt"
	"html/template"
)

func main() {
	safeContent := template.HTML("<p>Contenu HTML sûr</p>")
	fmt.Printf("Type de contenu sécurisé: %T\n", safeContent)
}
```

---

## 5️⃣ Serveur HTTP avec templates

Un serveur HTTP qui génère des pages dynamiques avec des templates.

```go
package main

import (
	"html/template"
	"log"
	"net/http"
)

type PageData struct {
	Title   string
	Message string
}

func handler(w http.ResponseWriter, r *http.Request) {
	tmpl := template.Must(template.New("page").Parse(`
		<html>
			<head><title>{{.Title}}</title></head>
			<body>
				<h1>{{.Message}}</h1>
			</body>
		</html>
	`))

	data := PageData{
		Title:   "Page Dynamique",
		Message: "Hello depuis un handler HTTP!",
	}

	err := tmpl.Execute(w, data)
	if err != nil {
		http.Error(w, err.Error(), http.StatusInternalServerError)
	}
}

func main() {
	http.HandleFunc("/", handler)
	log.Println("Serveur démarré sur :8080")
	log.Fatal(http.ListenAndServe(":8080", nil))
}
```

---

## 6️⃣ échappement d'URL

Lors de l'injection de contenu dynamique dans des liens, il est important d'échapper les valeurs.

```go
package main

import (
	"html/template"
	"os"
)

func main() {
	urlTemplate := template.Must(template.New("url").Parse(`
		<a href="/search?q={{.}}">Rechercher</a>
	`))

	searchTerm := "hello world & more"
	urlTemplate.Execute(os.Stdout, searchTerm)
}
```

---

## 7️⃣ Héritage de templates

L'héritage de templates permet de structurer ses fichiers avec des layouts.

```go
package main

import (
	"html/template"
	"log"
)

func templateInheritance() {
	baseTemplate := `
		{{define "base"}}
		<!DOCTYPE html>
		<html>
			<head>
				<title>{{template "title" .}}</title>
			</head>
			<body>
				{{template "content" .}}
			</body>
		</html>
		{{end}}
	`

	pageTemplate := `
		{{define "title"}}Ma Page{{end}}
		{{define "content"}}
		<h1>Bienvenue</h1>
		<p>Contenu ici</p>
		{{end}}
	`

	tmpl := template.Must(template.New("base").Parse(baseTemplate))
	template.Must(tmpl.Parse(pageTemplate))
}

func main() {
	templateInheritance()
	log.Println("Template inheritance loaded")
}
```

---

## 🚀 Conclusion

Ce guide montre comment utiliser `html` et `html/template` pour générer du contenu HTML en toute sécurité en Go. Il est essentiel d'échapper les entrées utilisateurs et d'utiliser des templates pour séparer la logique de l'affichage.
