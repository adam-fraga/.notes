# Documentation du package `net` en Go

Cette documentation détaille les fonctionnalités du package `net` en Go.  
Elle couvre la création de **serveurs HTTP et TCP**, l'utilisation des **clients HTTP**, ainsi que des exemples d'implémentation.

---

## 📌 Sommaire

- [Introduction](#introduction)
- [Serveur HTTP](#serveur-http)
  - [Démarrage](#démarrage-du-serveur-http)
  - [Middleware de Logging](#middleware-de-logging)
- [Serveur TCP](#serveur-tcp)
  - [Démarrage](#démarrage-du-serveur-tcp)
- [Client HTTP](#client-http)
  - [Envoi de requêtes](#envoi-de-requêtes)
- [Exemples de tests](#exemples-de-tests)
- [Notes et améliorations](#notes-et-améliorations)

---

## 📚 Introduction

Le package `net` de Go permet de gérer les connexions réseau, notamment :

- La création de **serveurs HTTP et TCP**.
- La gestion des **requêtes et réponses HTTP**.
- L'**écoute des connexions TCP** et l'échange de messages.

---

## 🌍 Serveur HTTP

### 📌 Démarrage du serveur HTTP

Le serveur HTTP écoute sur le port **8080** et expose une route `/hello`.

#### 📝 Implémentation :

```go
func startBasicServer() {
    mux := http.NewServeMux()
    mux.HandleFunc("/hello", func(w http.ResponseWriter, r *http.Request) {
        fmt.Fprintf(w, "Hello, World!")
    })

    server := &http.Server{
        Addr:         ":8080",
        Handler:      loggingMiddleware(mux),
        ReadTimeout:  5 * time.Second,
        WriteTimeout: 10 * time.Second,
    }

    fmt.Println("Serveur HTTP démarré sur le port 8080")
    server.ListenAndServe()
}
```

#### ⚡ Explication :

- `http.NewServeMux()` : Crée un routeur HTTP.
- `HandleFunc("/hello")` : Définit une route qui répond `"Hello, World!"`.
- `loggingMiddleware(mux)` : Ajoute un middleware pour loguer les requêtes.
- `ReadTimeout / WriteTimeout` : Définit des limites de temps pour éviter les blocages.

---

### 📌 Middleware de Logging

Ajoute un middleware pour journaliser les requêtes HTTP.

#### 📝 Implémentation :

```go
func loggingMiddleware(next http.Handler) http.Handler {
    return http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
        start := time.Now()
        fmt.Printf("Requête reçue: %s %s\n", r.Method, r.URL.Path)
        next.ServeHTTP(w, r)
        fmt.Printf("Réponse envoyée en %v\n", time.Since(start))
    })
}
```

#### ⚡ Explication :

- Affiche l’URL et la méthode de chaque requête reçue.
- Calcule le temps de traitement avant d’envoyer la réponse.

---

## 🔗 Serveur TCP

### 📌 Démarrage du serveur TCP

Le serveur TCP écoute sur le port **9000** et répond aux connexions entrantes.

#### 📝 Implémentation :

```go
func startTCPServer() {
    listener, err := net.Listen("tcp", ":9000")
    if err != nil {
        fmt.Println("Erreur serveur TCP:", err)
        return
    }
    defer listener.Close()

    fmt.Println("Serveur TCP en écoute sur le port 9000")

    for {
        conn, err := listener.Accept()
        if err != nil {
            fmt.Println("Erreur connexion:", err)
            continue
        }
        go handleTCPConnection(conn)
    }
}
```

#### ⚡ Explication :

- `net.Listen("tcp", ":9000")` : Démarre l’écoute sur le port `9000`.
- `listener.Accept()` : Attend qu’un client se connecte.
- `go handleTCPConnection(conn)` : Gère chaque connexion dans une goroutine.

---

## 📺 Client HTTP

### 📌 Envoi de requêtes

Un client HTTP effectue une requête GET vers `http://example.com`.

#### 📝 Implémentation :

```go
func demonstrateHTTPClient() {
    client := &http.Client{
        Timeout: 5 * time.Second,
    }

    resp, err := client.Get("http://example.com")
    if err != nil {
        fmt.Println("Erreur requête HTTP:", err)
        return
    }
    defer resp.Body.Close()

    body, _ := io.ReadAll(resp.Body)
    fmt.Println("Réponse HTTP:", string(body))
}
```

#### ⚡ Explication :

- `http.Client{Timeout: 5 * time.Second}` : Définit un timeout pour éviter de bloquer.
- `client.Get("http://example.com")` : Envoie une requête GET.
- `io.ReadAll(resp.Body)` : Lit le contenu de la réponse.

---

## 🧪 Exemples de tests

### 📌 Test du serveur HTTP

Lancez le serveur et effectuez une requête avec `curl` :

```sh
curl http://localhost:8080/hello
```

Vous devriez voir :

```sh
Hello, World!
```

### 📌 Test du serveur TCP

Utilisez `nc` (netcat) pour envoyer un message :

```sh
echo "Hello TCP" | nc localhost 9000
```

Le serveur doit répondre.

---

## 📝 Notes et améliorations

- Ajouter **HTTPS** avec `tls.Listen()`.
- Gérer **plusieurs routes HTTP** dynamiquement.
- Optimiser les **logs et la gestion des erreurs**.

---

## 🎯 Conclusion

Cette documentation couvre les bases du package `net` en Go :

- Création de **serveurs HTTP et TCP**.
- Gestion des **requêtes HTTP clients**.
- Démonstration des **middlewares et logs**.
