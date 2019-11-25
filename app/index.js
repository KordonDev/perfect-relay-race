import { print_run, Run, print_person, Person, calculate } from 'perfect-relay-race-wasm';

const run = Run.new(100, 15);
const run2 = Run.new(150, 25);
const run3 = Run.new(250, 25);
const run4 = Run.new(100, 25);

console.log(run);

print_run(run);

const person1 = Person.new("Bob", [run, run2]);
const person2 = Person.new("Georg", [run3, run4]);

print_person(person1);

const result = calculate([person1, person2]);
console.log(result);
