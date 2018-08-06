#![feature(test)]
#![feature(const_fn)]

extern crate gailibrary;

use gailibrary::langtonsant;

#[test]
fn langtons_ant() {
    langtonsant::langtons_ant();
}
