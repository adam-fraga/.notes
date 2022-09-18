//Les interfaces peuvent être utilisé comme les types vu précédement
//elle permettent de définir des types et sont moins flexible que le mot
//clés "type" mais plus ouverte.

//POSSIBLE
interface Point {
  x: number
}

interface Point {
  y: number
}

//IMPOSSIBLE de redéfinir une interface (moins ouverte que type keyword)
type Point2 = { x: number }

type Point2 = { y: number }

//Implémenter une interface
class TwoDimensionalPoint implements Point {
  x = 12
  y = 13
}
