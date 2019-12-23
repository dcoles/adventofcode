use intcode::emulator::{Program, IntcodeEmulator, Word, Exception};
use std::collections::{VecDeque, HashMap};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let program = Program::from_file("input.txt").expect("Failed to read input");

    // Part 1
    let mut network = Network::new();
    for addr in 0..50 {
        let network_queue = Rc::clone(&network.queue);
        network.add_computer(Computer::new(addr, &program, network_queue));
    }

    println!("Part 1");
    println!("======");
    network.run();
}

struct Computer {
    address: Word,
    cpu: IntcodeEmulator,
    input_queue: Rc<RefCell<VecDeque<Word>>>,
}

impl Computer {
    fn new(address: Word, program: &Program, network_queue: Rc<RefCell<VecDeque<Packet>>>) -> Self {
        let input_queue = Rc::new(RefCell::new(VecDeque::new()));
        input_queue.borrow_mut().push_back(address);

        let input_queue_ = Rc::clone(&input_queue);
        let input_handler = Box::new(move || {
            if let Some(input) = input_queue_.borrow_mut().pop_front() {
                println!("{:02X}: READ {}", address, input);
                Ok(input)
            } else {
                println!("{:02X}: ENOINPUT (-1)", address);
                Ok(-1)
            }
        });

        let mut output_buffer = Vec::new();
        let output_handler = Box::new(move |word| {
            println!("{:02X}: WRITE {}", address, word);
            output_buffer.push(word);

            if output_buffer.len() == 3 {
                let packet = Packet::new(output_buffer[0], &output_buffer[1..]);
                network_queue.borrow_mut().push_back(packet);
                output_buffer.clear();
            }

            Ok(())
        });

        let mut cpu = IntcodeEmulator::new(input_handler, output_handler);
        cpu.load_program(&program);
        Computer { address, cpu, input_queue }
    }

    fn receive_packet(&mut self, packet: Packet) {
        for input in packet.message.into_iter() {
            self.queue_input(input);
        }
    }

    fn queue_input(&mut self, input: Word) {
        self.input_queue.borrow_mut().push_back(input);
    }

    fn step(&mut self) {
        match self.cpu.step() {
            Err(Exception::Halt) | Ok(_) => (),
            Err(exception) => {
                eprintln!("{:02X} CPU PANIC!!!", self.address);
                self.cpu.dump_registers();
                self.cpu.print_disassembled();
                self.cpu.dump_memory();
                panic!("Unhandled exception on {}: {}", self.address, exception)
            },
        }
    }
}

struct Network {
    computers: HashMap<Word, Computer>,
    queue: Rc<RefCell<VecDeque<Packet>>>,
}

impl Network {
    fn new() -> Self {
        Network { computers: HashMap::new(), queue: Rc::new(RefCell::new(VecDeque::new())) }
    }

    fn add_computer(&mut self, computer: Computer) {
        assert!(!self.computers.contains_key(&computer.address));
        self.computers.insert(computer.address, computer);
    }

    fn run(&mut self) {
        loop {
            while let Some(packet) = self.queue.borrow_mut().pop_front() {
                println!("Routing packet to {:02X} (payload: {:?})", packet.address, packet.message);
                if packet.address == 0xFF {
                    return;
                }
                if let Some(computer) = self.computers.get_mut(&packet.address) {
                    computer.receive_packet(packet);
                }
            }

            for (_, computer) in &mut self.computers {
                computer.step()
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Packet {
    address: Word,
    message: Vec<Word>
}

impl Packet {
    fn new(address: Word, message: &[Word]) -> Self {
        Packet { address, message: message.to_vec() }
    }
}
