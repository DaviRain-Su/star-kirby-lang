/// TODO have bug

static TRACE_LEVEL: global::Global<usize> = global::Global::INIT;

fn ident_level() -> String {
    let temp_vec = vec!['\t' as u8; *TRACE_LEVEL.lock().unwrap()];
    String::from_utf8(temp_vec).unwrap()
}

fn trace_print(fs: String) {
    println!("TRACE_LEVEL = {}", *TRACE_LEVEL.lock().unwrap());
    println!("{}{}", ident_level(), fs);
}

fn inc_ident() {
    *TRACE_LEVEL.lock_mut().unwrap() += 1;
}

fn dec_ident() {
    *TRACE_LEVEL.lock_mut().unwrap() -= 1;
}

pub fn trace(msg: String) -> String {
    inc_ident();
    trace_print("BEGIN ".to_owned() + &msg);
    msg
}

pub fn un_trace(msg: String) {
    trace_print("END ".to_owned() + &msg);
    dec_ident();
}
