# Boot Code

import machine
import sys
import gc
import math
import cmath
import array
import struct
import json
import re
import io
import collections
import heapq
import binascii
import hashlib
import random
import time
import select
import errno
import micropython
import uctypes
import ubinascii
import uhashlib
import uheapq
import uio
import ujson
import uos
import ure
import uselect
import ustruct
import utime
import rp2
import _rp2
import _thread
import uasyncio
import uasyncio.core
import uasyncio.event
import uasyncio.funcs
import uasyncio.lock
import uasyncio.stream
from machine import Pin, ADC, PWM, I2C, SPI, UART, Timer, RTC, WDT
from micropython import const, opt_level, alloc_emergency_exception_buf
from machine import mem8, mem16, mem32  # Direct memory access
from array import array
from collections import namedtuple, OrderedDict, deque

filesystem = {

}

python3 = 0
txtedit = 0

# System

while True:
    cmdlet = input("BaushaOS> ").split()

    if cmdlet[0] == "echo":
        print(" ".join(cmdlet[1:]))

    elif cmdlet[0] == "python":
        print("Welcome to Python! Use /exit to exit, or execute anything here!")
        python3 = 1
        while python3 == 1:
            code = input("py> ")
            if code == "//exit":
                python3 = 0
            else:
                exec(code)

    elif cmdlet[0] == "dir":
        if cmdlet[1] == "-create":
            filesystem[cmdlet[2]] = []
        elif cmdlet[1] == "-delete":
            del filesystem[cmdlet[2]]
        elif cmdlet[1] == "-show":
            print("\n".join(filesystem[cmdlet[2]]))
        
    elif cmdlet[0] == "txtedit" and 0 <= 1 < len(cmdlet):
        print("Welcome to Text Editor! Use /exit to exit, or type anything here!")
        txtedit = 1
        while txtedit == 1:
            newline = input("tx> ")
            if newline == "/exit":
                txtedit = 0
            else:
                filesystem[cmdlet[1]].append(newline)

    elif cmdlet[0] == "poweroff":
        raise SystemExit