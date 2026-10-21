# Baremetal Trackpad RTOS Roadmap

## Core Kernel and Context Switching
- [ ] Task Control Block
- [ ] Task Stack initialization & Exception Framing
- [ ] PendSV Context Switcher
- [ ] Critical Section Replacement

### Testing
- [ ] Allocate 2 separate stacks for `fn task_a()` and `fn task_b()`
- [ ] Trigger a PendSV interrupt and have the program not crash
- [ ] Have 2 tasks, one that writes it in a critical section and one that reads it (there should be no skipping or race conditions)

## Kernel Primitives and Synchronization
- [ ] Preemptive Task Scheduler using SysTick interrupts
- [ ] Synchronization Primitives
    - [ ] Blocking Mutex: Put waiting tasks into Blocked state
    - [ ] Semaphores/Mutexes: Add support for priority inheritance
    - [ ] Messaging/Queues: Create fixed-capacity lock-free bounded SPCS/MPSC queues

## Hardware Drivers and Trackpad Integration
- [ ] Bus Communication Driver
- [ ] Interrupt Driven Input Task

## USB Stack and HID Class Implementation
- [ ] USBD Hardware Bringup (using `usb-device` and `nrf-usbd` crates)
- [ ] Implement State Machine for USB Control Endpoint (using `usbd-hid` crate)
- [ ] USB Transmission Task

## Integration, Optimization and Power Management
- [ ] End-to-end Pipeline Wiring
- [ ] Power Efficiency and Idle Loop
- [ ] Real-Time Verification: Make sure it's responsive enough

# Extensions

## Custom Casing and PCB

## IMU Integration

## Battery and Bluetooth Support

## Configuration App

