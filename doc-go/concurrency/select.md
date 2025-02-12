# **Documentation : Gestion de canaux concurrents avec `select` en Go**

## **1. Introduction**

L'instruction `select` en Go permet d'attendre plusieurs canaux en **simultané** et de réagir **immédiatement** dès qu'un canal reçoit une donnée.  
C'est **l'équivalent non bloquant d'un `switch`** pour la communication via les `chan`.

---

## **2. Exemple de Code**

### **Fichier : `select.go`**

```go
package main

import (
	"fmt"
	"time"
)

func main() {
	ch1 := make(chan string)
	ch2 := make(chan string)

	// Simule des opérations asynchrones
	go func() {
		time.Sleep(2 * time.Second)
		ch1 <- "Message from channel 1"
	}()

	go func() {
		time.Sleep(1 * time.Second)
		ch2 <- "Message from channel 2"
	}()

	// Utilisation de `select` pour gérer plusieurs canaux
	for i := 0; i < 2; i++ {
		select {
		case msg1 := <-ch1:
			fmt.Println("Received:", msg1)
		case msg2 := <-ch2:
			fmt.Println("Received:", msg2)
		case <-time.After(3 * time.Second): // Timeout de sécurité
			fmt.Println("Timeout: No message received")
		}
	}
}
```

---

## **3. Explication du Code**

### **3.1 Création de deux canaux**

```go
ch1 := make(chan string)
ch2 := make(chan string)
```

- `ch1` et `ch2` sont **deux canaux de type `string`**
- Chaque canal sera utilisé par une goroutine différente

---

### **3.2 Lancement de deux Goroutines**

```go
go func() {
	time.Sleep(2 * time.Second)
	ch1 <- "Message from channel 1"
}()
```

✅ Simule une opération **longue (2s)** puis envoie un message sur `ch1`

```go
go func() {
	time.Sleep(1 * time.Second)
	ch2 <- "Message from channel 2"
}()
```

✅ Simule une opération **plus rapide (1s)** puis envoie un message sur `ch2`

---

### **3.3 Lecture des canaux avec `select`**

```go
for i := 0; i < 2; i++ {
	select {
	case msg1 := <-ch1:
		fmt.Println("Received:", msg1)
	case msg2 := <-ch2:
		fmt.Println("Received:", msg2)
	case <-time.After(3 * time.Second):
		fmt.Println("Timeout: No message received")
	}
}
```

💡 **Décryptage ligne par ligne** :  
✅ **Boucle `for` pour lire deux messages**  
✅ **`select` écoute les trois cas suivants** :

- **Si `ch1` reçoit un message**, on affiche `"Received: Message from channel 1"`
- **Si `ch2` reçoit un message**, on affiche `"Received: Message from channel 2"`
- **Si rien ne se passe après 3 secondes**, un timeout affiche `"Timeout: No message received"`

---

## **4. Résultat Attendu**

```
Received: Message from channel 2
Received: Message from channel 1
```

✅ **Le message du canal 2 arrive avant celui du canal 1 (car `Sleep(1s) < Sleep(2s)`)**

---

## **5. Cas d'Utilisation de `select`**

✅ **Attente simultanée sur plusieurs canaux**  
✅ **Gestion des délais (`time.After`) pour éviter de bloquer indéfiniment**  
✅ **Traitement dynamique de messages provenant de différentes sources**

💡 **Exemple typique : gérer les réponses de plusieurs services en parallèle** ! 🚀
