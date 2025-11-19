# Singleton Pattern — Game Example

## 🎮 Concept

Only **one instance** of a class exists. Everyone shares it.

**Use when:**

- Only one source of truth is needed
- Global access is convenient
- Avoid conflicting instances (GameManager, Scoreboard, AudioManager)

---

## 🏗️ Singleton Class (TypeScript)

```ts
class GameManager {
  // 🔹 Static attribute stores the single instance
  private static instance: GameManager | null = null;

  public score: number = 0;

  // Private constructor prevents direct instantiation
  private constructor() {}

  // 🔹 Static method to access the single instance
  public static getInstance(): GameManager {
    if (!GameManager.instance) {
      GameManager.instance = new GameManager();
    }
    return GameManager.instance;
  }

  public increaseScore(points: number) {
    this.score += points;
  }
}
```

---

## 🎯 Usage

```ts
const manager1 = GameManager.getInstance();
manager1.increaseScore(10);

const manager2 = GameManager.getInstance();
console.log(manager2.score); // 10 → same instance as manager1

console.log(manager1 === manager2); // true
```

---

## 🧠 Notes

- **Static attribute (`instance`)** stores the single object for global access
- Private constructor → prevents `new GameManager()`
- Static `getInstance()` → provides controlled, global access
- Lazy initialization → instance created on first call
- Thread-safety: TypeScript is single-threaded; in multi-threaded languages, use locks

---

## 🔑 TL;DR

- **Purpose:** enforce single source of truth
- **Static attribute** is the core of Singleton
- **Usage in games:** GameManager, Scoreboard, AudioManager, Config
- **Behavior:** every reference points to the same object → avoids multiple conflicting instances

---

If you want, I can also make a **super-short combined Factory + Singleton markdown cheat-sheet** showing the GameManager spawning enemies via factories — perfect to have both patterns at a glance.
