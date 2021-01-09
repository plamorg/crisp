"""Mocks midi calls"""
import random
import time
import midi

notes = [60, 62, 64, 67, 69, 72]

try:
    while True:
        note = random.choice(notes)
        midi.play_note(note, 'on')
        time.sleep(1)
        midi.play_note(note, 'off')
        time.sleep(0.05)
except KeyboardInterrupt:
    pass
