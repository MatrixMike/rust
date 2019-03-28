#!/bin/bash
clang --target=msp430 "$(CFLAGS)" src/main.c -o bin/clang-blink.elf

