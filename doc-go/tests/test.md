# **Documentation : Tests en Go** 🧪

Go possède un framework de test intégré qui permet d'écrire et d'exécuter des tests unitaires facilement.

---

## **1. Structure d'un test Go**

Un test en Go se place dans un fichier **`xxx_test.go`** et utilise le package **`testing`**.  
Chaque test est une **fonction qui commence par `Test`** et prend un paramètre **`t *testing.T`**.

### **Exemple simple de test**

```go
package main

import (
	"testing"
)

// Fonction à tester
func Add(a, b int) int {
	return a + b
}

// Test
func TestAdd(t *testing.T) {
	result := Add(2, 3)
	expected := 5

	if result != expected {
		t.Errorf("Add(2, 3) = %d; want %d", result, expected)
	}
}
```

✅ **Lancer le test** :

```sh
go test
```

Si tout est bon, aucun message ne s'affiche. En cas d'échec, une erreur détaillée apparaît.

---

## **2. Table-Driven Tests (Tests paramétrés)**

Go recommande d'utiliser **des tests basés sur des tableaux de cas de test**.

```go
func TestAddTableDriven(t *testing.T) {
	tests := []struct {
		a, b     int
		expected int
	}{
		{1, 2, 3},
		{0, 0, 0},
		{-1, 1, 0},
		{10, 5, 15},
	}

	for _, tt := range tests {
		result := Add(tt.a, tt.b)
		if result != tt.expected {
			t.Errorf("Add(%d, %d) = %d; want %d", tt.a, tt.b, result, tt.expected)
		}
	}
}
```

✅ **Avantages** :  
✔️ Moins de répétition  
✔️ Facile à étendre

---

## **3. Tester les erreurs (`t.Fatal`, `t.FailNow`)**

Si un test doit **s'arrêter immédiatement**, utilise `t.Fatal`.

```go
func TestDivide(t *testing.T) {
	result, err := Divide(10, 0)
	if err == nil {
		t.Fatal("Expected an error but got nil")
	}
}
```

💡 **Différence entre `t.Error` et `t.Fatal`** :

- **`t.Error`** → Affiche l'erreur et continue le test.
- **`t.Fatal`** → Arrête immédiatement le test.

---

## **4. Benchmarks en Go 🏎️**

Go permet aussi de **mesurer les performances** avec des **benchmarks**.

### **Exemple : Benchmark de `Add`**

```go
func BenchmarkAdd(b *testing.B) {
	for i := 0; i < b.N; i++ {
		Add(2, 3)
	}
}
```

✅ **Exécuter un benchmark** :

```sh
go test -bench=.
```

---

## **5. Mocks avec `testing.T`**

On peut simuler des comportements en utilisant des **fonctions anonymes**.

```go
type Service struct{}

func (s *Service) FetchData() string {
	return "real data"
}

// Test avec mock
func TestService_FetchData(t *testing.T) {
	mockService := &Service{}
	mockService.FetchData = func() string { return "mocked data" }

	if mockService.FetchData() != "mocked data" {
		t.Errorf("Expected mocked data but got %s", mockService.FetchData())
	}
}
```

---

## **6. Couverture de code 📊**

Pour voir **le pourcentage de code testé**, utilise :

```sh
go test -cover
```

Tu peux aussi générer un **rapport détaillé** :

```sh
go test -coverprofile=coverage.out
go tool cover -html=coverage.out
```

👉 **Cela ouvre un rapport HTML interactif**.

---

## **Résumé**

| Fonction           | Description                             |
| ------------------ | --------------------------------------- |
| `t.Errorf(...)`    | Affiche une erreur sans stopper le test |
| `t.Fatal(...)`     | Arrête immédiatement le test            |
| `b.N`              | Nombre d'itérations dans un benchmark   |
| `go test`          | Exécute tous les tests                  |
| `go test -bench=.` | Exécute les benchmarks                  |
| `go test -cover`   | Vérifie la couverture du code           |

Go facilite l'écriture et l'exécution des tests, ce qui est essentiel pour un **code robuste et maintenable**. 🚀
