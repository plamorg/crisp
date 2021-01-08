"""Interfaces with arduino serial output"""
import serial
import midi

PORT = "/dev/tty.usbmodem14201"
BAUD_RATE = 9600
SER = serial.Serial(PORT, BAUD_RATE)


def get_serial_input():
    """Returns serial input delimited by space"""
    return SER.readline().decode('utf-8').strip().split(' ')


def main():
    """Repeatedly request for input and call play note function"""
    while True:
        serial_input = get_serial_input()
        # print(serial_input)
        note = serial_input[0]
        action = serial_input[1]
        midi.play_note(int(note), action)

if __name__ == "__main__":
    main()
