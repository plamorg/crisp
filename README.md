<p align="center">
  <a href="" rel="noopener">
 <img src="assets/crisp.png" alt="crisp logo"></a>
</p>

<h3 align="center">crisp</h3>

<div align="center">

[![Hackathon](https://img.shields.io/badge/hackathon-Hack\&Roll2021-orange.svg)](https://hacknroll2021.devpost.com/)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](LICENSE)

</div>

***

<p align="center"> crisp sounds from a crisp can
    <br> 
</p>

## 📝 Table of Contents

*   [Setting up a local environment](#getting_started)
*   [Usage](#usage)
*   [Technology Stack](#tech_stack)
*   [Authors](#authors)

## 🏁 Getting Started <a name = "getting_started"></a>

### Prerequisites

*   Python
*   pip
    *   [pyserial](https://github.com/pyserial/pyserial)
    *   [mido](https://github.com/mido/mido)
*   Rust
*   Cargo
    *   [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2)
    *   [midir](https://github.com/Boddlnagg/midir)
*   Arduino (Mega 2560)

## 🎈 Usage <a name="usage"></a>

Simultaneously run the arduino program, Python MIDI drivers, and the visualizer.

Python MIDI Drivers:

    $ cd driver
    $ python interface.py

Visualizer:

    $ cd visualizer
    $ cargo run

## ⛏️ Built With <a name = "tech_stack"></a>

*   [Arduino](https://www.arduino.cc/) - Hardware
*   [rust-sdl2](https://github.com/Rust-SDL2/rust-sdl2) - Visualizer graphics
*   [midir](https://github.com/Boddlnagg/midir) - Visualizer MIDI processing
*   [pyserial](https://github.com/pyserial/pyserial) - Driver serial port access
*   [mido](https://github.com/mido/mido) - Driver MIDI objects

## ✍️ Authors <a name = "authors"></a>

*   [@claby2](https://github.com/claby2) - Visualizer and Driver
*   [@priime0](https://github.com/priime0) - Hardware
*   [@smjleo](https://github.com/smjleo) - Hardware and Firmware
