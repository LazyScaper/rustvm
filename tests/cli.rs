use std::io::Write;
use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

#[test]
fn image_reads_input_and_halts_successfully() {
    let path = std::env::temp_dir().join(format!("rustvm-input-{}.obj", std::process::id()));
    // GETC, OUT, HALT at x3000.
    std::fs::write(&path, [0x30, 0x00, 0xf0, 0x20, 0xf0, 0x21, 0xf0, 0x25]).unwrap();
    let mut child = Command::new(env!("CARGO_BIN_EXE_RustVm"))
        .arg(&path)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn().unwrap();
    child.stdin.take().unwrap().write_all(b"K").unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    while child.try_wait().unwrap().is_none() {
        if Instant::now() > deadline {
            child.kill().unwrap();
            child.wait().unwrap();
            std::fs::remove_file(&path).unwrap();
            panic!("VM did not consume input and halt within five seconds");
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let output = child.wait_with_output().unwrap();
    std::fs::remove_file(&path).unwrap();
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stderr));
    assert!(String::from_utf8_lossy(&output.stdout).starts_with('K'));
}

#[test]
fn bundled_game_draws_board_and_accepts_a_move() {
    let mut child = Command::new(env!("CARGO_BIN_EXE_RustVm"))
        .arg(concat!(env!("CARGO_MANIFEST_DIR"), "/2048.obj"))
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped())
        .spawn().unwrap();
    let mut stdin = child.stdin.take().unwrap();
    let stdout = child.stdout.take().unwrap();
    let (sender, receiver) = std::sync::mpsc::channel();
    let reader = std::thread::spawn(move || {
        use std::io::BufRead;
        for line in std::io::BufReader::new(stdout).lines() {
            if sender.send(line.unwrap()).is_err() { break; }
        }
    });
    stdin.write_all(b"n").unwrap();
    let deadline = Instant::now() + Duration::from_secs(5);
    let mut borders = 0;
    let mut output = String::new();
    while Instant::now() < deadline && borders < 4 {
        if let Ok(line) = receiver.recv_timeout(Duration::from_millis(100)) {
            if line.contains("+--------------------------+") {
                borders += 1;
                if borders == 2 { stdin.write_all(b"w").unwrap(); }
            }
            output.push_str(&line);
            output.push('\n');
        }
    }
    // This interactive game has no documented quit key. End our child after
    // verifying both the initial board and the board following a move.
    let _ = child.kill();
    child.wait().unwrap();
    drop(stdin);
    reader.join().unwrap();
    assert_eq!(borders, 4, "Game did not render after a move: {output}");
}
