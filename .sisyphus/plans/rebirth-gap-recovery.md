# GoldMiner Rebirth Parity Recovery Plan

## TL;DR

> **Quick Summary**: Restore the current Bevy game to near-original `GoldMiner-Rebirth` parity by porting missing visible gameplay, shop, FX, animation, and transition behaviors into the existing plugin/state architecture.
>
> **Deliverables**:
> - Missing asset exposure and FX plumbing for original spritesheets
> - Gameplay parity for grabbed-object rotation, reward flow, money display, and strength presentation
> - Shop parity for shopkeeper presentation and end-state behavior
> - Audio/UI/timing parity for original transition details
>
> **Estimated Effort**: Large
> **Parallel Execution**: YES - 2 major waves + final verification
> **Critical Path**: 1 -> 2 -> 3/4 -> 5/7 -> 10

---

## Context

### Original Request
参考 `~/workspace/GoldMiner-Rebirth` 的原版代码，在当前项目里补上缺失的功能。

### Interview Summary
**Key Discussions**:
- Scope choice: 尽量对齐原版，把可见玩法、UI、演出差异一起纳入同一个计划
- Verification choice: 不优先补测试基础设施，采用构建/运行验证 + agent-executed 游戏流程 QA
- Technical direction: 保留当前 Rust + Bevy 0.18 插件化结构，只迁移原版行为与表现，不照搬 Lua 架构

**Research Findings**:
- Current repo has parity gaps around asset exposure, FX playback, grabbed-entity presentation, money counting, strength presentation, shopkeeper UI, and transition-audio behavior
- Existing assets already include `assets/images/gold_big_fx_sheet.png`, `assets/images/explosive_fx_sheet.png`, and `assets/images/shopkeeper_sheet.png`, but current `src/config.rs` does not expose all of them
- Audio infrastructure in `src/audio.rs` only loads handles and playback bundles; it has no fade-capable transition system

### Metis Review
**Identified Gaps** (addressed in this plan):
- Asset availability ambiguity resolved by direct asset directory audit
- Architecture ambiguity resolved by explicitly preserving current Bevy plugin/state/module boundaries
- Scope creep risk locked down by forbidding non-parity feature additions and opportunistic refactors
- Acceptance gaps addressed by concrete agent-runnable QA scenarios per task, including happy-path and failure-path verification

---

## Work Objectives

### Core Objective
Bring the current Bevy implementation materially in line with the original `GoldMiner-Rebirth` presentation and gameplay feedback loop, especially where the current port is visibly incomplete or behaviorally inconsistent.

### Concrete Deliverables
- `src/config.rs` exposes all currently-missing original-parity image assets needed by gameplay and shop screens
- Gameplay systems in `src/demo/` support original-like sparkle FX, explosive FX, grabbed-object rotation/offset behavior, money display animation, and strength presentation timing
- Shop screen in `src/screens/shop.rs` renders and updates a shopkeeper character with parity-appropriate state changes
- Transition/audio/UI polish matches original timing and messaging closely enough to remove obvious parity gaps

### Definition of Done
- [ ] `cargo fmt --all -- --check` passes
- [ ] `cargo clippy --locked --workspace --all-targets --all-features` passes
- [ ] `cargo test --locked --workspace --all-targets` passes
- [ ] `cargo build --release` passes
- [ ] `cargo run` allows agent-executed verification of gameplay, shop, and transition parity scenarios without obvious missing original behaviors listed in this plan

### Must Have
- Restore all identified high- and medium-priority visible parity gaps from the original reference
- Reuse current assets already present in `assets/images/`
- Keep all gameplay/shop logic under existing `Screen` state guards and module boundaries

### Must NOT Have (Guardrails)
- No plugin/state architecture rewrite
- No literal Lua-to-Rust structural port
- No new non-original features, bonus polish systems, or expanded save/progression work
- No test-infrastructure initiative beyond the existing repo verification commands
- No asset replacements when original-parity assets already exist locally

---

## Verification Strategy

> **ZERO HUMAN INTERVENTION** — all verification is agent-executed.

### Test Decision
- **Infrastructure exists**: NO practical gameplay test suite exists
- **Automated tests**: None added by default
- **Framework**: existing `cargo test` only
- **Primary verification**: `fmt` + `clippy` + `test` + `build` + interactive gameplay QA via running the game

### QA Policy
Every task below includes agent-executed QA scenarios with concrete commands, interactions, and evidence targets.

- **Frontend/UI-like gameplay validation**: launch `cargo run`, interact via keyboard/gamepad mapping, capture screenshots or terminal logs into `.sisyphus/evidence/`
- **Static verification**: use cargo checks and file/behavior inspection to verify assets and transitions are wired
- **Edge cases**: verify both successful visible behavior and non-trigger / cleanup behavior

---

## Execution Strategy

### Parallel Execution Waves

Wave 1 (Start Immediately - foundations and independent parity scaffolding):
- Task 1: Asset exposure and atlas constants [quick]
- Task 2: Shared FX animation plumbing [unspecified-high]
- Task 6: Money display animation infrastructure [quick]
- Task 8: Shopkeeper entity and screen presentation foundation [visual-engineering]
- Task 9: Transition-audio parity infrastructure [unspecified-high]

Wave 2 (After Wave 1 - feature parity wiring and cleanup):
- Task 3: BigGold sparkle parity [quick] (depends: 1, 2)
- Task 4: Standard explosive FX parity [unspecified-high] (depends: 1, 2)
- Task 5: Grabbed-entity rotation and carry parity [unspecified-high] (depends: 1)
- Task 7: Strength reward flow and player animation parity [deep] (depends: 5, 6)
- Task 10: Timing, skip-tip, reset, and shop end-state polish [quick] (depends: 5, 7, 8, 9)

Wave FINAL (After all implementation tasks - parallel review):
- F1: Plan compliance audit [oracle]
- F2: Code quality review [unspecified-high]
- F3: Real gameplay QA replay [unspecified-high]
- F4: Scope fidelity check [deep]

Critical Path: 1 -> 2 -> 3 -> 5 -> 7 -> 10 -> F1-F4
Parallel Speedup: ~55% vs sequential execution
Max Concurrent: 5

### Dependency Matrix

- **1**: blocked by none -> blocks 3, 4, 5
- **2**: blocked by none -> blocks 3, 4
- **3**: blocked by 1, 2 -> blocks 10
- **4**: blocked by 1, 2 -> blocks 10
- **5**: blocked by 1 -> blocks 7, 10
- **6**: blocked by none -> blocks 7, 10
- **7**: blocked by 5, 6 -> blocks 10
- **8**: blocked by none -> blocks 10
- **9**: blocked by none -> blocks 10
- **10**: blocked by 5, 7, 8, 9 -> blocks F1, F3, F4
- **F1**: blocked by 1-10 -> final approval set
- **F2**: blocked by 1-10 -> final approval set
- **F3**: blocked by 1-10 -> final approval set
- **F4**: blocked by 1-10 -> final approval set

### Agent Dispatch Summary

- **Wave 1**: 5 agents — T1 `quick`, T2 `unspecified-high`, T6 `quick`, T8 `visual-engineering`, T9 `unspecified-high`
- **Wave 2**: 5 agents — T3 `quick`, T4 `unspecified-high`, T5 `unspecified-high`, T7 `deep`, T10 `quick`
- **Final**: 4 agents — F1 `oracle`, F2 `unspecified-high`, F3 `unspecified-high`, F4 `deep`

---

## TODOs

- [x] 1. Expose missing original-parity assets in config

  **What to do**:
  - Add `gold_big_fx_sheet.png`, `explosive_fx_sheet.png`, and `shopkeeper_sheet.png` handles to `ImageAssets`
  - Expose them through `ImageAssets::get_image()` with parity-friendly IDs used by gameplay/shop modules
  - Define any shared atlas dimensions/constants needed by later tasks in the most natural current module

  **Must NOT do**:
  - Do not rename existing asset files
  - Do not introduce new asset packs or remote assets

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: localized resource wiring with low algorithmic complexity
  - **Skills**: `[]`
    - No special skill required; existing repo patterns are sufficient
  - **Skills Evaluated but Omitted**:
    - `ui-ux-pro-max`: task is asset exposure, not UI redesign

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with 2, 6, 8, 9)
  - **Blocks**: 3, 4, 5
  - **Blocked By**: None

  **References**:
  - `src/config.rs:140` - current image asset registry shape and handle-loading pattern
  - `src/config.rs:272` - current `get_image()` ID mapping to extend without breaking call sites
  - `assets/images/gold_big_fx_sheet.png` - original-parity sparkle sheet already present locally
  - `assets/images/explosive_fx_sheet.png` - original-parity standard explosion sheet already present locally
  - `assets/images/shopkeeper_sheet.png` - original-parity shopkeeper spritesheet already present locally
  - `rebirth/main.lua:135` - original shopkeeper sheet loading and animation setup
  - `rebirth/main.lua:194` - original FX sheet loading and frame-count source of truth

  **Acceptance Criteria**:
  - [ ] New image IDs resolve successfully from `ImageAssets::get_image()` for BigGold FX, standard explosive FX, and shopkeeper sheet
  - [ ] Atlas/frame constants needed later are declared in code, not duplicated ad hoc across modules

  **QA Scenarios**:
  ```text
  Scenario: Asset handles can be resolved by downstream systems
    Tool: Bash
    Preconditions: task implementation complete
    Steps:
      1. Run `cargo clippy --locked --workspace --all-targets --all-features`
      2. Confirm no compile errors mention missing image IDs or missing asset fields
    Expected Result: build graph resolves all new asset references cleanly
    Failure Indicators: unknown match arm IDs, missing struct fields, unresolved image handle access
    Evidence: .sisyphus/evidence/task-1-asset-exposure.txt

  Scenario: No accidental asset path regressions
    Tool: Bash
    Preconditions: same as above
    Steps:
      1. Run `cargo build --release`
      2. Search build output for asset-related failures
    Expected Result: release build succeeds without path/load compilation regressions
    Evidence: .sisyphus/evidence/task-1-release-build.txt
  ```

  **Commit**: YES
  - Message: `feat(config): 补齐原版缺失资源映射`
  - Files: `src/config.rs`
  - Pre-commit: `cargo clippy --locked --workspace --all-targets --all-features`

- [x] 2. Add shared FX animation plumbing for atlas-driven parity effects

  **What to do**:
  - Introduce or extend reusable gameplay FX support so both sparkle FX and standard explosive FX can animate and self-clean reliably
  - Keep the implementation aligned with current `src/demo/explosive.rs` / `hook.rs` patterns instead of creating a parallel subsystem
  - Ensure FX can optionally follow an entity or spawn at a fixed world position

  **Must NOT do**:
  - Do not introduce a full particle engine
  - Do not scatter one-off timers in unrelated modules

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: cross-cutting gameplay support with lifecycle and animation concerns
  - **Skills**: `[]`
    - Repo-local ECS patterns are the key guidance
  - **Skills Evaluated but Omitted**:
    - `ui-ux-pro-max`: effect plumbing is engine logic, not visual redesign

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with 1, 6, 8, 9)
  - **Blocks**: 3, 4
  - **Blocked By**: None

  **References**:
  - `src/demo/hook.rs:191` - current bigger explosive FX spawn pattern to reuse or generalize
  - `src/demo/hook.rs:431` - reward/cleanup timing where attached FX may need coordination
  - `rebirth/Entities.lua:373` - original generic FX object lifecycle and follow-entity behavior
  - `rebirth/Entities.lua:413` - original FX render rotation behavior bound to hook angle
  - `rebirth/main.lua:197` - sparkle animation frame cadence
  - `rebirth/main.lua:204` - standard explosive FX frame cadence

  **Acceptance Criteria**:
  - [ ] Gameplay code has one reusable way to animate these atlas-based parity FX
  - [ ] FX cleanup occurs automatically after playback or state exit

  **QA Scenarios**:
  ```text
  Scenario: Shared FX plumbing compiles and integrates with existing explosion path
    Tool: Bash
    Preconditions: task implementation complete
    Steps:
      1. Run `cargo test --locked --workspace --all-targets`
      2. Run `cargo clippy --locked --workspace --all-targets --all-features`
    Expected Result: existing explosion code and new shared FX code compile together without warnings-as-errors failures
    Failure Indicators: ECS bundle/type conflicts, lifetime cleanup issues, atlas index errors
    Evidence: .sisyphus/evidence/task-2-fx-plumbing.txt

  Scenario: FX system remains safe when no consumer triggers it
    Tool: Bash
    Preconditions: buildable project
    Steps:
      1. Run `cargo build --release`
      2. Confirm there is no compile-time requirement for immediate trigger callers beyond implemented sites
    Expected Result: optional FX plumbing does not force invalid initialization states
    Evidence: .sisyphus/evidence/task-2-build.txt
  ```

  **Commit**: YES
  - Message: `feat(demo): 增加通用特效动画支撑`
  - Files: `src/demo/hook.rs`, `src/demo/explosive.rs`, related module files
  - Pre-commit: `cargo test --locked --workspace --all-targets`

- [x] 3. Restore BigGold sparkle effect behavior

  **What to do**:
  - Trigger the sparkle effect specifically for `BigGold` in the same gameplay moments as the original
  - Keep it visually attached to the gold and cleaned up on despawn/collection
  - Match original frame order and approximate duration from the reference implementation

  **Must NOT do**:
  - Do not trigger sparkle for non-`BigGold` entities
  - Do not leave orphaned sparkle entities after grab resolution

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: focused behavior restoration on top of existing asset/FX plumbing
  - **Skills**: `[]`
    - Existing references are sufficient
  - **Skills Evaluated but Omitted**:
    - `ui-ux-pro-max`: this is fidelity restoration, not redesign

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with 4, 5, 7, 10)
  - **Blocks**: 10
  - **Blocked By**: 1, 2

  **References**:
  - `src/demo/hook.rs:452` - current grabbed-entity synchronization point where attached effect state may matter
  - `src/config.rs:228` - existing gold asset naming pattern to mirror for BigGold FX IDs
  - `rebirth/Entities.lua:422` - original `BigGold` FX creation point
  - `rebirth/Entities.lua:441` - original activation timing when effect is taken/applied
  - `rebirth/Entities.lua:472` - original FX update trigger during entity update loop

  **Acceptance Criteria**:
  - [ ] Grabbing or activating `BigGold` causes the sparkle animation to play with no effect on other entity types
  - [ ] Sparkle FX follows/despawns with the gold cleanly

  **QA Scenarios**:
  ```text
  Scenario: BigGold displays sparkle during gameplay interaction
    Tool: interactive_bash
    Preconditions: run `cargo run`, reach a level containing `BigGold`
    Steps:
      1. Start the game in tmux and enter gameplay
      2. Aim the hook at a `BigGold` entity and grab it
      3. Observe/capture the sparkle frames appearing near the gold before cleanup
    Expected Result: visible sparkle animation appears only for `BigGold`
    Failure Indicators: no sparkle, sparkle on wrong entities, leftover sparkle after entity removal
    Evidence: .sisyphus/evidence/task-3-biggold-sparkle.txt

  Scenario: Non-BigGold entities do not inherit sparkle behavior
    Tool: interactive_bash
    Preconditions: same session, level with at least one non-BigGold target
    Steps:
      1. Grab a normal gold or rock entity
      2. Observe whether sparkle is absent
    Expected Result: no BigGold sparkle animation triggers on non-BigGold pickups
    Evidence: .sisyphus/evidence/task-3-non-biggold.txt
  ```

  **Commit**: YES
  - Message: `feat(gameplay): 恢复大金块闪光特效`
  - Files: `src/demo/hook.rs`, related FX module files
  - Pre-commit: `cargo run`

- [ ] 4. Restore standard explosive FX parity for grabbed entities

  **What to do**:
  - Use the original standard explosion sheet where the original game shows a smaller explosion effect, distinct from the existing bigger dynamite/TNT effect
  - Wire this to the appropriate grabbed-entity destruction path and keep the current bigger effect for large explosive cases if still required
  - Ensure cleanup and z-order feel consistent with the gameplay scene

  **Must NOT do**:
  - Do not replace all explosion effects with the same sheet
  - Do not break TNT-specific behavior already present

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: it touches destruction semantics and effect differentiation
  - **Skills**: `[]`
    - Existing gameplay references are enough
  - **Skills Evaluated but Omitted**:
    - `ui-ux-pro-max`: not a UI layout problem

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with 3, 5, 7, 10)
  - **Blocks**: 10
  - **Blocked By**: 1, 2

  **References**:
  - `src/demo/hook.rs:171` - current dynamite-use path and bigger explosion spawn behavior
  - `src/demo/hook.rs:213` - current grabbed-entity destroy branch that needs parity-specific FX selection
  - `rebirth/main.lua:202` - original standard explosive sheet and frame count
  - `rebirth/Entities.lua:373` - original FX helper behavior used by explosive visuals

  **Acceptance Criteria**:
  - [ ] Standard explosive FX is available and used in the correct non-bigger explosion path
  - [ ] Existing bigger explosion usage still works where intended

  **QA Scenarios**:
  ```text
  Scenario: Standard destruction path uses small explosive FX
    Tool: interactive_bash
    Preconditions: run `cargo run`, enter gameplay, have a grabbed entity and dynamite available if required by current path
    Steps:
      1. Trigger the grabbed-entity destruction path that should use the standard effect
      2. Capture the on-screen explosion size and animation
    Expected Result: the effect is visually distinct from the larger explosive sheet path
    Failure Indicators: only bigger FX appears everywhere, no FX appears, wrong cleanup
    Evidence: .sisyphus/evidence/task-4-standard-explosion.txt

  Scenario: Existing larger explosive path is not regressed
    Tool: interactive_bash
    Preconditions: same session
    Steps:
      1. Trigger the path that previously used `BiggerExplosiveFX`
      2. Verify the large explosion still renders and resolves
    Expected Result: both explosion classes coexist correctly
    Evidence: .sisyphus/evidence/task-4-large-explosion.txt
  ```

  **Commit**: YES
  - Message: `feat(explosive): 区分原版大小爆炸特效`
  - Files: `src/demo/hook.rs`, `src/demo/explosive.rs`, `src/config.rs`
  - Pre-commit: `cargo run`

- [x] 5. Restore grabbed-entity carry rotation and offset parity

  **What to do**:
  - Ensure grabbed entities rotate with the hook angle and use an offset/anchor behavior that visually matches the original carry pose
  - Preserve current patrol entity handling while preventing carry rotation from conflicting with move logic
  - Review both fixed and moving entities so their carry presentation is consistent

  **Must NOT do**:
  - Do not break collision position logic
  - Do not allow carried entities to keep running patrol movement while attached

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: touches cross-module gameplay transforms and moving-entity behavior
  - **Skills**: `[]`
    - LSP/read references should be enough
  - **Skills Evaluated but Omitted**:
    - `ui-ux-pro-max`: gameplay transform parity, not style work

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with 3, 4, 7, 10)
  - **Blocks**: 7, 10
  - **Blocked By**: 1

  **References**:
  - `src/demo/hook.rs:451` - current carry synchronization already rotates the entity transform but needs parity validation and anchor treatment
  - `src/demo/entity.rs:143` - patrol system currently skips grabbed entities and is the place to protect move logic from conflicts
  - `rebirth/Entities.lua:477` - original carry position follows hook collision center
  - `rebirth/Entities.lua:514` - original render-time rotation and offset for grabbed entities
  - `rebirth/Entities.lua:580` - moving entity carry behavior in the original

  **Acceptance Criteria**:
  - [ ] Carried entities visibly rotate with the hook and look attached rather than sliding flat
  - [ ] Moving entities stop patrol presentation while attached and resume normal behavior after state resolution if still present

  **QA Scenarios**:
  ```text
  Scenario: Normal entity rotates while being reeled back
    Tool: interactive_bash
    Preconditions: run `cargo run`, enter gameplay, target a normal gold or rock
    Steps:
      1. Fire the hook at a carryable entity
      2. Watch the entity during reel-back while the hook angle changes
    Expected Result: the carried entity rotates with the hook angle instead of remaining flat/upright
    Failure Indicators: entity only translates, anchor looks incorrect, clipping is severe
    Evidence: .sisyphus/evidence/task-5-carry-rotation.txt

  Scenario: Moving entity does not keep patrol motion while grabbed
    Tool: interactive_bash
    Preconditions: same session, level with a mole or moving target
    Steps:
      1. Grab the moving entity while it is patrolling
      2. Observe whether patrol movement ceases during carry
    Expected Result: the entity follows hook carry only, without independent patrol drift
    Evidence: .sisyphus/evidence/task-5-moving-carry.txt
  ```

  **Commit**: YES
  - Message: `fix(gameplay): 对齐抓取物体旋转与挂载表现`
  - Files: `src/demo/hook.rs`, `src/demo/entity.rs`
  - Pre-commit: `cargo run`

- [x] 6. Add money-view animation infrastructure for HUD parity

  **What to do**:
  - Introduce a view-layer money counter distinct from immediate logical money so the HUD can count up smoothly like the original `money4View`
  - Keep gameplay economy authoritative in existing stats/player resources; only animate display state
  - Update gameplay HUD and shop money display integration so transitions remain coherent

  **Must NOT do**:
  - Do not change reward math
  - Do not make shop purchases wait for animation completion before applying logic

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: mostly resource/UI synchronization work
  - **Skills**: `[]`
    - Existing screen/resource patterns are sufficient
  - **Skills Evaluated but Omitted**:
    - `ui-ux-pro-max`: this is HUD behavior parity, not a redesign task

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with 1, 2, 8, 9)
  - **Blocks**: 7, 10
  - **Blocked By**: None

  **References**:
  - `src/screens/stats.rs:4` - current authoritative level money state
  - `src/demo/level.rs:175` - current HUD text update path that directly mirrors `stats.money`
  - `src/screens/shop.rs:263` - shop money display that also needs coherent behavior after purchases
  - `rebirth/GameStates.lua:554` - original HUD reads `player.money4View`, not raw money

  **Acceptance Criteria**:
  - [ ] HUD money display counts toward the actual total rather than jumping instantly
  - [ ] Shop screen still shows correct actual current money after spending

  **QA Scenarios**:
  ```text
  Scenario: Gameplay HUD money increments smoothly after reward
    Tool: interactive_bash
    Preconditions: run `cargo run`, enter gameplay, collect an item that awards money
    Steps:
      1. Capture the HUD immediately before and after reward resolution
      2. Observe whether displayed money counts upward instead of changing in one frame
    Expected Result: displayed money animates toward the new total while final value matches actual money
    Failure Indicators: instant jump, wrong final total, shop display desync later
    Evidence: .sisyphus/evidence/task-6-money-view.txt

  Scenario: Shop purchase still deducts exact real amount
    Tool: interactive_bash
    Preconditions: proceed to shop with known money amount
    Steps:
      1. Note displayed money before purchase
      2. Buy one item and verify resulting amount and selector/shop state
    Expected Result: purchase uses correct actual money and resulting displayed amount settles to the exact deducted total
    Evidence: .sisyphus/evidence/task-6-shop-money.txt
  ```

  **Commit**: YES
  - Message: `feat(ui): 恢复金币平滑计数显示`
  - Files: `src/screens/stats.rs`, `src/demo/level.rs`, `src/screens/shop.rs`
  - Pre-commit: `cargo run`

- [x] 7. Restore strength reward flow and player animation parity

  **What to do**:
  - When the original reward path grants strength, switch the player to `Strengthen` animation for the parity duration and then return cleanly to idle
  - Keep the current bonus/strength text or sprite presentation aligned with original timing and avoid conflicts with dynamite/grab-back states
  - Ensure strength gain still caps correctly and that reward/no-reward branches behave distinctly

  **Must NOT do**:
  - Do not leave the player stuck in `Strengthen`
  - Do not double-apply the strength gain or reward audio

  **Recommended Agent Profile**:
  - **Category**: `deep`
    - Reason: state interaction between reward, animation, timers, and HUD flow is easy to get subtly wrong
  - **Skills**: `[]`
    - Existing state machine code is the primary reference
  - **Skills Evaluated but Omitted**:
    - `ui-ux-pro-max`: animation timing/state fidelity matters more than design polish

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with 3, 4, 5, 10)
  - **Blocks**: 10
  - **Blocked By**: 5, 6

  **References**:
  - `src/demo/player.rs:178` - current player animation states already include `Strengthen`
  - `src/demo/hook.rs:506` - current reward handling and strength-branch logic
  - `src/demo/hook.rs:518` - current `Strength!` presentation point
  - `rebirth/GameStates.lua:536` - original show-strength timer and player animation switching behavior
  - `rebirth/main.lua:130` - original `playerStrengthenAnimation` frame sequence and timing

  **Acceptance Criteria**:
  - [ ] Strength reward branch visibly switches player animation to `Strengthen` for the intended timed window
  - [ ] After timer expiry, player animation returns to the correct non-strength state without residue

  **QA Scenarios**:
  ```text
  Scenario: Strength reward shows strengthen animation and resets cleanly
    Tool: interactive_bash
    Preconditions: run `cargo run`, trigger a reward branch that grants strength
    Steps:
      1. Collect the qualifying reward entity
      2. Observe player sprite frames during the strength window
      3. Wait for timer expiry and verify return to normal state
    Expected Result: player enters `Strengthen`, then exits back to idle/normal flow without getting stuck
    Failure Indicators: text appears but animation never changes, animation never resets, reward applies twice
    Evidence: .sisyphus/evidence/task-7-strength-anim.txt

  Scenario: Non-strength reward path does not trigger strengthen state
    Tool: interactive_bash
    Preconditions: same session, collect a reward with no strength outcome
    Steps:
      1. Collect a normal bonus item
      2. Observe player animation and reward UI
    Expected Result: no strengthen animation plays on normal bonus resolution
    Evidence: .sisyphus/evidence/task-7-no-strength.txt
  ```

  **Commit**: YES
  - Message: `feat(player): 恢复力量奖励动画流程`
  - Files: `src/demo/hook.rs`, `src/demo/player.rs`, possibly `src/demo/level.rs`
  - Pre-commit: `cargo run`

- [x] 8. Restore shopkeeper presentation and shop-state parity

  **What to do**:
  - Render the original shopkeeper using `shopkeeper_sheet.png` on the shop screen
  - Support at least the original idle/sad state presentation and align dialogue changes with buy/no-buy completion state
  - Keep existing shop input and item generation logic unless parity requires a local visual/state adjustment

  **Must NOT do**:
  - Do not redesign the shop layout away from the original-inspired composition
  - Do not entangle shopkeeper visuals with purchase logic more than necessary

  **Recommended Agent Profile**:
  - **Category**: `visual-engineering`
    - Reason: screen composition and sprite-state presentation dominate this task
  - **Skills**: `[]`
    - Existing sprite/UI conventions are enough
  - **Skills Evaluated but Omitted**:
    - `ui-ux-pro-max`: parity restoration should follow original layout, not invent a new visual language

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with 1, 2, 6, 9)
  - **Blocks**: 10
  - **Blocked By**: None

  **References**:
  - `src/screens/shop.rs:107` - current shop scene assembly entry point
  - `src/screens/shop.rs:433` - current finish-shopping dialogue state, which should drive shopkeeper mood
  - `rebirth/main.lua:135` - original shopkeeper atlas and idle/sad states
  - `rebirth/README.md` - screenshot references showing the shop composition expected from parity

  **Acceptance Criteria**:
  - [ ] Shop screen shows a shopkeeper character using the local spritesheet
  - [ ] Shopkeeper visual state updates appropriately for normal vs no-buy end states

  **QA Scenarios**:
  ```text
  Scenario: Shop screen shows shopkeeper during normal browsing
    Tool: interactive_bash
    Preconditions: run `cargo run`, reach the shop screen
    Steps:
      1. Enter shop after a level transition
      2. Capture the screen while browsing items with Left/Right
    Expected Result: shopkeeper is visible in idle state alongside dialogue bubble and items
    Failure Indicators: no shopkeeper, wrong atlas frame, layout overlap with selector/items
    Evidence: .sisyphus/evidence/task-8-shopkeeper-idle.txt

  Scenario: Exiting without purchase shows sad/no-buy state
    Tool: interactive_bash
    Preconditions: same shop session with no purchases made
    Steps:
      1. Press `Space` to finish shopping without buying
      2. Observe dialogue and shopkeeper state during finish timer
    Expected Result: dialogue and sprite reflect the sad/no-buy outcome before screen transition
    Evidence: .sisyphus/evidence/task-8-shopkeeper-sad.txt
  ```

  **Commit**: YES
  - Message: `feat(shop): 恢复店主形象与状态表现`
  - Files: `src/screens/shop.rs`, `src/config.rs`
  - Pre-commit: `cargo run`

- [x] 9. Restore transition-audio behavior parity

  **What to do**:
  - Extend audio handling so transition music behavior matches the original reference more closely, whether that means fade-out, play-to-completion, or another non-abrupt state-consistent handoff
  - Limit the behavior to relevant transition music paths; keep the audio API small and local to existing `audio.rs` usage patterns
  - Ensure state exits and repeated transitions do not stack duplicate transition handlers or lingering music players

  **Must NOT do**:
  - Do not build a full audio mixer framework
  - Do not change all sound-effect playback to use the transition-music path

  **Recommended Agent Profile**:
  - **Category**: `unspecified-high`
    - Reason: audio state transitions need careful lifecycle handling
  - **Skills**: `[]`
    - Existing audio/state code should drive the implementation
  - **Skills Evaluated but Omitted**:
    - `ui-ux-pro-max`: audio transition logic only

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 1 (with 1, 2, 6, 8)
  - **Blocks**: 10
  - **Blocked By**: None

  **References**:
  - `src/audio.rs:18` - current music bundle helpers and extension point for transition-aware handling
  - `src/audio.rs:62` - currently loaded music assets (`Goal`, `MadeGoal`)
  - `rebirth/main.lua:61` - original music resource set
  - `rebirth/GameStates.lua:521` - original goal/made-goal transition points where music behavior matters

  **Acceptance Criteria**:
  - [ ] Relevant transition music behavior matches the verified original reference more closely than the current abrupt handling
  - [ ] Re-entering transitions does not leave duplicate lingering music entities

  **QA Scenarios**:
  ```text
  Scenario: Goal transition audio behaves like the original reference
    Tool: interactive_bash
    Preconditions: run `cargo run`, reach a gameplay-to-goal transition with music active
    Steps:
      1. Trigger the relevant transition
      2. Listen/observe the transition-audio behavior across the state change window
      3. Compare the result against the verified original reference notes used by the executor
    Expected Result: audio transition is no longer abrupt and matches the verified original behavior
    Failure Indicators: hard stop, duplicate music, behavior that contradicts verified reference notes
    Evidence: .sisyphus/evidence/task-9-transition-audio.txt

  Scenario: Repeated transition does not accumulate duplicate music instances
    Tool: interactive_bash
    Preconditions: restart or repeat a transition path
    Steps:
      1. Trigger the same transition twice across separate runs/screens
      2. Observe whether audio doubles or persists unexpectedly
    Expected Result: single correct music playback with clean cleanup each time
    Evidence: .sisyphus/evidence/task-9-no-duplication.txt
  ```

  **Commit**: YES
  - Message: `feat(audio): 对齐过场音频切换行为`
  - Files: `src/audio.rs`, relevant screen modules
  - Pre-commit: `cargo run`

- [ ] 10. Apply remaining timing and presentation parity polish

  **What to do**:
  - Normalize the known minor parity gaps: gameplay timer default/reset, skip-tip placement, hook reset after dynamite use, and any remaining shop/gameplay end-state presentation mismatches uncovered while implementing tasks 3-9
  - Keep this task narrowly scoped to already-identified original parity details; do not turn it into a new discovery sprint
  - Update in-code constants to match original values where the reference is explicit

  **Must NOT do**:
  - Do not add new parity goals not already identified in this plan
  - Do not refactor unrelated gameplay systems while touching constants or small UI transforms

  **Recommended Agent Profile**:
  - **Category**: `quick`
    - Reason: final parity cleanup across known constants and small branches
  - **Skills**: `[]`
    - No special skill needed
  - **Skills Evaluated but Omitted**:
    - `ui-ux-pro-max`: task is fidelity cleanup, not redesign

  **Parallelization**:
  - **Can Run In Parallel**: YES
  - **Parallel Group**: Wave 2 (with 3, 4, 5, 7)
  - **Blocks**: F1, F3, F4
  - **Blocked By**: 5, 7, 8, 9

  **References**:
  - `src/screens/stats.rs:22` - current timer default/reset value (`61.0`) to align with original `60`
  - `src/demo/level.rs:127` - current `Press Select to Skip` placement
  - `src/demo/hook.rs:219` - current dynamite-use hook reset branch
  - `src/screens/shop.rs:435` - end-state dialogue currently set without full sprite parity
  - `rebirth/GameStates.lua:558` - original time HUD and level/skip-tip placement data
  - `rebirth/GameStates.lua:563` - original skip-tip x/y location
  - `rebirth/Entities.lua:162` - original hook animation reset behavior after explosive use

  **Acceptance Criteria**:
  - [ ] Timer and skip-tip constants match original intent where explicitly known
  - [ ] Dynamite use resets hook presentation correctly
  - [ ] No obvious remaining parity gaps from the identified list remain unaddressed

  **QA Scenarios**:
  ```text
  Scenario: HUD timing and skip-tip placement match parity expectations
    Tool: interactive_bash
    Preconditions: run `cargo run`, reach gameplay and then reach the goal threshold
    Steps:
      1. Observe initial timer value at gameplay start
      2. Reach goal and verify the skip-tip appears in the intended top HUD region
    Expected Result: timer starts/reset at parity value and skip-tip placement no longer looks offset from original reference
    Failure Indicators: timer still starts at 61, skip-tip remains visibly misplaced
    Evidence: .sisyphus/evidence/task-10-hud-polish.txt

  Scenario: Dynamite use returns hook to clean idle presentation
    Tool: interactive_bash
    Preconditions: same session, have a grabbed entity and dynamite available
    Steps:
      1. Use dynamite during reel-back
      2. Observe hook atlas/state after entity destruction and before next fire
    Expected Result: hook returns to the correct idle presentation without stale grabbed-state visuals
    Evidence: .sisyphus/evidence/task-10-hook-reset.txt
  ```

  **Commit**: YES
  - Message: `fix(parity): 收口原版细节差异`
  - Files: `src/screens/stats.rs`, `src/demo/level.rs`, `src/demo/hook.rs`, `src/screens/shop.rs`
  - Pre-commit: `cargo run`

---

## Final Verification Wave

- [ ] F1. **Plan Compliance Audit** — `oracle`
  Verify every listed parity gap has a corresponding implemented behavior, using the original references and evidence files. Confirm no guardrail violations (no architecture rewrite, no extra features).
  Output: `Must Have [N/N] | Must NOT Have [N/N] | Tasks [N/N] | VERDICT`

- [ ] F2. **Code Quality Review** — `unspecified-high`
  Run `cargo fmt --all -- --check`, `cargo clippy --locked --workspace --all-targets --all-features`, `cargo test --locked --workspace --all-targets`, `cargo build --release`. Review touched files for dead branches, duplicate constants, stale comments, or state-leak risks.
  Output: `Fmt [PASS/FAIL] | Clippy [PASS/FAIL] | Tests [PASS/FAIL] | Build [PASS/FAIL] | VERDICT`

- [ ] F3. **Real Gameplay QA Replay** — `unspecified-high`
  Launch `cargo run`, replay at least: one BigGold pickup, one moving-entity pickup, one strength reward, one shop buy flow, one shop no-buy flow, and one transition path with fade behavior. Save all evidence to `.sisyphus/evidence/final-qa/`.
  Output: `Scenarios [N/N pass] | Integration [PASS/FAIL] | Edge Cases [PASS/FAIL] | VERDICT`

- [ ] F4. **Scope Fidelity Check** — `deep`
  Compare the actual diff to this plan and verify work stayed within original-parity scope. Flag any implementation that added non-original features or unrelated refactors.
  Output: `Tasks [N/N compliant] | Scope Creep [NONE/N issues] | Unaccounted Files [NONE/N] | VERDICT`

---

## Commit Strategy

- **1**: `feat(config): 补齐原版缺失资源映射` — `src/config.rs` — `cargo clippy --locked --workspace --all-targets --all-features`
- **2**: `feat(demo): 增加通用特效动画支撑` — shared FX files — `cargo test --locked --workspace --all-targets`
- **3**: `feat(gameplay): 恢复大金块闪光与标准爆炸特效` — tasks 3-4 grouped — `cargo run`
- **4**: `fix(gameplay): 对齐抓取表现与力量奖励流程` — tasks 5-7 grouped — `cargo run`
- **5**: `feat(shop): 恢复店主与音频过场表现` — tasks 8-9 grouped — `cargo run`
- **6**: `fix(parity): 收口原版细节差异` — task 10 — full verification command set

---

## Success Criteria

### Verification Commands
```bash
cargo fmt --all -- --check
cargo clippy --locked --workspace --all-targets --all-features
cargo test --locked --workspace --all-targets
cargo build --release
cargo run
```

### Final Checklist
- [ ] All identified visible parity gaps in this plan are implemented or explicitly proven obsolete
- [ ] BigGold sparkle, standard explosive FX, grabbed-object rotation, money-view animation, strength presentation, shopkeeper state, and transition-audio parity are all observable in-game
- [ ] Timer/skip-tip/hook-reset minor parity details are aligned
- [ ] No unrelated features or architecture rewrites were introduced
- [ ] All verification commands pass
