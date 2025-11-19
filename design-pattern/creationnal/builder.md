# Builder Pattern — Game Example

## 🎮 Concept

- Construct **complex objects** step by step
- Different builders can produce **different variations** of the same type
- Keeps construction logic **separate from representation**

**Use when:**

- Objects have **lots of optional parameters**
- You want **readable, maintainable code** instead of huge constructors
- You want **different “flavors”** of the same object

---

## 🏗️ Product — RPG Character

```ts
class Character {
  name: string = "";
  type: string = "";
  hp: number = 100;
  mana: number = 50;
  weapon: string = "None";

  info() {
    return `${this.name} the ${this.type} | HP: ${this.hp} | Mana: ${this.mana} | Weapon: ${this.weapon}`;
  }
}
```

---

## 🛠️ Builder Interface

```ts
interface CharacterBuilder {
  setName(name: string): this;
  setType(type: string): this;
  setHp(hp: number): this;
  setMana(mana: number): this;
  setWeapon(weapon: string): this;
  build(): Character;
}
```

---

## ⚔️ Concrete Builder

```ts
class RPGCharacterBuilder implements CharacterBuilder {
  private character = new Character();

  setName(name: string) {
    this.character.name = name;
    return this;
  }
  setType(type: string) {
    this.character.type = type;
    return this;
  }
  setHp(hp: number) {
    this.character.hp = hp;
    return this;
  }
  setMana(mana: number) {
    this.character.mana = mana;
    return this;
  }
  setWeapon(weapon: string) {
    this.character.weapon = weapon;
    return this;
  }

  build() {
    return this.character;
  }
}
```

---

## 🎯 Usage

```ts
const builder = new RPGCharacterBuilder();
const mage = builder
  .setName("Zyra")
  .setType("Mage")
  .setHp(70)
  .setMana(150)
  .setWeapon("Staff of Fire")
  .build();

console.log(mage.info());
// Zyra the Mage | HP: 70 | Mana: 150 | Weapon: Staff of Fire
```

---

## 🧠 Notes

- Builder **fluent interface** → chainable methods
- You can create **different types** of the same product (Mage, Warrior, Rogue)
- Keeps constructors **clean and readable**
- Often used with **Director** class for pre-defined builds (optional)

---

## 🔑 TL;DR

- **Purpose:** construct complex objects step by step
- **Product:** the final object (Character)
- **Builder:** encapsulates construction logic
- **Fluent API:** chain setters → more readable
- **Game usage:** RPG characters, levels, weapons, skill trees

---

If you want, I can make a **combined cheat-sheet with Factory + Singleton + Builder** in markdown for games. That way you have all three main creational patterns ready.

Do you want me to do that?
