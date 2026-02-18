# Motor Test Example

**Purpose:** Working reference for 4-motor TB6612FNG control on Raspberry Pi Pico W.

## What This Tests

- Individual motor control (FL, FR, RL, RR)
- Speed modulation via software PWM (20%, 50%, 80%)
- Direction control (forward, reverse, brake)
- Emergency stop via STBY pins
- All motors coordinated forward/reverse

## Hardware Requirements

- Raspberry Pi Pico W
- 2× TB6612FNG motor drivers
- 4× DC motors (GB37-520 or similar)
- 11-12V battery
- 5V regulator (for Pico VSYS)

## Pin Configuration

Matches `spine-config.md`:

### TB6612FNG #1 (Front Motors)
- STBY: GPIO1
- Front-Left: PWMA=GPIO2, AIN1=GPIO3, AIN2=GPIO6
- Front-Right: PWMB=GPIO7, BIN1=GPIO10, BIN2=GPIO11

### TB6612FNG #2 (Rear Motors)
- STBY: GPIO0
- Rear-Left: PWMA=GPIO20, AIN1=GPIO21, AIN2=GPIO22
- Rear-Right: PWMB=GPIO26, BIN1=GPIO27, BIN2=GPIO28

## Test Sequence

### Phase 1: Basic Movement
1. Forward @ 50% (3s)
2. Reverse @ 50% (3s)

### Phase 2: Arc Turns (smooth curves)
3. Gentle arc right: left=60%, right=30% (4s)
4. Gentle arc left: right=60%, left=30% (4s)

### Phase 3: Pivot Turns (sharp, one side stationary)
5. Pivot right: left wheels @ 50%, right stopped (3s)
6. Pivot left: right wheels @ 50%, left stopped (3s)

### Phase 4: Spin Turns (zero-radius rotation)
7. Spin clockwise: left forward, right reverse @ 40% (3s)
8. Spin counter-clockwise: left reverse, right forward @ 40% (3s)

### Phase 5: Safety
9. Emergency stop (STBY low, 2s pause, re-enable)
10. Loop repeats

## Code Structure

```
src/
├── main.rs           # Test sequence runner
├── lib.rs            # Module exports
└── motor_control.rs  # Reusable motor control abstractions
```

### Motor Control API

**Basic movement:**
- `forward(motors, speed, duration)` - All wheels forward
- `reverse(motors, speed, duration)` - All wheels reverse
- `brake(motors)` - Stop all motors

**Arc turns (smooth curves):**
- `arc_right(motors, left_speed, right_speed, duration)` - Left faster than right
- `arc_left(motors, right_speed, left_speed, duration)` - Right faster than left

**Pivot turns (one side stationary):**
- `pivot_right(motors, speed, duration)` - Left wheels only
- `pivot_left(motors, speed, duration)` - Right wheels only

**Spin turns (zero-radius rotation):**
- `spin_clockwise(motors, speed, duration)` - Left forward, right reverse
- `spin_counterclockwise(motors, speed, duration)` - Left reverse, right forward

## How to Run

```bash
cd playground/motor-test
cargo run --release
```

## Turning Mechanics

Raymond uses **skid steering** (like a tank):

### Turn Right
- Left wheels: forward at speed X
- Right wheels: forward at speed X/2 (or stopped, or reverse for sharper turn)
- Result: robot pivots right

### Turn Left
- Left wheels: forward at speed X/2 (or stopped, or reverse)
- Right wheels: forward at speed X
- Result: robot pivots left

### Pivot in Place (zero-radius turn)
- Left wheels: forward
- Right wheels: reverse (same speed)
- Result: robot spins on center axis

### Smooth Arc Turns
- Both sides forward, but at different speeds
- Speed differential determines turn radius
- Example: left=80%, right=40% = gentle right curve

## Direction Logic (for GB37-520 motors)

```
Forward:  IN1=LOW,  IN2=HIGH
Reverse:  IN1=HIGH, IN2=LOW
Brake:    IN1=LOW,  IN2=LOW
```

## Notes

- Software PWM at 50Hz (20ms period)
- Direction wiring verified: Feb 18, 2026
- Encoder pins reserved but not used in this test: GPIO 12-19
- Safe startup: all motors braked, drivers disabled, 2s delay before enable

---

*Frozen working snapshot from Phase 1 motor testing*
