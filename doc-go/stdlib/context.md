Voici ton code formaté en Markdown ! 🚀

````md
# Utilisation du package `context` en Go avec un serveur HTTP

Ce document montre comment utiliser le package `context` en Go dans un serveur HTTP.  
Le package `context` est utilisé pour gérer les délais, les annulations et les valeurs associées à une requête entre les processus et les appels API.

## Concepts clés démontrés :

- Toujours utiliser `defer cancel()` pour éviter les fuites de contexte
- Les valeurs de contexte sont propagées dans toute la chaîne
- Plusieurs goroutines peuvent partager le même contexte
- Différents types de résiliation de contexte (timeout, deadline, annulation)
- Bonnes pratiques de gestion des erreurs dans les gestionnaires HTTP

### Bonnes pratiques d'utilisation du contexte :

- Le contexte doit être le premier paramètre d'une fonction
- Ne jamais stocker un contexte dans une struct
- Le type de clé doit être non exporté pour éviter les collisions
- Toujours propager le contexte lors d'appels descendants
- Utiliser les valeurs de contexte avec parcimonie, principalement pour des données liées à la requête

---

## 📌 Code Go

```go
package stdlib

import (
	"context"
	"fmt"
	"html/template"
	"net/http"
	"time"
)

// Simule une requête à une base de données avec un délai
func SimulateDBQuery(ctx context.Context) (string, error) {
	select {
	case <-time.After(2 * time.Second):
		return "Query Result", nil
	case <-ctx.Done():
		return "", ctx.Err()
	}
}

// Simule un worker effectuant une tâche en arrière-plan
func Worker(ctx context.Context, id int) {
	for {
		select {
		case <-ctx.Done():
			fmt.Printf("Worker %d: Stopped due to context cancellation\n", id)
			return
		default:
			fmt.Printf("Worker %d: Working...\n", id)
			time.Sleep(time.Second)
		}
	}
}

// Gestionnaire HTTP utilisant un timeout avec context
func HandleWithTimeout(w http.ResponseWriter, r *http.Request) {
	ctx, cancel := context.WithTimeout(r.Context(), 3*time.Second)
	defer cancel()

	result, err := SimulateDBQuery(ctx)
	if err != nil {
		http.Error(w, "Query failed: "+err.Error(), http.StatusGatewayTimeout)
		return
	}

	fmt.Fprintf(w, "Query succeeded: %s", result)
}

// Gestionnaire lançant un worker qui s'arrête si la requête est annulée
func HandleWorker(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()
	go Worker(ctx, 1)

	fmt.Fprintln(w, "Worker started. Try canceling the request.")
}

// Gestionnaire affichant un template HTML en utilisant des valeurs du contexte
func HandleTemplateRendering(w http.ResponseWriter, r *http.Request) {
	tmpl := `<h1>Hello, {{.Name}}</h1><p>Request ID: {{.RequestID}}</p>`

	ctx := r.Context()
	name := ctx.Value("userName")
	if name == nil {
		name = "Guest"
	}

	requestID := ctx.Value("requestID")
	if requestID == nil {
		requestID = "unknown"
	}

	t, _ := template.New("webpage").Parse(tmpl)
	t.Execute(w, map[string]string{
		"Name":      name.(string),
		"RequestID": requestID.(string),
	})
}

// Gestionnaire injectant des valeurs dans le contexte avant de rendre un template
func HandleWithValues(w http.ResponseWriter, r *http.Request) {
	ctx := r.Context()

	ctx = context.WithValue(ctx, "userName", "Alice")
	ctx = context.WithValue(ctx, "requestID", "req-12345")

	HandleTemplateRendering(w, r.WithContext(ctx))
}

// Fonction principale démarrant un serveur HTTP avec différents gestionnaires
func mainFunc() {
	mux := http.NewServeMux()

	mux.HandleFunc("/timeout", HandleWithTimeout)
	mux.HandleFunc("/worker", HandleWorker)
	mux.HandleFunc("/template", HandleTemplateRendering)
	mux.HandleFunc("/context-values", HandleWithValues)

	server := &http.Server{
		Addr:    ":8080",
		Handler: mux,
	}

	fmt.Println("Starting server on :8080")
	server.ListenAndServe()
}
```
````

## 🔥 Explications des endpoints

| Endpoint          | Description                                                                              |
| ----------------- | ---------------------------------------------------------------------------------------- |
| `/timeout`        | Exécute une requête avec `context.WithTimeout()`, annulant la requête si elle dépasse 3s |
| `/worker`         | Lance un worker qui s'arrête si la requête est annulée                                   |
| `/template`       | Affiche un template HTML utilisant des valeurs du contexte                               |
| `/context-values` | Injecte des valeurs dans le contexte avant d'appeler `/template`                         |
