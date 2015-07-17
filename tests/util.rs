// This Source Code Form is subject to the terms of
// the Mozilla Public License, v. 2.0. If a copy of
// the MPL was not distributed with this file, You
// can obtain one at http://mozilla.org/MPL/2.0/.

use cbor::encoder::{Encoder, EncodeResult};
use cbor::decoder::{Config, Decoder};
use std::io::Cursor;

pub fn identity<F, G>(enc: F, dec: G) -> bool
where F: Fn(Encoder<&mut Cursor<Vec<u8>>>) -> EncodeResult,
      G: Fn(Decoder<&mut Cursor<Vec<u8>>>) -> bool
{
    let mut buffer = Cursor::new(Vec::new());
    match enc(Encoder::new(&mut buffer)) {
        Ok(_)  => (),
        Err(e) => panic!("encoder failure: {:?}", e)
    }
    buffer.set_position(0);
    dec(Decoder::new(Config::default(), &mut buffer))
}
