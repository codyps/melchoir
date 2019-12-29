set architecture arm
target extended-remote localhost:2331
monitor reset
monitor semihosting enable
monitor semihosting breakOnError
monitor semihosting IOclient 3
load
continue
