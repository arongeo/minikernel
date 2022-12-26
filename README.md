# minikernel

A small kernel (if you can call it that) for the Raspberry Pi 4 written in Rust. Definitely not the best kernel (again, if you can call it that) you'll ever see, a lot of the solutions aren't nearly as thought through as I would like them to be, but this is what I could write now, looking forward to improve :)

## TODO
- GPIO Input
- filesystem access
- system calls
- miniUART file transfer
- etc.

## DONE
- miniUART output
- miniUART input
- GPIO output

## References
- [BCM2711 Datasheet](https://datasheets.raspberrypi.com/bcm2711/bcm2711-peripherals.pdf)

## Thank you to:
- [isometimes](https://github.com/isometimes) for [rpi4os.com](https://www.rpi4os.com/)
- Everyone who worked on [rust-embedded/rust-raspberrypi-OS-tutorials](https://github.com/rust-embedded/rust-raspberrypi-OS-tutorials)
- Everyone who helped by answering my or someone else's question about anything connected to the topic
