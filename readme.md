## Debugging:
1. Install VSCode
2. Install `probe-rs` with the following command:
   - `cargo install probe-rs --features cli`
3. Add the neccesary build target with the following command:
    - `rustup target install thumbv7m-none-eabi`
4. The rest "should" already be setup & work. You can add a new breakpoint by clicking on the left margin next to any line of code.

## Setting up debugging for a new project
To set up debugging within a new project, you can use the `launch.json` and `tasks.json` files as a template. The main important points to remember are the following:
- To debug you **must** be using a build which has no link-time optimizations or compiler optimizations. Easiest way to guarantee this is to just use a non-release build.

### Automating the build process:
To automate the build process, you can use vscodes "tasks" feature. It allows you to define different tasks within the `tasks.json` file, which are just console commands. These tasks can then be invoked from launch scripts within the `launch.json` file. This repo has 4 tasks defined, for preforming debug & release builds & deployments. The debug build task is then called from the `launch.json` before it begins the debugging process.

## Automated Testing
In order to prevent flawed code from making it onto a flight vehicle, and to make it easier to know when bad code made it into the codebase, I *strongly* recommend using Github actions to automatically build the codebase on each commit. It is also able to run any tests which are specified in the codebase. You can use the file located at `.github/workflows/rust.yml` as a starting point for your workflow.

## Additional Resources
[Rust on an STM32 Microcontroller (Medium.com)](https://medium.com/digitalfrontiers/rust-on-a-stm32-microcontroller-90fac16f6342)

[Rust on STM32: Getting Started (jonathanklimt.de)](https://jonathanklimt.de/electronics/programming/embedded-rust/rust-on-stm32-2/)

[probe.rs documentation](https://probe.rs/docs/getting-started/installation/)