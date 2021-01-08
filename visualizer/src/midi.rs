extern crate midir;

use std::error::Error;
use std::io::stdin;

use midir::{Ignore, MidiInput};

pub fn run_midi() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();

    let mut midi_in = MidiInput::new("midir reading input")?;
    midi_in.ignore(Ignore::None);

    let in_ports = midi_in.ports();
    let in_port = match in_ports.len() {
        0 => return Err("No input port found".into()),
        _ => {
            println!(
                "Input port: {}",
                midi_in.port_name(&in_ports[0]).expect("Port is invalid")
            );
            &in_ports[0]
        }
    };

    println!("\nOpening connection");
    let in_port_name = midi_in.port_name(in_port)?;

    const NOTE_ON: u8 = 0x9;
    const NOTE_OFF: u8 = 0x8;

    let _conn_in = midi_in.connect(
        in_port,
        "midir-read-input",
        move |_, message, _| {
            let status = match message.get(0) {
                Some(e) => e,
                None => return,
            };
            let note = match message.get(1) {
                Some(e) => e,
                None => return,
            };
            if *status == NOTE_ON {
            } else if *status == NOTE_OFF {
            }
        },
        (),
    )?;

    input.clear();
    stdin().read_line(&mut input)?;

    println!("Closing connection");
    Ok(())
}
