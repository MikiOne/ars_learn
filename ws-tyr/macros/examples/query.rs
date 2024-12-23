use try_macros::query;
//TokenStream [
//     Ident {
//         ident: "SELECT",
//         span: #0 bytes(95..101),
//     },
//     Punct {
//         ch: '*',
//         spacing: Alone,
//         span: #0 bytes(102..103),
//     },
//     Ident {
//         ident: "FROM",
//         span: #0 bytes(104..108),
//     },
//     Ident {
//         ident: "users",
//         span: #0 bytes(109..114),
//     },
//     Ident {
//         ident: "u",
//         span: #0 bytes(115..116),
//     },
//     Ident {
//         ident: "JOIN",
//         span: #0 bytes(117..121),
//     },
//     Group {
//         delimiter: Parenthesis,
//         stream: TokenStream [
//             Ident {
//                 ident: "SELECT",
//                 span: #0 bytes(123..129),
//             },
//             Punct {
//                 ch: '*',
//                 spacing: Alone,
//                 span: #0 bytes(130..131),
//             },
//             Ident {
//                 ident: "from",
//                 span: #0 bytes(132..136),
//             },
//             Ident {
//                 ident: "profiles",
//                 span: #0 bytes(137..145),
//             },
//             Ident {
//                 ident: "p",
//                 span: #0 bytes(146..147),
//             },
//         ],
//         span: #0 bytes(122..148),
//     },
//     Ident {
//         ident: "WHERE",
//         span: #0 bytes(149..154),
//     },
//     Ident {
//         ident: "u",
//         span: #0 bytes(155..156),
//     },
//     Punct {
//         ch: '.',
//         spacing: Alone,
//         span: #0 bytes(156..157),
//     },
//     Ident {
//         ident: "id",
//         span: #0 bytes(157..159),
//     },
//     Punct {
//         ch: '=',
//         spacing: Alone,
//         span: #0 bytes(160..161),
//     },
//     Ident {
//         ident: "p",
//         span: #0 bytes(162..163),
//     },
//     Punct {
//         ch: '.',
//         spacing: Alone,
//         span: #0 bytes(163..164),
//     },
//     Ident {
//         ident: "id",
//         span: #0 bytes(164..166),
//     },
//     Ident {
//         ident: "and",
//         span: #0 bytes(167..170),
//     },
//     Ident {
//         ident: "u",
//         span: #0 bytes(171..172),
//     },
//     Punct {
//         ch: '.',
//         spacing: Alone,
//         span: #0 bytes(172..173),
//     },
//     Ident {
//         ident: "age",
//         span: #0 bytes(173..176),
//     },
//     Punct {
//         ch: '>',
//         spacing: Alone,
//         span: #0 bytes(177..178),
//     },
//     Literal {
//         kind: Integer,
//         symbol: "10",
//         suffix: None,
//         span: #0 bytes(179..181),
//     },
// ]
fn main() {
    // query!(select * from user where id > 3);
    query!(SELECT * FROM users u JOIN (SELECT * from profiles p) WHERE u.id = p.id and u.age > 10);
    hello();
}