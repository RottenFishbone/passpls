# passpls

A small program to locally generate an easy to remember password. 

Passwords security follows XKCD's popular scheme of building password entropy from 
a list of easy to remember words. The built-in dictionary has ~57,000 unique words providing ~15.79 bits 
of entropy per word used.

The dictionary was built using SCOWL and later stripped of apostrophes. Attempts were made to only
grab easy to read words.

Random numbers are selected using a cryptographically secure random number generator.

---

## Compilation
After ensuring that rustup is installed and a toolchain is set:
```
git clone https://github.com/RottenFishbone/passpls.git
cd passpls
cargo run --release
```

---

## Optional Features
By default the binary is compiled with clipboard support and terminal styling built-in.
These can be disabled (removing dependencies alongside them) and/or selected specifically 
using the `clipboard` and `style` features during compilation.

---

## Author
Jayden Dumouchel -- jdumouch@ualberta.ca | rottenfishbone@pm.me

### Notes
This is untested on windows but *afaik* it will compile and run correctly.

### Disclaimer
I am not an experienced cryptographer. This was mostly for fun but according to my research
will generate cryptographically secure and strong passwords. As always, a long and completely random 
password generated and stored into a password manager will likely yield stronger passwords.
