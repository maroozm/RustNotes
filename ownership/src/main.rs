fn read(y: bool) {}

fn main() {
    read(x);
    let x = true;
    // this will throw errors (and the prpgram is unsafe) because read() fn expects x to be a bool, but x hasn't been defined on line 4. the right way is:
    let x = true;
    read(x);
}

