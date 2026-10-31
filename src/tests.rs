use crate::*;

#[test]
#[should_panic]
fn hex_conv() {
    assert_eq!("0", u8_to_hex(0));
    assert_eq!("1f", u8_to_hex(31));
    assert_eq!("9b", u8_to_hex(155));
    assert_eq!("5f", u8_to_hex(095));
    assert_eq!("8c", u8_to_hex(140));
    assert_eq!("84", u8_to_hex(132));
    assert_eq!("80", u8_to_hex(128));
    assert_eq!("f9", u8_to_hex(249));
    assert_eq!("d9", u8_to_hex(217));
    assert_eq!("ff", u8_to_hex(255));
    assert_eq!("1", u8_to_hex(1));
    assert_eq!("5", u8_to_hex(5));
    assert_eq!("7", u8_to_hex(7));
}

#[test]
fn hex_conv2() {
    assert_eq!("00", u8_to_hex(0));
    assert_eq!("1f", u8_to_hex(31));
    assert_eq!("ff", u8_to_hex(255));
    assert_eq!("01", u8_to_hex(1));
}

#[test]
fn socks5test() {
    socks::Socks5Stream::connect(
        "127.0.0.1:9050",
        "duckduckgogg42xjoc72x3sjasowoarfbgcmvfimaftt6twagswzczad.onion:80",
    )
    .unwrap();
}

#[test]
fn msg_prcss_test() {
    use message::*;
    let chinese_salt = "COMMAND_THAT_I_MADE_UP gotospace :i am a fucking astronaut\r\n";
    let mut param_array = [""; 14]; param_array[0] = "gotospace";
    assert_eq!((None, "COMMAND_THAT_I_MADE_UP", Some(param_array), Some(":i am a fucking astronaut")), whatitmean(chinese_salt).expect("Returned error from an acceptable state"));
}