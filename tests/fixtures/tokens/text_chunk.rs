use cool_thing::{ContentType, TextChunk, UserData};
use encoding_rs::UTF_8;

test_fixture!("Text chunk token", {
    test("User data", {
        parse_token!("foo", UTF_8, TextChunk, |c: &mut TextChunk| {
            c.set_user_data(42usize);

            assert_eq!(
                *c.user_data().unwrap().downcast_ref::<usize>().unwrap(),
                42usize
            );
        });
    });

    test("Serialization", {
        let src =
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor \
             incididunt ut labore et dolore magna aliqua. Ut enim ad minim veniam, quis nostrud \
             exercitation & ullamco laboris nisi ut aliquip ex ea commodo > consequat.";

        serialization_test!(
            src,
            TextChunk,
            &[
                ("Parsed", Box::new(|_, _| {}), src),
                (
                    "With prepends and appends",
                    Box::new(|c, _| {
                        c.before("<span>", ContentType::Text);
                        c.before("<div>Hey</div>", ContentType::Html);
                        c.before("<foo>", ContentType::Html);
                        c.after("</foo>", ContentType::Html);
                        c.after("<!-- 42 -->", ContentType::Html);
                        c.after("<foo & bar>", ContentType::Text);
                    }),
                    concat!(
                        "&lt;span&gt;<div>Hey</div><foo>",
                        "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod \
                         tempor incididunt ut labore et dolore magna aliqua. Ut enim ad minim \
                         veniam, quis nostrud exercitation & ullamco laboris nisi ut aliquip \
                         ex ea commodo > consequat.",
                        "&lt;foo &amp; bar&gt;<!-- 42 --></foo>"
                    )
                ),
                (
                    "Removed",
                    Box::new(|c, _| {
                        assert!(!c.removed());

                        c.remove();

                        assert!(c.removed());

                        c.before("<before>", ContentType::Html);
                        c.after("<after>", ContentType::Html);
                    }),
                    "<before><after>",
                ),
                (
                    "Replaced",
                    Box::new(|c, _| {
                        c.before("<before>", ContentType::Html);
                        c.after("<after>", ContentType::Html);

                        assert!(!c.removed());

                        c.replace("<div></div>", ContentType::Html);
                        c.replace("<!--42-->", ContentType::Html);

                        assert!(c.removed());
                    }),
                    "<before><div></div><!--42--><after>",
                ),
            ]
        );
    });
});
