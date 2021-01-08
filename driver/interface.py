"""Interfaces with arduino serial output"""
import serial
import midi

PORT = ""
BAUD_RATE = 9600
SER = serial.Serial(PORT, BAUD_RATE)


def get_serial_input():
    """Returns serial input delimited by space"""
    return SER.readline().split("\\s+")


def main():
    """Repeatedly request for input and call play note function"""
    try:
        serial_input = get_serial_input()
        note = serial_input[0]
        action = serial_input[1]
        midi.play_note(note, action)
    except KeyboardInterrupt:
        pass


if __name__ == "__main__":
    main()
