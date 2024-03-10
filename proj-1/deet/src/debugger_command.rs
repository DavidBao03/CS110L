pub enum DebuggerCommand {
    Quit,
    Run(Vec<String>),
    Cont,
    Back,
    Break(Vec<usize>),
}

impl DebuggerCommand {
    fn parse_address(addr: &str) -> Option<usize> {
        let addr_without_0x = if addr.to_lowercase().starts_with("*0x") {
            &addr[3..]
        } else if addr.to_lowercase().starts_with("0x") {
            &addr[2..]
        } else {
            &addr
        };
        // println!("{} {}", addr, addr_without_0x);
        usize::from_str_radix(addr_without_0x, 16).ok()
        // println!("num = {:?}", num);
    }

    pub fn from_tokens(tokens: &Vec<&str>) -> Option<DebuggerCommand> {
        match tokens[0] {
            "q" | "quit" => Some(DebuggerCommand::Quit),
            "r" | "run" => {
                let args = tokens[1..].to_vec();
                Some(DebuggerCommand::Run(
                    args.iter().map(|s| s.to_string()).collect(),
                ))
            }
            "c" | "cont" | "continue" => {
                Some(DebuggerCommand::Cont)
            }
            "bt" | "back" | "backtrace" => {
                Some(DebuggerCommand::Back)
            }
            "b" | "break" => {
                let args = tokens[1..].to_vec();
                // for s in &args {
                //     println!("{}", s);
                // }
                Some(DebuggerCommand::Break(
                    args.iter().map(|s| match DebuggerCommand::parse_address(s) {
                        Some(val) => val,
                        None      => {
                            println!("fail to parse {}", s);
                            0
                        }
                    }
                ).collect(),
                ))
            }
            // Default case:
            _ => None,
        }
    }
}
