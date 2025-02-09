// This example demonstrates the usage of the html package and its subpackages
// (html/template and html/parser) from Go's standard library.

package stdlib

import (
	"fmt"
	"html"
	"html/template"
	"log"
	"net/http"
	"os"
	"strings"
)

// PageData represents the data structure for our template
type PageData struct {
	Title       string
	Message     string
	Items       []string
	HTMLContent string
}

// Define template functions
var funcMap = template.FuncMap{
	"lower": strings.ToLower,
	"upper": strings.ToUpper,
}

// Example template string
const templateString = `
<!DOCTYPE html>
<html>
<head>
    <title>{{.Title}}</title>
</head>
<body>
    <h1>{{.Title}}</h1>
    <p>{{.Message}}</p>
    
    <!-- Demonstrating template functions -->
    <p>Lowercase title: {{lower .Title}}</p>
    <p>Uppercase title: {{upper .Title}}</p>
    
    <!-- Demonstrating loops -->
    <ul>
    {{range .Items}}
        <li>{{.}}</li>
    {{end}}
    </ul>

    <!-- Demonstrating conditional -->
    {{if .HTMLContent}}
        <!-- Using the html/template package's automatic escaping -->
        <div>{{.HTMLContent}}</div>
    {{end}}
</body>
</html>`

func main() {
	fmt.Println("HTML Package Examples")
	fmt.Println("--------------------")

	// Example 1: Basic HTML escaping using html package
	fmt.Println("\n1. Basic HTML Escaping:")
	rawString := `<script>alert("XSS attack!");</script>`
	escapedString := html.EscapeString(rawString)
	fmt.Printf("Original: %s\n", rawString)
	fmt.Printf("Escaped:  %s\n", escapedString)

	// Example 2: HTML unescaping
	fmt.Println("\n2. HTML Unescaping:")
	escapedText := "&lt;div&gt;Hello, World!&lt;/div&gt;"
	unescapedText := html.UnescapeString(escapedText)
	fmt.Printf("Escaped:   %s\n", escapedText)
	fmt.Printf("Unescaped: %s\n", unescapedText)

	// Example 3: Using html/template package
	fmt.Println("\n3. HTML Template Example:")

	// Create template with custom functions
	tmpl, err := template.New("page").Funcs(funcMap).Parse(templateString)
	if err != nil {
		log.Fatal(err)
	}

	// Prepare data for the template
	data := PageData{
		Title:       "HTML Package Demo",
		Message:     "Welcome to Go HTML package demonstration",
		Items:       []string{"Item 1", "Item 2", "Item 3"},
		HTMLContent: "<p>This HTML will be safely escaped</p><script>alert('test');</script>",
	}

	// Execute template and write to file
	file, err := os.Create("output.html")
	if err != nil {
		log.Fatal(err)
	}
	defer file.Close()

	err = tmpl.Execute(file, data)
	if err != nil {
		log.Fatal(err)
	}

	// Example 4: Template with multiple files
	fmt.Println("\n4. Multiple Template Files:")

	// Create a template set
	templates := template.New("templates")
	templates = template.Must(templates.Parse(`
		{{define "header"}}
		<header><h1>{{.}}</h1></header>
		{{end}}
		
		{{define "footer"}}
		<footer><p>{{.}}</p></footer>
		{{end}}
	`))

	// Example 5: Safe and unsafe HTML content
	fmt.Println("\n5. Safe vs Unsafe HTML Content:")

	// Creating safe HTML content
	safeContent := template.HTML("<p>This is trusted HTML</p>")
	fmt.Printf("Safe HTML content type: %T\n", safeContent)

	// Example 6: HTTP handler with template
	http.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		// Create a new template for each request (best practice)
		tmpl := template.Must(template.New("page").Parse(`
			<html>
				<head><title>{{.Title}}</title></head>
				<body>
					<h1>{{.Message}}</h1>
				</body>
			</html>
		`))

		data := PageData{
			Title:   "Dynamic Page",
			Message: "Hello from HTTP handler!",
		}

		err := tmpl.Execute(w, data)
		if err != nil {
			http.Error(w, err.Error(), http.StatusInternalServerError)
		}
	})

	// Example 7: URL escaping in templates
	urlTemplate := template.Must(template.New("url").Parse(`
		<a href="/search?q={{.}}">Search</a>
	`))

	fmt.Println("\n7. URL Template Example:")
	searchTerm := "hello world & more"
	urlTemplate.Execute(os.Stdout, searchTerm)
}

// Helper function to demonstrate template inheritance
func templateInheritance() {
	// Base template
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

	// Page template
	pageTemplate := `
		{{define "title"}}My Page{{end}}
		{{define "content"}}
		<h1>Welcome</h1>
		<p>Content goes here</p>
		{{end}}
	`

	// Parse both templates
	tmpl := template.Must(template.New("base").Parse(baseTemplate))
	template.Must(tmpl.Parse(pageTemplate))
}
