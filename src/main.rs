mod brutus_args;
mod combinator;
use brutus_args::BrutusArgs;
use combinator::Combinator;
use std::env;
use std::time::Instant;

#[derive(Debug, Clone)]
struct BrutusContext {
    args: BrutusArgs,
    combinator: Combinator,
    output: String,
}

impl BrutusContext {
    fn new(args: &mut BrutusArgs) -> BrutusContext {
        return BrutusContext {
            args: args.clone(),
            combinator: Combinator::new(args.length, args.charset.len() as u32),
            output: args.get_template_str(),
        };
    }
    fn capture_state(&mut self) {
        self.output.clear();
        self.output.push_str(&self.args.prefix);
        for i in 0..self.combinator.len() {
            self.output
                .push(self.args.char_at(self.combinator.get(i)).unwrap());
        }
    }
    fn search_for_target(&mut self) -> bool {
        for _ in 0..self.args.max_iterations() {
            self.capture_state();
            if self.output.eq(&self.args.target_str) {
                return true;
            }
            self.combinator.shift(1);
        }
        return false;
    }
    fn as_vec(&mut self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        for _ in 0..self.args.max_iterations() {
            self.capture_state();
            result.push(self.output.to_string());
            self.combinator.shift(1);
        }
        return result;
    }
    fn run(&mut self) {
        if self.args.has_target() {
            let target = self.args.target_str.to_string();
            let result = self.search_for_target();
            println!("found target {} = {}", target, result);
        } else {
            let result = self.as_vec();
            println!("vec result = {:?}", result);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut brutus_args = BrutusArgs::new(args);
    let mut brutus_context = BrutusContext::new(&mut brutus_args);
    println!("context = {:?}", brutus_context);
    let now = Instant::now();
    brutus_context.run();
    println!("barrage took {} ms", now.elapsed().as_millis());
}
