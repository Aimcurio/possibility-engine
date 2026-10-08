# THE POSSIBILITY ENGINE

> **"Intelligence determines what to attempt. The simulation determines what actually happens. The player determines who controls the avatar."**

The Possibility Engine is a high-integrity, modular 3D interactive sandbox built in Rust with Bevy ECS, featuring physics-authoritative simulation, deterministic offline task planning, multi-avatar control arbitration, and live scenario execution.

---

## Architecture Overview

```
                                      +------------------------------------+
                                      |          Player / User             |
                                      +-----------------+------------------+
                                                        |
                                            Input & UI Interactivity
                                                        |
                                                        v
+------------------------+             +----------------+------------------+
| Deterministic Planner  | <---------> |        Control Arbitrator        |
| (Perception & Goals)   |             | (Human, AI, Copilot, Scripted)   |
+-----------+------------+             +----------------+------------------+
            |                                           |
    Action Requests                              Sim Commands
            |                                           |
            v                                           v
+--------------------------------------------------------------------------+
|                  Authoritative World Simulation Core                     |
|                                                                          |
|  * Fixed Timestep Kinematics      * Containment & Contents Model         |
|  * Radial Artificial Gravity      * Socket Grasping & Assembly           |
|  * Simulation Invariants          * Deterministic State Transitions      |
+--------------------------------------------------------------------------+
                                    |
                             World Transforms
                                    |
                                    v
+--------------------------------------------------------------------------+
|                     Presentation & 3D Visual Viewport                    |
|                                                                          |
|  * 3D Laboratory Environment      * 3 Selectable Avatars (Alpha/Beta/Gamma)|
|  * Live HUD Diagnostics           * Emissive Field & Particle Gizmos     |
+--------------------------------------------------------------------------+
```

### Decoupled Crates:
1. **`possibility_core`**: Shared mathematical types (`Vec3`, `Aabb`), strong IDs (`AvatarId`, `ObjectId`), simulation invariants, commands, and events. Zero graphics dependencies.
2. **`possibility_sim`**: Authoritative world physics, fixed-timestep kinematic simulation, container/contents model (cereal and liquid milk), radial artificial gravity field generator, and mass/volume conservation enforcement.
3. **`possibility_cognition`**: Perception API, goal decomposition, deterministic offline task planner (Scenario A cereal prep, Scenario B gravity activation), and bounded natural language instruction interpreter.
4. **`possibility_control`**: Multi-avatar control arbitrator supporting `Human`, `AutonomousAI`, `DirectedAI`, `Copilot`, and `Scripted` modes with safe interruption and state preservation.
5. **`possibility_app`**: Navigable 3D Bevy game application, laboratory scene, UI HUD, camera controller, and headless verification runner (`--headless`, `--verify`).

---

## Mandatory Playable Scenarios

### Scenario A — Bowl of Cereal
- **Objective:** Direct an avatar to prepare a bowl of cereal using in-world physical objects.
- **Observable Execution:**
  1. Avatar identifies bowl, cereal box, and milk carton.
  2. Avatar navigates to counter, picks up cereal box, and dispenses 45g of cereal grains into the bowl.
  3. Avatar places cereal box, picks up milk carton, and pours 120ml of milk into the bowl.
  4. Authoritative verification checks `bowl.cereal_grams >= 25.0` AND `bowl.milk_ml >= 50.0`.
  5. Mass conservation and non-negative volume invariants are verified.

### Scenario B — Artificial Gravity Experiment
- **Objective:** Instruct an avatar to activate a fictional artificial-gravity field generator.
- **Observable Execution:**
  1. Avatar navigates to the gravity device terminal at `(4.5, 0.0, 1.5)`.
  2. Device enters validated active state; core pulses with cyan energy.
  3. Nearby test objects (Alloy Cubes Alpha & Beta, Test Sphere) accelerate toward the field center via bounded radial force `F = I / (1 + r^2)`.
  4. Deactivating the device immediately terminates the field force.
  5. Reset restores all objects to their initial laboratory coordinates.

### Scenario C — Multi-Avatar Control Handoff
- **Objective:** Switch an avatar between Autonomous AI, Human direct control, and Directed AI without state corruption.
- **Observable Execution:**
  1. Avatar 0 (Alpha) begins executing cereal preparation autonomously and grasps the cereal box.
  2. Player interrupts by switching Avatar 0 to `Human` mode.
  3. Player steers Avatar 0 manually; the cereal box remains securely held in the hand socket.
  4. Player returns control to `DirectedAI`.
  5. The planner inspects the current world state and seamlessly resumes remaining steps to completion.

---

## Building and Verification

### Prerequisites
- Rust 1.80+ (tested and verified on Rust 1.97.1)
- Cargo

### Run All Unit & Adversarial Tests
```bash
cargo test --workspace --all-features
```

### Run Code Formatting & Strict Lint Checks
```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

### Run Headless Verification Scenarios
```bash
cargo run --bin possibility_app -- --headless --verify all
```

### Launch Interactive 3D Application
```bash
cargo run --bin possibility_app
```

---

## In-Game Controls

| Key | Action |
|---|---|
| `[1]`, `[2]`, `[3]` | Select Active Avatar (Alpha: Scientist, Beta: Engineer, Gamma: Explorer) |
| `[H]` | Switch Selected Avatar to **Human** (Direct Control) |
| `[A]` | Switch Selected Avatar to **Autonomous AI** |
| `[D]` | Switch Selected Avatar to **Directed AI** |
| `[C]` | Switch Selected Avatar to **Copilot** |
| `[W, A, S, D]` / Arrows | Move Active Human Avatar |
| `[Space]` | Pick Up Nearest Object / Place Held Object |
| `[F1]` | Trigger **Scenario A** (Prepare Bowl of Cereal) |
| `[F2]` | Trigger **Scenario B** (Activate Gravity Field) |
| `[F3]` | Trigger **Scenario C** (Control Handoff Demo) |
| `[R]` | Reset Simulation & Laboratory State |
