-- LUA TYPES
type(42) --`number`
type("Tutoriel Lua") -- `string`
type(false) -- `boolean`

-- IF A VAR IS NOT DECLARE IT'S NIL
type(var) -- `nil`

local addition = 1 + 2

-- PRINT
io.write(addition, "\n")
print(addition)

-- STRING

local str = "Walther" .. "White" --Concat
print(str)

local str2 = "Les " .. 3 .. " mousquetaires" -- INFERE 3 TO STRING

print(str2)
print(string.format("Hello %s", str2)) --%s flag substitute var
print(string.format("not true = %s", tostring(not true))) --cast and print boolean into string

--Multi line string
local str_multiline = [[
  Hello
  World
]]

--Str method
io.write("length:", string.len(str_multiline))
io.write(#str_multiline) --Same

-- NUMBER

Prix = 42.99
Remise = 0.15 -- 15 % de remise

-- Decrement must be explicit "-= does not work"
Prix = Prix - (Prix * Remise)

-- EXTERNAL SCOPE
do
	local x = 1
	do -- INTERNAL SCOPE
		local y = 2
		Z = x + y
	end
	print(x) --`1`
	print(y) -- `nil` because `y` does not exist in external scope
	print(Z) -- `3`
end
-- `Z` is global and live outside the external scope
Z = Z + 4
print(Z) -- `7`

-- CONTROL FLOW

-- "~=" for not equals and not "!="

local limit = 42
local nombre = 43
if nombre < limit then
	print("En-dessous de la limite.")
elseif nombre == limit then
	print("Précisément à la limite…")
else
	print("Au-dessus de la limite !")
end

-- LOOP
limit = 10
nombre = 1
while nombre <= limit do
	print("nombre:", nombre)
	nombre = nombre + 1
end

-- Even if number is > to limit loop executed one time !
nombre = 11
repeat
	print("nombre:", nombre)
	nombre = nombre + 1
until nombre > limit

-- Does'nt need to declare variable in for loop
local debut = 1
local fin = 10
for i = debut, fin do
	print("Nombre actuel :", nombre) -- `1,2,3,4,5,6,7,8,9,10`
end

-- Step fixed to do operation when the step is reached
local etape = 2
for y = debut, fin, etape do
	print("Nombre actuel :", nombre) -- `1,3,5,7,9`
end

-- Negative step
etape = -2

-- Reversed loop
for x = fin, debut, etape do
	print("Nombre actuel :", nombre) -- `10,8,6,4,2`
end

-- ARRAY

local decades = { 1910, 1920, 1930, 1940, 1950, 1960, 1970, 1980, 1990 }

print(decades[1]) -- BEGIN AT INDEX 1 !!!!
print(decades[#decades]) -- LAST INDEX

-- Use ipairs as an iterator
for index, year in ipairs(decades) do
	print(index, year)
end

-- FUNCTIONS

-- Define func
function Bonjour(nom)
	print("Bonjour", nom)
end

-- Calling func
Bonjour("très cher monsieur")
Bonjour("très cher monsieur") -- WORK on literral
-- nom = "Walther"
-- Bonjour nom -- ERROR on variale need parentheses

-- Print all the function arguments
function Var_args(...)
	for index, arg in ipairs({ ... }) do
		print(index, arg)
	end
end
Var_args("Peter", 42, true)

-- Multiple return
local function premier_et_dernier(liste)
	return liste[1], liste[#liste]
end

local personnes = { "Jim", "Jack", "John" }

-- Destructuring a function with multiple return
local premier, dernier = premier_et_dernier(personnes)
print("Le premier est", premier)
print("Le dernier est", dernier)

local function min_moyenne_max(...)
	-- définir les valeurs de début pour `min` et `max` sur le premier argument
	local min = select(1, ...)
	local max = select(1, ...)
	-- Définir la valeur médiane sur zéro au début
	local moyenne = 0
	-- itérer sur les chiffres
	-- nous n’avons pas besoin de la variable index
	-- nous utilisons `_` en tant que balise
	for _, nombre in ipairs({ ... }) do
		-- définir un nouveau minimum le cas échéant
		if min > nombre then
			min = nombre
		end
		-- définir un nouveau maximum le cas échéant
		if max < nombre then
			max = nombre
		end
		-- additionner des chiffres pour la moyenne
		moyenne = moyenne + nombre
	end
	-- diviser la somme des nombres par leur nombre
	moyenne = moyenne / #{ ... }
	return min, moyenne, max
end
-- ici, nous n’avons pas besoin de la valeur `moyenne`
-- nous utilisons `_` en tant que balise
min, _, max = min_moyenne_max(78, 34, 91, 7, 28)
print("Le minimum et le maximum des nombres sont", min, max)

-- MAP (take a function as first argument and an array in second arg)
local function map(f, liste)
	-- New list for returned value
	local _liste = {
	-- itérer sur les éléments de la liste avec index
	for index, valeur in ipairs(liste) do
		-- utiliser la fonction `f()` sur la valeur actuelle de la liste
		-- et enregistrer la valeur de retour dans la nouvelle liste sur le même index
		_liste[index] = f(valeur)
	end
	-- retourner une nouvelle liste
	return _liste
end
-- Liste de chiffres
local nombres = { 3, 4, 5 }
-- Fonction appliquée à tous les éléments de la liste
local function carre(nombre)
	return nombre * nombre
end
-- génération des carrees via la fonction `map()`
local carrees = map(carre, nombres) -- `{9, 16, 25}`
-- obtenir les carrees
for _, nombre in ipairs(carrees) do
	print(nombre)
end
