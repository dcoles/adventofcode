use intcode::emulator::{Program, IntcodeEmulator, Word, Exception};
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
    receiving: Rc<Cell<bool>>
}

impl Computer {
    fn new(address: usize, program: &Program, network_queue: Rc<RefCell<VecDeque<Packet>>>) -> Self {
        let input_queue = Rc::new(RefCell::new(VecDeque::new()));
        input_queue.borrow_mut().push_back(address as Word);
        let receiving = Rc::new(Cell::new(false));

        let receiving_ = Rc::clone(&receiving);
        let input_queue_ = Rc::clone(&input_queue);
        let input_handler = Box::new(move || {
            receiving_.set(true);
            if let Some(input) = input_queue_.borrow_mut().pop_front() {
                //println!("{:02X}: READ {}", address, input);
                Ok(input)
            } else {
                //println!("{:02X}: ENOINPUT (-1)", address);
                Ok(-1)
            }
        });

        let receiving_ = Rc::clone(&receiving);
        let mut output_buffer = Vec::new();
        let output_handler = Box::new(move |word| {
            receiving_.set(false);
            //println!("{:02X}: WRITE {}", address, word);
            output_buffer.push(word);

            if output_buffer.len() == 3 {
                let addr = output_buffer[0] as usize;
                let packet = Packet::new(addr, &output_buffer[1..]);
                network_queue.borrow_mut().push_back(packet);
                output_buffer.clear();
            }

            Ok(())
        });

        let mut cpu = IntcodeEmulator::new(input_handler, output_handler);
        cpu.load_program(&program);
        Computer { address, cpu, input_queue, receiving }
    }

    fn receive_packet(&mut self, packet: Packet) {
        for input in packet.payload.into_iter() {
            self.queue_input(input);
        }
    }

    fn queue_input(&mut self, input: Word) {
        self.input_queue.borrow_mut().push_back(input);
    }

    fn is_idle(&self) -> bool {
        self.input_queue.borrow().is_empty() && self.receiving.get()
    }

    fn step(&mut self) {
        // This is very inefficient, should really run until I/O
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
                //println!("NET: Routing packet to {:02X} (payload: {:?})", packet.address, packet.payload);
                if packet.address == ADDR_NAT {
                    // Handled by the NAT
                    //println!("NET: Got NAT packet (payload: {:?})", packet.payload);
                    if self.first_nat_packet.is_none() {
                        self.first_nat_packet = Some(packet.clone());
                    }
                    self.nat = Some(packet);
                } else {
                    // Send to computer
                    if let Some(computer) = self.computers.get_mut(&packet.address) {
                        computer.receive_packet(packet);
                    }
                }
            }

            // Step computers
            for (_, computer) in &mut self.computers {
                computer.step()
            }

            // Handle idle state
            if self.computers.values().all(|c| c.is_idle()) {
                //println!("NET: Network idle...");
                if let Some(mut packet) = self.nat.take() {
                    packet.address = ADDR_ZERO;
                    //println!("NET: Releasing packet to {:02X} (payload: {:?})", packet.address, packet.payload);
                    if packet.payload[1] == last_nat_y {
                        // Found first Y delivered twice in a row
                        self.nat = Some(packet);  // Requeue for solution
                        return;
                    }
                    last_nat_y = packet.payload[1];

                    if let Some(computer) = self.computers.get_mut(&packet.address) {
                        computer.receive_packet(packet);
                    }

                    self.nat = None;
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
struct Packet {
    address: usize,
    payload: Vec<Word>
}

impl Packet {
    fn new(address: usize, payload: &[Word]) -> Self {
        Packet { address, payload: payload.to_vec() }
    }
}
