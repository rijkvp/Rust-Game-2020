# Entities

### Player

- transform
- sprite
- health
- physicsTag
- movable/controllable

### Enemy

- transform
- sprite
- health
- physicsTag
- firingAI/meleeAI

### Button

- transform
- text
- state
- texture?

### Bullet

- transform (direction is stored in rotation of transform! degrees only calculated one time!)
- physicsTag
- lifetime

### Tile

- sprite
- physicsTag?! only if solid!

### Camera

- look into SDL2 viewport docs = NOPE INTEGRATE SELf

# Coponents

### Health

- hp: f32
- is_dead: bool

### Lifetime

- time_left
- is_dead

### Physics

- velocity: f32
- satic/dynamic
