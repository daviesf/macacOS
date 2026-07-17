# macacOS Roadmap

> **Status:** Planning
> **Current Version:** Pre-Alpha
> **Architecture:** x86-64 (UEFI)
> **Future Support:** ARM64 (AArch64)

---

# Vision

macacOS is an educational operating system written primarily in Rust.

Its purpose is to understand how a modern operating system works by implementing every major subsystem from scratch.

The project values correctness, readability and documentation over premature optimization.

---

# Development Principles

* Build incrementally.
* Every milestone must be bootable.
* Every subsystem must be documented.
* Prefer simple implementations before optimized ones.
* Keep architecture-specific code isolated.

---

# Milestones

## Pre-Alpha

### Project Foundation

* [ ] Define project philosophy
* [x] Choose programming language (Rust)
* [x] Choose kernel architecture (Modular Monolithic)
* [x] Choose firmware target (UEFI)
* [x] Plan ARM64 portability
* [x] Create repository structure

---

# v0.1 — Bootloader

## Goal

Run our own bootloader through UEFI.

### Features

* [ ] UEFI application
* [ ] Bootable EFI image
* [ ] Print text on screen
* [ ] Build automation
* [ ] Boot successfully in QEMU

**Expected Result**

```
Hello from macacOS Bootloader!
```

---

# v0.2 — Kernel

## Goal

Load and execute the kernel.

### Features

* [ ] Bootloader loads kernel into memory
* [ ] Transfer execution to kernel
* [ ] Basic kernel entry point
* [ ] Panic handler

**Expected Result**

```
Hello from macacOS Kernel!
```

---

# v0.3 — Terminal

## Goal

Provide basic text output.

### Features

* [ ] Text renderer
* [ ] Cursor
* [ ] Screen clearing
* [ ] Text colors

---

# v0.4 — Memory

## Goal

Implement memory management.

### Features

* [ ] Physical memory map
* [ ] Heap initialization
* [ ] Global allocator
* [ ] malloc/free equivalent

---

# v0.5 — Interrupts

## Goal

Handle CPU interrupts.

### Features

* [ ] GDT
* [ ] IDT
* [ ] Exception handlers
* [ ] Timer interrupt

---

# v0.6 — Keyboard

## Goal

Read keyboard input.

### Features

* [ ] PS/2 keyboard driver
* [ ] Key decoding
* [ ] Terminal input

---

# v0.7 — Shell

## Goal

Interact with the operating system.

### Features

* [ ] Command parser
* [ ] Built-in commands
* [ ] Command history

Example:

```
> help
> version
> clear
```

---

# v0.8 — File System

## Goal

Read and write files.

### Features

* [ ] Virtual File System (VFS)
* [ ] Initial filesystem support
* [ ] File API

---

# v0.9 — Processes

## Goal

Support multitasking.

### Features

* [ ] Scheduler
* [ ] Context switching
* [ ] Process management
* [ ] User mode preparation

---

# v1.0 — First Usable Release

## Goal

Deliver a minimal usable operating system.

### Features

* [ ] Interactive shell
* [ ] Stable memory management
* [ ] Keyboard support
* [ ] File system
* [ ] Basic multitasking

---

# Future Versions

## Graphics

* Framebuffer
* Window manager
* Desktop environment
* GUI toolkit

---

## Drivers

* SATA
* NVMe
* USB
* Audio
* Mouse
* Network

---

## Networking

* Ethernet
* IPv4
* TCP
* UDP
* DNS

---

## User Space

* ELF loader
* Dynamic memory
* Standard library
* User applications

---

## ARM64 Port

* Boot on AArch64
* Architecture abstraction
* Shared kernel components

---

# Long-Term Goals

* Boot on real hardware.
* Maintain clean and well-documented source code.
* Keep the project educational.
* Document every architectural decision.
* Produce a complete operating system built from scratch.

---

# Current Focus

**Current milestone:** **v0.1 — Bootloader**

The next objective is to build a UEFI bootloader capable of running inside QEMU and displaying a message on the screen.
