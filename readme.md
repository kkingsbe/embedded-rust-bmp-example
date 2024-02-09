# Multi-Mission Library (rs)
This crate is designed to hold composable, reusable driver code / wrappers for various sensors and hardware output devices. Currently it does not support differing STM32 models and only supports the STM32F4, however moving into the future that will be the plan.

## Repo Structure
### Top Level
The top level modules within the `src` directory are for the different categories of devices which may need driver code. Currently there is only a `sensor` module, but some other examples would be `stepper_motor`, `servo`, `display`, etc. 

### 2nd Level
Within the top level modules, there are more specific modules that are still somewhat generalized. Examples of modules which could go within the `sensor` module are `barometer`, `imu`, `magnetometer`, etc. These modules each define at least one trait to define the behavior which all submodules within them will follow. For example, wihtin the `barometer` module there is a `Barometer` trait which defines functionality that is used by all barometers, such as reading the pressure, temperature, and determining altitude.

### 3rd Level
Within the next level of nesting, we have modules for specific hardware. For example, at `sensor::barometer::bmp180` we have a module specifically for the BMP180 barometric pressure sensor. This module implements all of the traits defined in the modules above it, so in this specific case it would implement the `Sensor` and `Barometer` traits. This allows for extremely predictable functionality, and for switching between physical sensors without having to change *any* code (assuming there is already a driver written for the new sensor).

### Additional Comments
Of course everything previously described are simply the initial guidelines which are set out by one person (myself). The code structure and guidelines will (and should) shift as more people begin to contribute to this crate, and as different subteams use this code for their projects.

To see a template repo of how to set up the toolchain for programming an STM32 with Rust, take a look at [this link](https://github.com/kkingsbe/embedded-rust-stm)