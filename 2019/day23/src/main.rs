use intcode::emulator::{Program, IntcodeEmulator, Context, Word};
use std::collections::{VecDeque, HashMap};
use std::cell::{RefCell, Cell};
use std::rc::Rc;

const ADDR_ZERO: usize = 0x00;
const ADDR_NAT: usize = 0xFF;
const N_COMPUTERS: usize = 50;

fn main() {
    let program = Program::from_file("input.txt").expect("Failed to read input");

    let mut network = Network::new();
    for addr in 0..N_COMPUTERS {
        let network_queue = Rc::clone(&network.queue);
        network.add_computer(Computer::new(addr, &program, network_queue));
    }

    network.run();

    let first_nat_packet = network.first_nat_packet.expect("No first");
    println!("Part 1: `Y` of first packet sent to address 255: {}", first_nat_packet.payload[1]);
    let nat = network.nat.expect("No NAT packet");
    println!("Part 2: First `Y` released by NAT twice in a row: {}", nat.payload[1]);

}

struct Computer {
    address: usize,
    cpu: IntcodeEmulator,
    input_queue: Rc<RefCell<VecDeque<Word>>>,
    idle: Rc<Cell<bool>>
}

impl Computer {
    fn new(address: usize, program: &Program, network_queue: Rc<RefCell<VecDeque<Packet>>>) -> Self {
        let input_queue = Rc::new(RefCell::new(VecDeque::new()));
        input_queue.borrow_mut().push_back(address as Word);
        let idle = Rc::new(Cell::new(false));

        let idle_ = Rc::clone(&idle);
        let input_queue_ = Rc::clone(&input_queue);
        let mut last_input = -1;
        let input_handler = Box::new(move |_: &mut Context| {
            if let Some(input) = input_queue_.borrow_mut().pop_front() {
                println!("@{}: READ {}", address, input);
                last_input = input;

                Ok(input)
            } else {
                println!("@{}: ENOINPUT (-1)", address);
                if last_input == -1 {
                    idle_.set(true);
                }
                last_input = -1;

                Ok(-1)
            }
        });

        let mut output_buffer = Vec::new();
        let output_handler = Box::new(move |_: &mut Context, word| {
            println!("@{}: WRITE {}", address, word);
            output_buffer.push(word);

            if output_buffer.len() == 3 {
                let addr = output_buffer[0] as usize;
                let packet = Packet::new(address, addr, &output_buffer[1..]);
                network_queue.borrow_mut().push_back(packet);
                output_buffer.clear();
            }

            Ok(())
        });

        let mut cpu = IntcodeEmulator::new(input_handler, output_handler);
        cpu.load_program(&program);
        Computer { address, cpu, input_queue, idle }
    }

    fn receive_packet(&mut self, packet: Packet) {
        for input in packet.payload.into_iter() {
            self.queue_input(input);
        }
    }

    fn queue_input(&mut self, input: Word) {
        self.idle.set(false);
        self.input_queue.borrow_mut().push_back(input);
    }

    fn is_idle(&self) -> bool {
        self.idle.get()
    }

    fn step(&mut self) {
        // This is very inefficient, should really run until I/O
        match self.cpu.step() {
            Ok(_) => (),
            Err(exception) => {
                eprintln!("@{} CPU PANIC!!!", self.address);
                self.cpu.dump_registers();
                self.cpu.print_disassembled();
                self.cpu.dump_memory();
                panic!("Unhandled exception on {}: {}", self.address, exception)
            },
        }
    }
}

struct Network {
    computers: HashMap<usize, Computer>,
    queue: Rc<RefCell<VecDeque<Packet>>>,
    nat: Option<Packet>,
    first_nat_packet: Option<Packet>,
}

impl Network {
    fn new() -> Self {
        Network { computers: HashMap::new(), queue: Rc::new(RefCell::new(VecDeque::new())), nat: None, first_nat_packet: None }
    }

    fn add_computer(&mut self, computer: Computer) {
        assert!(!self.computers.contains_key(&computer.address));
        self.computers.insert(computer.address, computer);
    }

    fn run(&mut self) {
        let mut last_nat_y = -1;
        loop {
            // Route packets
            while let Some(packet) = self.queue.borrow_mut().pop_front() {
                println!("NET: Routing packet from @{} to @{} (payload: {:?})", packet.source, packet.destination, packet.payload);
                if packet.destination == ADDR_NAT {
                    // Handled by the NAT
                    if self.first_nat_packet.is_none() {
                        self.first_nat_packet = Some(packet.clone());
                    }
                    self.nat = Some(packet);
                } else {
                    // Send to computer
                    if let Some(computer) = self.computers.get_mut(&packet.destination) {
                        computer.receive_packet(packet);
                    }
                }
            }

            // Step computers
            let mut active_computers: Vec<_> = self.computers.values_mut().filter(|c| !c.is_idle()).collect();
            active_computers.sort_by_key(|c| c.address);
            if active_computers.is_empty() {
                // Handle idle state
                println!("NET: Network idle...");
                if let Some(mut packet) = self.nat.take() {
                    packet.destination = ADDR_ZERO;
                    //println!("NET: Releasing packet to @{} (payload: {:?})", packet.address, packet.payload);
                    if packet.payload[1] == last_nat_y {
                        // Found first Y delivered twice in a row
                        self.nat = Some(packet);  // Requeue for solution
                        return;
                    }
                    last_nat_y = packet.payload[1];

                    if let Some(computer) = self.computers.get_mut(&packet.destination) {
                        computer.receive_packet(packet);
                        assert_eq!(computer.is_idle(), false);
                    }

                    self.nat = None;
                }
            } else {
                // Run active computers
                for computer in active_computers {
                    while !computer.is_idle() {
                        computer.step()
                    }
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Packet {
    source: usize,
    destination: usize,
    payload: Vec<Word>
}

impl Packet {
    fn new(source: usize, destination: usize, payload: &[Word]) -> Self {
        Packet { source, destination, payload: payload.to_vec() }
    }
}
