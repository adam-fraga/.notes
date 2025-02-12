# Guide d'utilisation de gRPC en Go

## Installation des outils requis

Avant de commencer, assurez-vous d'installer les outils nécessaires pour générer le code Go à partir des fichiers `.proto` :

```sh
go install google.golang.org/protobuf/cmd/protoc-gen-go@latest
go install google.golang.org/grpc/cmd/protoc-gen-go-grpc@latest
```

## Génération du code Go à partir du fichier Proto

Utilisez `protoc` pour générer les fichiers Go nécessaires :

```sh
protoc --go_out=. --go_opt=paths=source_relative \
    --go-grpc_out=. --go-grpc_opt=paths=source_relative \
    user.proto
```

## Installation des dépendances

Ajoutez gRPC à votre projet :

```sh
go get google.golang.org/grpc
```

---

## Code du client gRPC en Go

Le client va :

- Se connecter au serveur gRPC
- Envoyer une requête pour créer un utilisateur
- Récupérer les informations de l'utilisateur
- Écouter les mises à jour de l'utilisateur via un flux

### `client/main.go`

```go
package stdlib

import (
	"context"
	"io"
	"log"
	"time"

	"google.golang.org/grpc"
	"google.golang.org/grpc/credentials/insecure"
	pb "userservice/proto"
)

func GRPC() {
	// Connexion au serveur gRPC
	conn, err := grpc.Dial("localhost:50051", grpc.WithTransportCredentials(insecure.NewCredentials()))
	if err != nil {
		log.Fatalf("did not connect: %v", err)
	}
	defer conn.Close()

	client := pb.NewUserServiceClient(conn)
	ctx, cancel := context.WithTimeout(context.Background(), time.Second*10)
	defer cancel()

	// Création d'un utilisateur
	user, err := client.CreateUser(ctx, &pb.CreateUserRequest{
		Name:  "John Doe",
		Email: "john@example.com",
	})
	if err != nil {
		log.Fatalf("could not create user: %v", err)
	}
	log.Printf("Created user: %v", user)

	// Récupération des informations de l'utilisateur
	fetchedUser, err := client.GetUser(ctx, &pb.GetUserRequest{Id: user.Id})
	if err != nil {
		log.Fatalf("could not get user: %v", err)
	}
	log.Printf("Got user: %v", fetchedUser)

	// Écoute des mises à jour de l'utilisateur
	stream, err := client.WatchUser(ctx, &pb.GetUserRequest{Id: user.Id})
	if err != nil {
		log.Fatalf("could not watch user: %v", err)
	}

	for {
		update, err := stream.Recv()
		if err == io.EOF {
			break
		}
		if err != nil {
			log.Fatalf("failed to receive update: %v", err)
		}
		log.Printf("Received user update: %v", update)
	}
}
```

---

## Explication du fonctionnement

1. **Connexion au serveur gRPC**

   - Utilisation de `grpc.Dial` pour se connecter au serveur gRPC.
   - `insecure.NewCredentials()` est utilisé pour une connexion non sécurisée (à ne pas faire en production sans TLS).

2. **Création d'un utilisateur**

   - Envoi d'une requête `CreateUser` avec un nom et un email.

3. **Récupération des données utilisateur**

   - Requête `GetUser` pour récupérer les informations de l'utilisateur créé.

4. **Écoute des mises à jour**
   - Ouverture d'un stream pour recevoir les mises à jour de l'utilisateur.
   - Boucle infinie pour lire les mises à jour jusqu'à la fin du stream.

---

## Conclusion

Ce guide couvre les bases de l'utilisation de gRPC en Go, en mettant en place un client capable d'envoyer des requêtes et d'écouter des mises à jour en temps réel.

Pour aller plus loin :

- Implémentez un serveur gRPC correspondant.
- Sécurisez les connexions avec TLS.
- Gérez les erreurs de manière plus robuste.
