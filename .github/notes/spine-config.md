# Spine Configuration Guide

Complete hardware configuration for Raymond's spine (Raspberry Pi Pico W 2040) controlling 4 independent motors with encoders.

---

## Hardware Overview

### Components
- **MCU**: Raspberry Pi Pico W 2040
- **Motors**: 4× GB37-520 (12V, 0.7A, 330RPM, with encoders)
- **Motor Drivers**: 2× L298N (MVP) or 2× TB6612FNG (upgrade)
- **IMU**: BNO055 (I2C)
- **ToF Sensor**: TBD model
- **Power**: 11V battery + voltage regulator

### Motor Specifications
- Model: GB37-520
- Rated voltage: DC 12V
- Rated current: ≤0.7A
- Speed: 330RPM
- Rated power: ≈7.2W
- Encoder accuracy: 1320 CPR (counts per revolution)
- Pins per motor: 6 (M+, M-, VCC, GND, A, B)

---

## Power Distribution

### Battery to Components
```
11V Battery (+)
  ├─→ L298N #1 Motor Supply (12V/Vs)
  ├─→ L298N #2 Motor Supply (12V/Vs)
  └─→ Voltage Regulator Input (11V → 5V)

5V Regulator Output
  ├─→ Pico W VSYS (5V)
  └─→ Encoder Power (VCC on all 4 motors)

All GND → Common Ground (battery, drivers, Pico, sensors)
```

### Power Budget
| Component       | Voltage | Current (typical) | Current (peak) |
|-----------------|---------|-------------------|----------------|
| 4× Motors       | 12V     | 2.8A              | 3.6A           |
| Pico W          | 5V      | 200mA             | 300mA          |
| 4× Encoders     | 5V      | 40mA              | 60mA           |
| BNO055          | 3.3V    | 12mA              | 15mA           |
| ToF             | 3.3V    | 20mA              | 30mA           |

**Recommended battery**: 3S LiPo, ≥3000mAh for 1+ hour runtime

---

## Configuration Option 1: L298N (MVP)

### Why L298N for MVP?
- ✅ Readily available and cheap
- ✅ Simple to wire
- ✅ Good for learning basics
- ⚠️ Less efficient (BJT-based)
- ⚠️ 2-4V voltage drop
- ⚠️ Generates significant heat

### Motor Wiring (Independent Control)

**L298N Driver #1 (Front Motors)**
```
Channel A:
  OUT1 → Motor Front-Left M+
  OUT2 → Motor Front-Left M-

Channel B:
  OUT3 → Motor Front-Right M+
  OUT4 → Motor Front-Right M-
```

**L298N Driver #2 (Rear Motors)**
```
Channel A:
  OUT1 → Motor Rear-Left M+
  OUT2 → Motor Rear-Left M-

Channel B:
  OUT3 → Motor Rear-Right M+
  OUT4 → Motor Rear-Right M-
```

### Control Pin Mapping (Pico W → L298N)

**L298N #1 (Front Motors)**
| L298N Pin | Pico W GPIO | Function                  |
|-----------|-------------|---------------------------|
| ENA       | GPIO2       | PWM - Front-Left speed    |
| IN1       | GPIO3       | Front-Left direction bit 1|
| IN2       | GPIO6       | Front-Left direction bit 2|
| ENB       | GPIO7       | PWM - Front-Right speed   |
| IN3       | GPIO10      | Front-Right direction bit 1|
| IN4       | GPIO11      | Front-Right direction bit 2|
| GND       | GND         | Common ground             |

**L298N #2 (Rear Motors)**
| L298N Pin | Pico W GPIO | Function                  |
|-----------|-------------|---------------------------|
| ENA       | GPIO20      | PWM - Rear-Left speed     |
| IN1       | GPIO21      | Rear-Left direction bit 1 |
| IN2       | GPIO22      | Rear-Left direction bit 2 |
| ENB       | GPIO26      | PWM - Rear-Right speed    |
| IN3       | GPIO27      | Rear-Right direction bit 1|
| IN4       | GPIO28      | Rear-Right direction bit 2|
| GND       | GND         | Common ground             |

### L298N Notes
- Do **not** connect 5V pin (we're using EN pins for PWM)
- Heat sinks recommended
- Test on bench before full load

---

## Configuration Option 2: TB6612FNG (Upgrade)

### Why TB6612FNG?
- ✅ Much more efficient (MOSFET-based)
- ✅ Only ~0.5V voltage drop (vs 2-4V on L298N)
- ✅ Native 3.3V logic compatible
- ✅ Minimal heat generation
- ✅ Better low-speed control
- ✅ 1.2A continuous, 3.2A peak per channel

### Motor Wiring (Independent Control)

**TB6612FNG Driver #1 (Front Motors)**
```
Channel A:
  AO1 → Motor Front-Left M+
  AO2 → Motor Front-Left M-

Channel B:
  BO1 → Motor Front-Right M+
  BO2 → Motor Front-Right M-
```

**TB6612FNG Driver #2 (Rear Motors)**
```
Channel A:
  AO1 → Motor Rear-Left M+
  AO2 → Motor Rear-Left M-

Channel B:
  BO1 → Motor Rear-Right M+
  BO2 → Motor Rear-Right M-
```

### Control Pin Mapping (Pico W → TB6612FNG)

**TB6612FNG #1 (Front Motors)**
| TB6612 Pin | Pico W GPIO | Function                  |
|------------|-------------|---------------------------|
| STBY       | GPIO1       | Standby/Enable (HIGH=on)  |
| PWMA       | GPIO2       | PWM - Front-Left speed    |
| AIN1       | GPIO3       | Front-Left direction bit 1|
| AIN2       | GPIO6       | Front-Left direction bit 2|
| PWMB       | GPIO7       | PWM - Front-Right speed   |
| BIN1       | GPIO10      | Front-Right direction bit 1|
| BIN2       | GPIO11      | Front-Right direction bit 2|
| GND        | GND         | Common ground             |

**TB6612FNG #2 (Rear Motors)**
| TB6612 Pin | Pico W GPIO | Function                  |
|------------|-------------|---------------------------|
| STBY       | GPIO0       | Standby/Enable (HIGH=on)  |
| PWMA       | GPIO20      | PWM - Rear-Left speed     |
| AIN1       | GPIO21      | Rear-Left direction bit 1 |
| AIN2       | GPIO22      | Rear-Left direction bit 2 |
| PWMB       | GPIO26      | PWM - Rear-Right speed    |
| BIN1       | GPIO27      | Rear-Right direction bit 1|
| BIN2       | GPIO28      | Rear-Right direction bit 2|
| GND        | GND         | Common ground             |

### TB6612FNG Standby (STBY) Options

**Option A: Always Enabled (simplest)**
```
STBY → 3.3V or 5V (tied high)
```

**Option B: Software Control (recommended)**
```
TB6612 #1 STBY → GPIO1
TB6612 #2 STBY → GPIO0
```
Allows emergency stop by pulling STBY low.

**Option C: Shared Control**
```
Both drivers STBY → GPIO1 (parallel)
```

---

## Encoder Wiring (Same for Both Drivers)

Each motor has 6 pins:
- **M+, M-**: Motor power (to driver outputs)
- **VCC**: Encoder power (5V from regulator)
- **GND**: Encoder ground (common GND)
- **A, B**: Quadrature encoder outputs (to Pico GPIO)

### Encoder Pin Assignments

| Motor        | A Signal | B Signal | VCC  | GND        |
|--------------|----------|----------|------|------------|
| Front-Left   | GPIO12   | GPIO13   | 5V   | Common GND |
| Rear-Left    | GPIO14   | GPIO15   | 5V   | Common GND |
| Front-Right  | GPIO16   | GPIO17   | 5V   | Common GND |
| Rear-Right   | GPIO18   | GPIO19   | 5V   | Common GND |

**Note**: Encoders typically require 5V. Verify with your motor datasheet.

---

## Sensor & Communication Pins

### I2C Bus (BNO055, ToF)
| Function | Pico W GPIO |
|----------|-------------|
| SDA      | GPIO4       |
| SCL      | GPIO5       |

### UART (Brain ↔ Spine)
| Function | Pico W GPIO |
|----------|-------------|
| TX       | GPIO8       |
| RX       | GPIO9       |

---

## Complete Pin Map Summary

### For L298N Configuration
| GPIO | Function                  |
|------|---------------------------|
| 0    | (Available)               |
| 1    | (Available)               |
| 2    | L298N #1 ENA (FL PWM)     |
| 3    | L298N #1 IN1 (FL)         |
| 4    | I2C SDA                   |
| 5    | I2C SCL                   |
| 6    | L298N #1 IN2 (FL)         |
| 7    | L298N #1 ENB (FR PWM)     |
| 8    | UART1 TX (to Brain)       |
| 9    | UART1 RX (from Brain)     |
| 10   | L298N #1 IN3 (FR)         |
| 11   | L298N #1 IN4 (FR)         |
| 12   | Encoder FL-A              |
| 13   | Encoder FL-B              |
| 14   | Encoder RL-A              |
| 15   | Encoder RL-B              |
| 16   | Encoder FR-A              |
| 17   | Encoder FR-B              |
| 18   | Encoder RR-A              |
| 19   | Encoder RR-B              |
| 20   | L298N #2 ENA (RL PWM)     |
| 21   | L298N #2 IN1 (RL)         |
| 22   | L298N #2 IN2 (RL)         |
| 26   | L298N #2 ENB (RR PWM)     |
| 27   | L298N #2 IN3 (RR)         |
| 28   | L298N #2 IN4 (RR)         |

### For TB6612FNG Configuration
| GPIO | Function                  |
|------|---------------------------|
| 0    | TB6612 #2 STBY            |
| 1    | TB6612 #1 STBY            |
| 2    | TB6612 #1 PWMA (FL)       |
| 3    | TB6612 #1 AIN1 (FL)       |
| 4    | I2C SDA                   |
| 5    | I2C SCL                   |
| 6    | TB6612 #1 AIN2 (FL)       |
| 7    | TB6612 #1 PWMB (FR)       |
| 8    | UART1 TX                  |
| 9    | UART1 RX                  |
| 10   | TB6612 #1 BIN1 (FR)       |
| 11   | TB6612 #1 BIN2 (FR)       |
| 12   | Encoder FL-A              |
| 13   | Encoder FL-B              |
| 14   | Encoder RL-A              |
| 15   | Encoder RL-B              |
| 16   | Encoder FR-A              |
| 17   | Encoder FR-B              |
| 18   | Encoder RR-A              |
| 19   | Encoder RR-B              |
| 20   | TB6612 #2 PWMA (RL)       |
| 21   | TB6612 #2 AIN1 (RL)       |
| 22   | TB6612 #2 AIN2 (RL)       |
| 26   | TB6612 #2 PWMB (RR)       |
| 27   | TB6612 #2 BIN1 (RR)       |
| 28   | TB6612 #2 BIN2 (RR)       |

---

## Motor Control Logic (H-Bridge)

Same for both L298N and TB6612FNG:

### Direction Control
| IN1/AIN1 | IN2/AIN2 | Result     |
|----------|----------|------------|
| LOW      | LOW      | Brake      |
| LOW      | HIGH     | Reverse    |
| HIGH     | LOW      | Forward    |
| HIGH     | HIGH     | Brake      |

### Speed Control
- Use PWM on EN (L298N) or PWM (TB6612FNG)
- 0% duty = stopped
- 100% duty = full speed
- Typical range: 30-100% (below 30% motors may stall)

---

## Code Migration (L298N → TB6612FNG)

### Key Differences in Code

**L298N:**
```rust
// No STBY needed, driver always on
en_pin.set_duty(speed);
in1_pin.set_high();
in2_pin.set_low();
```

**TB6612FNG:**
```rust
// Must enable STBY first
stby_pin.set_high();

// Then control as before
pwm_pin.set_duty(speed);
ain1_pin.set_high();
ain2_pin.set_low();

// Emergency stop
stby_pin.set_low(); // instantly disables all motors
```

### Pin Abstraction (Recommended)
Create a motor driver trait that abstracts both:
```rust
trait MotorDriver {
    fn enable(&mut self);
    fn disable(&mut self);
    fn set_speed(&mut self, motor: Motor, speed: u16);
    fn set_direction(&mut self, motor: Motor, dir: Direction);
}
```

This way, switching drivers only requires changing the implementation, not the control logic.

---

## Safety Checklist

- [ ] All grounds connected (battery, drivers, Pico, sensors)
- [ ] Encoder VCC at correct voltage (5V typical)
- [ ] Motor driver heat sinks attached
- [ ] Battery voltage appropriate (11-12V for 12V motors)
- [ ] Emergency stop accessible (STBY pin or command)
- [ ] Test motors without wheels first (bench test)
- [ ] Verify encoder direction matches motor direction
- [ ] Check for shorts before applying power
- [ ] Current limiting or fuse on battery connection

---

## Testing Sequence

### Phase 0 Test
1. Power rails test (verify 5V, 3.3V, 12V)
2. Ground continuity test
3. LED blink test (Pico alive)

### Phase 1 Test
1. One motor forward (no encoder)
2. One motor reverse
3. One motor PWM speed control
4. All four motors individually
5. Emergency stop command

### Phase 2 Test
1. One encoder reading
2. All four encoders
3. Encoder + motor correlation
4. IMU data reading
5. ToF data reading

---

## Upgrade Path

### When to Upgrade from L298N to TB6612FNG?

**Upgrade if you experience:**
- Excessive heat on drivers
- Poor low-speed control
- Battery draining too fast
- Motors not reaching expected speed (voltage drop)

**Upgrade is easy because:**
- Pin mapping is almost identical
- Code changes are minimal (add STBY control)
- Wiring is the same (just different labels)

---

## Notes for Future

- Consider adding current sensing for better motor control
- May add additional ToF sensors (front/back/sides)
- Encoder data can be used for odometry
- BNO055 + encoders = accurate position tracking

---

*Last updated: Phase 0 planning*
