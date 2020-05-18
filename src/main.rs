extern crate pcsc;

use pcsc::*;

struct Status { sw1: u8, sw2: u8 }

struct APDU { cla: u8, ins: u8, p1: u8, p2: u8, body: [u8] }

fn select(body: &[u8]) -> APDU {
    APDU { cla: 0x00, ins: 0xA4, p1: 0x04, p2: 0x00, body }
}


// APDUs
pub const SELECT: &[u8] = b"\x00\xA4\x04\x00";
pub const GET_RESPONSE: &[u8] = b"\xA0\xC0\x00\x00";

// AIDs
pub const MASTERCARD_DEBIT: &[u8] = b"\xA0\x00\x00\x00\x04\x30\x60";
pub const MASTERCARD_CREDIT: &[u8] = b"\xA0\x00\x00\x00\x04\x10\x10";

fn main() {
    let card = connect();
    match card {
        Some(c) => send_apdu(c, &[select, b"\x07", MASTERCARD_CREDIT].concat()),
        None => {}
    }
}


fn transmit(card: Card, cmd: &APDU) -> Result<Status, Error> {
    let mut resp_buffer: [u8; 2] = [0; 2];
    match card.transmit(data, &mut resp_buffer) {
        Ok(resp) => Ok(Status { sw1: resp[0], sw2: resp[1] }),
        Err(err) => {
            eprintln!("Failed to transmit APDU command to card: {}", err);
            Err(err)
        }
    }
}

fn send_apdu(card: Card, apdu: &[u8]) {
// Send an APDU command.

    println!("Sending APDU: {:02X?}", apdu);
    let response = transmit(card, apdu);

    println!("Status Code: {:02X?}", status);

// TODO: Get the response length from status code and set the resp_buf to it's size

    let mut resp_buf = [0; MAX_BUFFER_SIZE];
    let response = match card.transmit(&[GET_RESPONSE, b"\x52"].concat(), &mut resp_buf) {
        Ok(resp) => resp,
        Err(err) => {
            eprintln!("Failed to transmit APDU command to card: {}", err);
            std::process::exit(1);
        }
    };
    println!("Response: {:02X?}", response)
}

fn connect() -> Option<Card> {
    let ctx = match Context::establish(Scope::User) {
        Ok(ctx) => ctx,
        Err(err) => {
            eprintln!("Failed to establish context: {}", err);
            std::process::exit(1);
        }
    };

// List available readers.
    let mut readers_buf = [0; 2048];
    let mut readers = match ctx.list_readers(&mut readers_buf) {
        Ok(readers) => readers,
        Err(err) => {
            eprintln!("Failed to list readers: {}", err);
            std::process::exit(1);
        }
    };

// Use the first reader.
    let reader = match readers.next() {
        Some(reader) => reader,
        None => {
            println!("No readers are connected.");
            return None;
        }
    };
    println!("Using reader: {:?}", reader);

// Connect to the card and return it.
    match ctx.connect(reader, ShareMode::Shared, Protocols::ANY) {
        Ok(card) => Some(card),
        Err(Error::NoSmartcard) => {
            println!("A smartcard is not present in the reader.");
            return None;
        }
        Err(err) => {
            eprintln!("Failed to connect to card: {}", err);
            std::process::exit(1);
        }
    }
}
