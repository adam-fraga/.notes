# **Documentation : Collections en Go (Slices, Maps, Structs)**

## **1. Introduction**

Go propose plusieurs structures de données pour stocker et manipuler des collections :  
✅ **Tableaux (`array`)** – Taille fixe  
✅ **Slices (`slice`)** – Taille dynamique  
✅ **Maps (`map`)** – Tableaux associatifs  
✅ **Structures (`struct`)** – Objets personnalisés

---

## **2. Tableaux (`arrays`)**

```go
table_one := [5]int{1, 1, 1, 1, 5} // Array de taille 5
table_two := table_one             // Copie du tableau (pas une référence)
table_two[1] = 12
```

💡 **Remarque :**

- **Les tableaux ont une taille fixe** et ne peuvent pas être redimensionnés.
- **L'affectation `=` crée une copie complète du tableau.**

---

## **3. Slices (`slices`)**

Un **slice** est une vue dynamique sur un tableau. Contrairement aux tableaux, ils **peuvent être redimensionnés**.

```go
table_four := make([]int, 5)    // Slice avec 5 éléments initialisés à 0
table_five := make([]int, 0, 5) // Slice vide avec capacité de 5

slice_reference := table_two[1:] // Slice basé sur `table_two`
manual_slice := []int{1, 2, 3}   // Déclaration directe

// Ajouter un élément
new_manual_slice := append(manual_slice, 13)

// Afficher longueur et capacité
fmt.Printf("Len: %d, Cap: %d, Slice: %#v\n", len(new_manual_slice), cap(new_manual_slice), new_manual_slice)
```

💡 **Remarque :**

- **`len(slice)`** → Nombre d'éléments
- **`cap(slice)`** → Taille max avant réallocation
- **Les slices sont passés par référence (modifications appliquées sur l'original).**

---

## **4. Maps (`map`)**

Une **map** est une table de correspondance `clé -> valeur` (équivalent à un `HashMap` en Rust).

```go
// Déclaration et assignation
m := map[string]int{
	"route": 66,
	"age":   32,
}

// Lire une valeur (renvoie 0 si clé inexistante)
j := m["root"] // j == 0

// Vérifier si une clé existe
i, ok := m["route"]
fmt.Printf("i: %d, clé trouvée: %v\n", i, ok)

// Supprimer une clé
delete(m, "route")

// Itération
for key, value := range m {
	fmt.Println("Key:", key, "Value:", value)
}
```

💡 **Remarque :**

- Si une clé n'existe pas, **Go renvoie la valeur par défaut (`0` pour `int`, `""` pour `string`)**
- **`delete(m, key)`** supprime une clé
- On peut vérifier si une clé existe avec **`val, ok := map[key]`**

---

## **5. Structures (`struct`)**

Une **struct** est une définition d'objet regroupant plusieurs valeurs.

```go
type Vertex struct {
	X int
	Y int
}

// Méthode attachée à `Vertex`
func (v *Vertex) position() {
	fmt.Printf("X: %d, Y: %d\n", v.X, v.Y)
}

func main() {
	v := Vertex{1, 2} // Instanciation
	v.X = 4           // Modification d'un champ

	p := &v    // Pointeur vers la struct
	p.X = 1e9  // Modification via pointeur

	fmt.Println(v) // {1000000000 2}
	v.position()   // Appel d'une méthode
}
```

💡 **Remarque :**

- **Une struct peut avoir des méthodes associées.**
- **L'accès aux champs peut se faire via un pointeur `*struct` sans déréférencement explicite (`p.X` fonctionne directement).**

---

## **6. Résumé**

| Type     | Description                                |
| -------- | ------------------------------------------ |
| `array`  | Taille fixe, copie par valeur              |
| `slice`  | Taille dynamique, référence sur un tableau |
| `map`    | Table de hachage (`clé -> valeur`)         |
| `struct` | Objet personnalisé avec des champs         |

Go privilégie **la simplicité et la performance**, notamment avec **les slices et les maps** qui sont les plus couramment utilisés. 🚀
