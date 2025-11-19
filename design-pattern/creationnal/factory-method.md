# Factory Method — Game Example (Cheat-Sheet)

## 🎮 Concept

You want to spawn enemies without writing `if (mage) … else if (warrior) …`.

**Factory Method = let a class decide which subclass to create.**

- All enemies share the same interface
- Base class can hold common logic
- Subclasses override only what’s different
- Factory builds the right enemy at runtime

---

## 🧩 Product Interface (what every enemy must do)

```ts
interface Enemy {
  attack(): number;
}
```

---

## 🏗️ Abstract Base Enemy (shared logic)

```ts
abstract class BaseEnemy implements Enemy {
  attack() {
    const dmg = this.baseDamage() + this.specialBonus();
    return dmg;
  }

  protected baseDamage() {
    return 10; // shared base damage
  }

  protected abstract specialBonus(): number; // subclasses override
}
```

---

## ⚔️ Concrete Enemies

```ts
class FireMage extends BaseEnemy {
  protected specialBonus() {
    return 15;
  } // fire burst
}

class Berserker extends BaseEnemy {
  protected specialBonus() {
    return 25;
  } // rage bonus
}
```

---

## 🏭 Factory — decides which enemy is created

```ts
abstract class EnemyFactory {
  abstract createEnemy(): Enemy;
}

class FireMageFactory extends EnemyFactory {
  createEnemy() {
    return new FireMage();
  }
}

class BerserkerFactory extends EnemyFactory {
  createEnemy() {
    return new Berserker();
  }
}
```

---

## 🎯 Usage

```ts
function spawnEnemy(factory: EnemyFactory) {
  return factory.createEnemy();
}

const enemy1 = spawnEnemy(new FireMageFactory());
const enemy2 = spawnEnemy(new BerserkerFactory());

enemy1.attack();
enemy2.attack();
```

---

## 🧠 Why use this pattern?

- Avoid huge `if/else` creation logic
- Easy to add new enemy types
- Clean polymorphism → game code doesn’t care which enemy it gets
- Shared logic lives in the abstract base class
- Subclasses override only the unique part
- Factories decide _create what_ → game stays flexible

---

## 🔑 TL;DR (the simplest recap)

- **Interface** = what every enemy can do
- **Abstract Base Class** = common behavior
- **Concrete Classes** = specific behavior
- **Factory** = decides which concrete class to create
- **Polymorphism** = game uses all enemies the same way

---

If you want, I can also generate a printable PDF version later.
