use crate::{Error, Protocol};

struct Case {
    b: Vec<u8>,
    p: Protocol,
}

#[test]
fn test_proto() {
    let cases = [
        Case {
            b: "+OK\r\n".to_owned().into_bytes(),
            p: Protocol::SimpleString("OK".to_owned()),
        },
        Case {
            b: "-PARSEERROR test\r\n".to_owned().into_bytes(),
            p: Protocol::Errors(Error::ParseError("test".to_owned())),
        },
        Case {
            b: ":10086\r\n".to_owned().into_bytes(),
            p: Protocol::Integers(10086),
        },
        Case {
            b: "*3\r\n+OK\r\n-PARSEERROR test\r\n:10086\r\n\r\n"
                .to_owned()
                .into_bytes(),
            p: Protocol::Arrays(vec![
                Protocol::SimpleString("OK".to_owned()),
                Protocol::Errors(Error::ParseError("test".to_owned())),
                Protocol::Integers(10086),
            ]),
        },
    ];

    for case in cases.iter() {
        unsafe {
            assert_eq!(
                String::from_utf8_unchecked(case.p.to_bytes()),
                String::from_utf8_unchecked(case.b.clone())
            );
        }
        assert_eq!(case.p.to_bytes(), case.b);

        let p = Protocol::from(case.b.clone());
        assert_eq!(p, case.p);
        assert_eq!(p.to_bytes(), case.b);
    }
}
