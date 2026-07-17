# macacOS Architecture

## Overview

macacOS is an educational operating system written primarily in Rust.

The project's goal is to understand how modern operating systems work by implementing every major subsystem from scratch.

Performance is important, but readability and learning always take priority.

---

## Philosophy

- Learn before optimizing.
- Keep the architecture simple.
- Every subsystem should be understandable.
- Every implementation decision should be documented.
- Build incrementally with runnable milestones.

---

## Kernel

Architecture:
- Modular Monolithic Kernel

Responsibilities:
- Memory management
- Process management
- Scheduler
- Interrupt handling
- Drivers
- Virtual File System
- System calls

---

## Supported Architectures

Current:
- x86-64

Planned:
- ARM64 (AArch64)

Architecture-specific code must remain isolated under `arch/`.

---

## Firmware

Target firmware:

- UEFI

Legacy BIOS support is not planned.

---

## Boot Process

Power On

↓

UEFI Firmware

↓

macacOS Bootloader

↓

macacOS Kernel

↓

Terminal

↓

GUI (future)

---

## Programming Languages

Primary:
- Rust

Secondary:
- Assembly (only when required)

---

## Project Goals

- Build every major OS subsystem.
- Keep documentation synchronized with the implementation.
- Produce a clean, maintainable codebase.