use std::collections::HashMap;
use crate::debugger_command::DebuggerCommand;
use crate::inferior::Inferior;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use crate::inferior::Status;
use crate::dwarf_data::{DwarfData, Error as DwarfError};
use rustyline::history::FileHistory;

pub struct Debugger {
    target: String,
    history_path: String,
    readline: Editor<(), FileHistory>,
    inferior: Option<Inferior>,
    debug_data: DwarfData,
    break_list: HashMap<usize, u8>,
}

impl Debugger {
    /// Initializes the debugger.
    pub fn new(target: &str) -> Debugger {
        // initialize the DwarfData
        let debug_data = match DwarfData::from_file(target) {
            Ok(val) => val,
            Err(DwarfError::ErrorOpeningFile) => {
                println!("Could not open file {}", target);
                std::process::exit(1);
            }
            Err(DwarfError::DwarfFormatError(err)) => {
                println!("Could not debugging symbols from {}:{:?}", target, err);
                std::process::exit(1);
            }
        };

        debug_data.print();

        let history_path = format!("{}/.deet_history", std::env::var("HOME").unwrap());
        let mut readline = Editor::<(), FileHistory>::new().expect("Create fail");
        // Attempt to load history from ~/.deet_history if it exists
        let _ = readline.readline(&history_path);

        Debugger {
            target: target.to_string(),
            history_path,
            readline,
            inferior: None,
            debug_data: debug_data,
            break_list: HashMap::new(),
        }
    }

    pub fn run(&mut self) {
        loop {
            match self.get_next_command() {
                DebuggerCommand::Run(args) => {
                    if self.inferior.is_some() {
                        println!("Child is already processing!");
                        let child = self.inferior.as_mut().unwrap();
                        println!("Killing running inferior (pid {})", child.pid());
                        child.kill();
                        self.inferior = None;
                    }
                    if let Some(inferior) = Inferior::new(&self.target, &args, &mut self.break_list) {
                        // Create the inferior
                        self.inferior = Some(inferior);
                        
                        // You may use self.inferior.as_mut().unwrap() to get a mutable reference
                        // to the Inferior object
                        match self.inferior.as_mut().unwrap().continue_run(&self.break_list).ok().unwrap() {
                            Status::Exited(exit_code) => println!("Child exit (status {})", exit_code),
                            Status::Signaled(signal)  => println!("Child exit due to {}", signal),
                            Status::Stopped(signal, rip) => {
                                match self.debug_data.get_line_from_addr(rip) {
                                    Some(val) => println!("Child stopped (signal {}) at {}", signal, val),                   
                                    None      => println!("Child stopped (signal {}) at {:#?}", signal, rip),
                                } 
                            }
                        } 
                    } else {
                        println!("Error starting subprocess");
                    }
                }
                DebuggerCommand::Quit => {
                    if self.inferior.is_some() {
                        let child = self.inferior.as_mut().unwrap();
                        println!("Killing running inferior (pid {})", child.pid());
                        child.kill();
                        self.inferior = None;
                    }
                    return;
                }
                DebuggerCommand::Cont => {
                    if self.inferior.is_none() {
                        println!("No child is processing!");
                        continue;
                    }
                    match self.inferior.as_mut().unwrap().continue_run(&self.break_list).ok().unwrap() {
                            Status::Exited(exit_code) => {
                                println!("Child exit (status {})", exit_code);
                                self.inferior = None;
                            }
                            Status::Signaled(signal)  => {
                                println!("Child exit due to {}", signal);
                                self.inferior = None;
                            }
                            Status::Stopped(signal, rip) => {
                                match self.debug_data.get_line_from_addr(rip) {
                                    Some(val) => println!("Child stopped (signal {}) at {}", signal, val),
                                    None      => println!("Child stopped (signal {}) at {:#?}", signal, rip),
                                } 
                            }
                    } 
                }
                DebuggerCommand::Back => {
                    if self.inferior.is_none() {
                        println!("No child is processing!");
                        continue;
                    }
                    self.inferior.as_ref().unwrap().print_backtrace(&self.debug_data).expect("Back trace fail!");
                }
                DebuggerCommand::Break(args) => {
                    for string in args {
                        let addr = self.parse_address(&string).unwrap();
                        if self.inferior.is_some() {
                            if let Ok(inst) = self.inferior.as_mut().unwrap().write_byte(addr, 0xcc) {
                                println!("Set break point {} at {:#x}", self.break_list.len(), addr);
                                self.break_list.insert(addr, inst);
                            } else {
                                println!("Invalid breakpoint at {:#x}", addr);
                            }
                        } else {
                            println!("Set break point {} at {:#x}", self.break_list.len(), addr);
                            self.break_list.insert(addr, 0);
                        }
                    }
                }
            }
        }
    }

    fn parse_address(&self, addr: &str) -> Option<usize> {
        if addr.starts_with('*') {
            let addr_without_0x = if addr[1..].to_lowercase().starts_with("0x") {
                &addr[3..]
            } else {
                &addr[1..]
            };
            // println!("{} {}", addr, addr_without_0x);
            usize::from_str_radix(addr_without_0x, 16).ok()
        } else {
            match addr.to_string().parse() {
                Ok(val) => self.debug_data.get_addr_for_line(None, val),
                Err(_)  => self.debug_data.get_addr_for_function(None, addr),
            }
        }
    }

    /// This function prompts the user to enter a command, and continues re-prompting until the user
    /// enters a valid command. It uses DebuggerCommand::from_tokens to do the command parsing.
    ///
    /// You don't need to read, understand, or modify this function.
    fn get_next_command(&mut self) -> DebuggerCommand {
        loop {
            // Print prompt and get next line of user input
            match self.readline.readline("(deet) ") {
                Err(ReadlineError::Interrupted) => {
                    // User pressed ctrl+c. We're going to ignore it
                    println!("Type \"quit\" to exit");
                }
                Err(ReadlineError::Eof) => {
                    // User pressed ctrl+d, which is the equivalent of "quit" for our purposes
                    return DebuggerCommand::Quit;
                }
                Err(err) => {
                    panic!("Unexpected I/O error: {:?}", err);
                }
                Ok(line) => {
                    if line.trim().len() == 0 {
                        continue;
                    }
                    self.readline.add_history_entry(line.as_str());
                    if let Err(err) = self.readline.save_history(&self.history_path) {
                        println!(
                            "Warning: failed to save history file at {}: {}",
                            self.history_path, err
                        );
                    }
                    let tokens: Vec<&str> = line.split_whitespace().collect();
                    if let Some(cmd) = DebuggerCommand::from_tokens(&tokens) {
                        return cmd;
                    } else {
                        println!("Unrecognized command.");
                    }
                }
            }
        }
    }
}
