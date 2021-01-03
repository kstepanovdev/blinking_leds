# blinking_leds


1. udev
Need to be careful with udev rules. I needed to set it to:
```
# STM32F4 - ST-Link/V2-1
ATTRS{idVendor}=="0483", ATTRS{idProduct}=="374b", MODE:="0666" # pay attention to version of idProduct. 374b, not 3748. Read what lsusb | grep -i stm says.
```
Because my board has an STLINK <b>2.1<b> debugger.

2. openocd
Also, I monkey-patched this file `/usr/share/openocd/scripts/board/stm32f4discovery.cfg` accordingly to version of debugger, otherwise I get following error:

```
Error: open failed
in procedure 'init'
in procedure 'ocd_bouncer'
```

Be careful, when you read a tutorial from https://docs.rust-embedded.org/discovery, because it's brilliant, but it's mainly about cortex F3.
