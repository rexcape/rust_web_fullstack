# rust_web_fullstack

[teacher.rs](https://github.com/rexcape/rust_web_fullstack/blob/main/webservice/src/dbaccess/teacher.rs)

cargo check

```plain
    Checking webservice v0.1.0 (E:\Projects\rust\rust_web_fullstack\webservice)
error[E0308]: mismatched types
  --> webservice\src\bin\..\dbaccess\teacher.rs:14:19
   |
14 |             name: r.name.clone(),
   |                   ^^^^^^^^^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
   |
   = note: expected struct `std::string::String`
                found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
  --> webservice\src\bin\..\dbaccess\teacher.rs:15:26
   |
15 |             picture_url: r.picture_url.clone(),
   |                          ^^^^^^^^^^^^^^^^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
   |
   = note: expected struct `std::string::String`
                found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
  --> webservice\src\bin\..\dbaccess\teacher.rs:16:22
   |
16 |             profile: r.profile.clone(),
   |                      ^^^^^^^^^^^^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
   |
   = note: expected struct `std::string::String`
                found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
  --> webservice\src\bin\..\dbaccess\teacher.rs:35:15
   |
35 |         name: r.name,
   |               ^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
   |
   = note: expected struct `std::string::String`
                found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
  --> webservice\src\bin\..\dbaccess\teacher.rs:36:22
   |
36 |         picture_url: r.picture_url,
   |                      ^^^^^^^^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
   |
   = note: expected struct `std::string::String`
                found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
  --> webservice\src\bin\..\dbaccess\teacher.rs:37:18
   |
37 |         profile: r.profile,
   |                  ^^^^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
   |
   = note: expected struct `std::string::String`
                found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
  --> webservice\src\bin\..\dbaccess\teacher.rs:60:15
   |
60 |         name: row.name,
   |               ^^^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
   |
   = note: expected struct `std::string::String`
                found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
  --> webservice\src\bin\..\dbaccess\teacher.rs:61:22
   |
61 |         picture_url: row.picture_url,
   |                      ^^^^^^^^^^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
   |
   = note: expected struct `std::string::String`
                found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
  --> webservice\src\bin\..\dbaccess\teacher.rs:62:18
   |
62 |         profile: row.profile,
   |                  ^^^^^^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
   |
   = note: expected struct `std::string::String`
                found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
  --> webservice\src\bin\..\dbaccess\teacher.rs:84:13
   |
84 |             row.name
   |             ^^^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
   |
   = note: expected struct `std::string::String`
                found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
  --> webservice\src\bin\..\dbaccess\teacher.rs:89:13
   |
89 |             row.picture_url
   |             ^^^^^^^^^^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
   |
   = note: expected struct `std::string::String`
                found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
  --> webservice\src\bin\..\dbaccess\teacher.rs:94:13
   |
94 |             row.profile
   |             ^^^^^^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
   |
   = note: expected struct `std::string::String`
                found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
   --> webservice\src\bin\..\dbaccess\teacher.rs:110:15
110 |         name: r.name,
    |               ^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
    |
    = note: expected struct `std::string::String`
                 found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
   --> webservice\src\bin\..\dbaccess\teacher.rs:111:22
    |
111 |         picture_url: r.picture_url,
    |                      ^^^^^^^^^^^^^ expected struct `std::string::String`, found enum `std::option::Option`
    |
    = note: expected struct `std::string::String`
                 found enum `std::option::Option<std::string::String>`

error[E0308]: mismatched types
   --> webservice\src\bin\..\dbaccess\teacher.rs:112:18
    |
112 |         profile: r.profile,
    |
    = note: expected struct `std::string::String`
                 found enum `std::option::Option<std::string::String>`

For more information about this error, try `rustc --explain E0308`.
error: could not compile `webservice` due to 15 previous errors
```
