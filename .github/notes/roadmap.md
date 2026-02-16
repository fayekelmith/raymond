# Raymond Roadmap (Draft)

This roadmap keeps the project focused, measurable, and portfolio-ready. We will revise dates and scope as we learn.

## Vision
Build a reliable, modular mobile robot with:
- A robust movement spine (Pico W)
- A higher-level brain (Jetson Orin Nano)
- A dashboard for control and analytics
- Clear documentation, testing, and iteration logs

## Guiding Principles
- Learn by building in small, verifiable steps
- Prefer stable, wired communication for MVP
- Document every milestone with evidence (video, logs, notes)
- Build for safety first, performance second

## System Overview
- **Spine (Pico W)**: motor control, real-time IO, safety
- **Brain (Jetson)**: sensor fusion, navigation, autonomy
- **Dashboard**: telemetry, controls, analytics
- **Shared**: common types, protocols, utilities

## Communication Strategy
- **MVP**: UART (wired), stable and simple
- **Phase 3+**: optional Wi-Fi for remote use
- **Future**: consider CAN/RS-485 for multi-node reliability

## Hardware Stack

### Available Now
- Motors + chassis (4WD omni wheels, 12V motors with encoders)
- Motor driver: **TB6612FNG** (2×, for 4 independent motors)
- IMU: **BNO055** (I2C)
- Lidar: **RPLidar A1M8**
- Camera: **Raspberry Pi AI Camera**
- Connectivity: **SIM7600G-H 4G modem**
- Audio input: **Microphone** (model TBD)
- LED Matrix: **64×64 array** (for emotion display)
- Power: 11V battery + voltage regulator

### To Acquire
- ToF sensors (multiple for 360° coverage)
- Speaker for TTS output
- Charging dock (optional, future)

---

# Phase 0 — Foundations
**Goal**: Prepare wiring, power, and dev workflow.

**Deliverables**
- Wiring diagram (power + signal)
- Pin map (Pico W GPIOs -> motor driver / sensors)
- Power budget and safety notes
- Flash + logging workflow confirmed

**Milestones**
- M0.1: Confirm stable 5V/3.3V rails for Pico W
- M0.2: UART wiring tested (loopback)
- M0.3: Basic project layout + run instructions

---

# Phase 1 — Spine MVP (Movement)
**Goal**: Control motors safely and predictably with TB6612FNG.

**Deliverables**
- TB6612FNG motor driver control (direction + PWM + STBY)
- Independent 4-motor control (omnidirectional capable)
- Safety stop command
- Simple UART command protocol
- Bench test logs

**Milestones**
- M1.1: Blink LED + basic task scheduling
- M1.2: One motor forward/back with TB6612FNG
- M1.3: All four motors independently controlled
- M1.4: Emergency stop command (via STBY pin)
- M1.5: Basic movement patterns (forward, strafe, rotate)

---

# Phase 2 — Spine Sensors (Encoders + IMU)
**Goal**: Read encoder and IMU data for accurate movement tracking.

**Deliverables**
- Motor encoder reading (all 4 motors)
- BNO055 IMU integration (I2C)
- Rich telemetry over UART (orientation, speed, encoder counts)
- Data structure for 3D visualization

**Milestones**
- M2.1: Read one motor encoder (A/B quadrature)
- M2.2: Read all four motor encoders
- M2.3: BNO055 orientation (quaternion + Euler angles)
- M2.4: UART telemetry packet format (position, orientation, motor states)
- M2.5: Encoder-based odometry calculations

---

# Phase 3 — Brain MVP (Jetson)
**Goal**: Basic teleop and telemetry ingestion.

**Deliverables**
- UART client on Jetson
- Telemetry display (CLI or small UI)
- Teleop commands sent to spine

**Milestones**
- M3.1: UART link stable
- M3.2: Teleop forward/back
- M3.3: Live telemetry display

---

# Phase 4 — Autonomy v1
**Goal**: Obstacle-aware movement.

**Deliverables**
- RPLidar integration on Jetson
- Basic obstacle stop/avoid
- Heading control with IMU

**Milestones**
- M4.1: Lidar scan ingest
- M4.2: Obstacle stop
- M4.3: Simple waypoint drive

---

# Phase 5 — Dashboard MVP
**Goal**: Monitor + control from a dashboard with 3D visualization.

**Deliverables**
- Status panel (battery, speed, errors, orientation)
- Manual control UI (keyboard/gamepad)
- **3D robot visualization** (real-time orientation from BNO055)
- Session logs and replay

**Milestones**
- M5.1: Status view (text-based telemetry)
- M5.2: Manual drive controls (web interface)
- M5.3: 3D robot model rendering (Three.js or similar)
- M5.4: Real-time orientation sync (quaternion from BNO055)
- M5.5: Session logging and replay with 3D playback

---

# Phase 6 — ToF Sensors + Close-Range Safety
**Goal**: Add Time-of-Flight sensors for obstacle detection.

**Deliverables**
- Multiple ToF sensors (front, sides, possibly rear)
- Close-range obstacle detection (< 2m)
- Cliff detection (down-facing ToF)
- Integration with navigation system

**Milestones**
- M6.1: Single ToF sensor integration and testing
- M6.2: Multiple ToF sensors (I2C multiplexing or different addresses)
- M6.3: Obstacle detection logic (stop/slow on proximity)
- M6.4: Cliff detection and emergency stop
- M6.5: Sensor fusion with Lidar data

---

# Phase 7 — LED Matrix Emotions
**Goal**: Add visual personality and status feedback.

**Deliverables**
- 64×64 LED matrix driver integration
- Emotion rendering library (happy, thinking, low battery, etc.)
- Status indicators (mode, errors, activity)
- Animation system

**Milestones**
- M7.1: LED matrix basic control (test patterns)
- M7.2: Emotion sprite/animation library
- M7.3: Status indicators (battery, connectivity, mode)
- M7.4: Dynamic emotions based on robot state
- M7.5: Sync with audio output (when available)

---

# Phase 8 — 4G Connectivity
**Goal**: Enable remote operations and monitoring.

**Deliverables**
- SIM7600G-H modem integration
- Remote telemetry streaming
- Remote command interface
- Video streaming (optional)
- Cloud logging

**Milestones**
- M8.1: Modem initialization and network connection
- M8.2: MQTT or WebSocket telemetry streaming
- M8.3: Remote control interface (web or app)
- M8.4: Camera streaming over 4G
- M8.5: Cloud data logging and analytics

---

# Phase 9 — Voice & Local AI (Brain Intelligence)
**Goal**: Enable voice interaction with local LLM.

**Deliverables**
- Microphone integration and audio capture
- Wake word detection
- Speech-to-text (Whisper on Jetson)
- Local LLM for reasoning (TinyLLaMA/Phi-2)
- Text-to-speech output (requires speaker)
- Natural language command parsing

**Milestones**
- M9.1: Microphone audio capture
- M9.2: Wake word detection ("Hey Raymond")
- M9.3: Whisper STT integration on Jetson
- M9.4: Basic command parsing (go, stop, follow)
- M9.5: Local LLM integration (conversational AI)
- M9.6: TTS output (once speaker acquired)
- M9.7: LED emotions synced with speech

---

# Phase 10 — Advanced Autonomy
**Goal**: Full autonomous navigation with AI reasoning.

**Deliverables**
- SLAM (Simultaneous Localization and Mapping)
- Named waypoint navigation
- Person following (camera + AI)
- Gesture recognition
- Multi-room navigation
- Charging dock auto-return

**Milestones**
- M10.1: SLAM implementation (map building)
- M10.2: Named waypoints ("go to kitchen")
- M10.3: Person detection and following
- M10.4: Gesture recognition (wave, point)
- M10.5: Multi-room autonomous navigation
- M10.6: Charging dock integration

---

# Phase 11 — Robustness + Polish
**Goal**: Production-quality reliability and error handling.

**Deliverables**
- Comprehensive error handling + watchdog
- Automatic recovery from failures
- Performance optimization
- Documented tests + benchmarks
- Portfolio documentation and demos

**Milestones**
- M11.1: Watchdog timers and safe shutdown
- M11.2: Automatic error recovery
- M11.3: Performance profiling and optimization
- M11.4: Integration tests for all subsystems
- M11.5: Portfolio-ready documentation and videos

---

## Three-Month Sprint Focus

This is your 3-month challenge to master the fundamentals and build momentum.

**Month 1: Spine Foundations + Movement**
- Phase 0 completed
- Phase 1 completed

**Month 2: Sensors + Brain MVP**
- Phase 2 completed
- Phase 3 completed

**Month 3: Autonomy + Dashboard MVP**
- Phase 4 completed
- Phase 5 completed (including 3D visualization)

---

## Open Decisions (We Will Confirm)
- Exact ToF sensor model + quantity
- Speaker model for TTS
- Final UART packet format
- LED matrix mounting location (brain vs standalone)
- Microphone array configuration
- LLM model selection (based on Jetson performance)
- Final power distribution approach
- Motor wiring polarity

## Evidence Checklist (For Portfolio)
- Short demo videos for each milestone
- Wiring diagrams + pin maps
- Sensor accuracy notes
- Failure notes and fixes

---

If anything changes, update this file first so our plan stays grounded.
