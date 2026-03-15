# piday2026
Calculates an arbitrary number of digits of pi using the Chudnovsky algorithm. Built entirely on March 14th, 2026, AKA 3/14. Made in Rust, using the `rug` crate for large integers and floating point precision.

## Usage
Works as a shell program. To use (assuming a Unix or Unix-like OS), make sure you first have Git, Rust, and `cargo` installed. Then, simply:

**1\. Git clone into the repo.**
```bash
git clone https://github.com/pythonkid90/piday2026
cd piday2026
```
**2\. Build the cargo package.**
```bash
cargo build --release
```
**3\. Copy the program to /usr/local/bin (optional).**
```bash
cp target/release/piday2026 /usr/local/bin/pi
```

If you copy the program to /usr/local/bin, you can always use the script as shown below:
```bash
pi <digits of pi you want to calculate>
```
You can still use the script when you are in the `piday2026` directory using:
```bash
target/release/piday2026 <digits to calculate>
```
or:
```bash
cargo run -- <digits to calulate>
```

## Functionality
Digits has to be an unsigned 32-bit integer, and leading/trailing whitespace will be trimmed. The program will then begin calculation, giving the number of digits currently calculated as it goes on. Upon completion, it will print all digits and ask you if you want it saved to a file, which will be saved to your current working directory if you choose to save.