import machine
from machine import Pin, PWM
import time

buzzer = PWM(Pin(15))

def play_tone(frequency, duration):
    buzzer.freq(frequency)
    buzzer.duty_u16(32768)
    time.sleep(duration)
    buzzer.duty_u16(0)
