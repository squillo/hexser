// R10 (PRD HEXSER-0.7 D1): an actor is the one writer, so its door takes `&mut self`. Handling
// a command through a shared reference must not compile.
struct Counter {
  count: u64,
}

impl hexser::application::Actor for Counter {
  type Command = u64;
  type Context<'a> = ();
  type Outcome = u64;

  fn handle(&mut self, command: u64, _context: ()) -> u64 {
    self.count += command;
    self.count
  }
}

fn main() {
  let counter = Counter { count: 0 };
  let shared = &counter;
  let _ = hexser::application::Actor::handle(shared, 1, ());
}
