target extended-remote localhost:2331
monitor reset
monitor semihosting enable
monitor semihosting breakOnError 1
monitor semihosting IOclient 3
# <cpu-freq> <swo-freq> <port-mask> <mode>
# mode = 0 for uart, unclear what other values are
monitor SWO EnableTarget 0 0 0xf 0
load
continue
