# **Documentation : Gestion des Mutex en Go avec `sync.Mutex`**

## **1. Introduction**

Cette documentation couvre l'utilisation de **`sync.Mutex`** en Go pour gérer l'accès concurrentiel à une ressource partagée.

Nous allons voir :

- **Comment protéger une variable partagée** contre les **conditions de course**
- **Pourquoi utiliser un `mutex`** pour synchroniser l'accès à la donnée
- **Un exemple de code avec 10 goroutines** incrémentant une valeur de manière sécurisée

---

## **2. Exemple de Code**

### **Fichier : `mutex.go`**

```go
package main

import (
	"fmt"
	"sync"
)

// Counter est une structure qui contient une valeur partagée et un mutex
type Counter struct {
	mu    sync.Mutex // Mutex pour protéger l'accès concurrentiel
	value int        // Ressource partagée
}

// Increment augmente la valeur du compteur en garantissant un accès sécurisé
func (c *Counter) Increment() {
	c.mu.Lock()         // Verrouillage avant modification
	defer c.mu.Unlock() // Déverrouillage après l'opération
	c.value++
}

// GetValue retourne la valeur actuelle du compteur de manière sécurisée
func (c *Counter) GetValue() int {
	c.mu.Lock()
	defer c.mu.Unlock()
	return c.value
}

func main() {
	var wg sync.WaitGroup
	counter := Counter{}

	// Lancer 10 goroutines qui incrémentent le compteur
	for i := 0; i < 10; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			counter.Increment()
		}()
	}

	wg.Wait() // Attendre que toutes les goroutines aient terminé
	fmt.Println("Final Counter Value:", counter.GetValue())
}
```

---

## **3. Explication du Code**

### **3.1 Pourquoi utiliser un `sync.Mutex` ?**

Go exécute les goroutines de manière concurrente.  
Si plusieurs goroutines modifient **simultanément** une variable **sans protection**, on peut avoir :

- **Des conditions de course (`race conditions`)**
- **Une corruption des données**

Le **mutex (`sync.Mutex`)** permet de garantir qu'une seule goroutine accède à la ressource partagée à la fois.

---

### **3.2 Structure du Compteur (`Counter`)**

```go
type Counter struct {
	mu    sync.Mutex
	value int
}
```

- `mu` : **Mutex** utilisé pour bloquer/débloquer l'accès
- `value` : **Compteur partagé** modifié par plusieurs goroutines

---

### **3.3 Méthodes Sécurisées**

#### 🔹 **Méthode `Increment()`**

```go
func (c *Counter) Increment() {
	c.mu.Lock()         // 🔒 Verrouille l'accès au compteur
	defer c.mu.Unlock() // 🔓 Déverrouille après l'opération
	c.value++           // Modification de la variable partagée
}
```

✅ **Empêche plusieurs goroutines de modifier `value` en même temps**  
✅ **Le `defer` garantit que le mutex est toujours libéré, même en cas d'erreur**

---

#### 🔹 **Méthode `GetValue()`**

```go
func (c *Counter) GetValue() int {
	c.mu.Lock()         // 🔒 Empêche la lecture concurrente
	defer c.mu.Unlock() // 🔓 Libère après récupération de la valeur
	return c.value
}
```

✅ **Lecture sécurisée de la valeur actuelle**  
✅ **Évite que la valeur soit modifiée pendant la lecture**

---

### **3.4 Création et Gestion des Goroutines**

```go
var wg sync.WaitGroup
counter := Counter{}
```

- `wg` est utilisé pour attendre la fin des goroutines
- `counter` est notre **variable partagée**

#### **Lancement des Goroutines**

```go
for i := 0; i < 10; i++ {
	wg.Add(1) // On signale l'ajout d'une goroutine

	go func() {
		defer wg.Done() // On signale la fin de la goroutine
		counter.Increment()
	}()
}
```

✅ **Chaque goroutine incrémente la valeur du compteur de manière concurrente**  
✅ **Le `wg.Done()` permet de s’assurer que toutes les goroutines se terminent**

#### **Attente de la fin des Goroutines**

```go
wg.Wait() // On attend que toutes les goroutines aient terminé
```

✅ **Assure que le programme ne se termine pas avant la fin des goroutines**

#### **Affichage du Résultat**

```go
fmt.Println("Final Counter Value:", counter.GetValue())
```

✅ **Affiche la valeur finale après toutes les incrémentations**

---

## **4. Résultat Attendu**

```
Final Counter Value: 10
```

💡 **Même avec l'exécution concurrente, la valeur finale est toujours correcte grâce au Mutex !** 🚀

---

## **5. Pourquoi utiliser `sync.Mutex` au lieu d'un `chan` ?**

### **Cas où `sync.Mutex` est préférable :**

✅ **Protéger une structure de données modifiable (`Counter`, `Map`, `Slice`, etc.)**  
✅ **Réduction de la surcharge mémoire (pas de buffering comme avec `chan`)**  
✅ **Simplicité et performance pour des accès concurrents rapides**

### **Cas où `chan` est préférable :**

✅ **Modèle "Producteur-Consommateur" où plusieurs goroutines échangent des messages**  
✅ **Évite le partage de mémoire, favorise la communication par passage de messages**

💡 **En général, `sync.Mutex` est plus performant quand l'accès aux données est fréquent.**

---

## **6. Améliorations Possibles**

🔹 **Remplacer `sync.Mutex` par `sync.RWMutex`** si on a plus de lectures que d'écritures  
🔹 **Utiliser `atomic.AddInt64`** si l'on veut une solution ultra-rapide et sécurisée

---

## **7. Conclusion**

Cet exemple montre comment **protéger un accès concurrentiel en Go** en utilisant `sync.Mutex`.  
C'est une **solution simple et efficace** pour éviter les **conditions de course** et garantir l'intégrité des données. 🚀🔥
