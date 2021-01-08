"""Implements wrapper to play notes through mido"""
import mido

# Open port with default port name
port = mido.open_output(name=None, virtual=False)


def play_note(note, action):
    """Play or stop note"""
    if not action in ('on', 'off'):
        raise Exception('Invalid note action received')
    action = 'note_' + action
    msg = mido.Message(action, note=note)
    port.send(msg)
