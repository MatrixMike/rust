#!/bin/bash
clang --target=MSP430 "$(CFLAGS)" .c -o bin/clang-blink.elf

