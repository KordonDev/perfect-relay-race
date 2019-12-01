import { print_run, Run, print_person, Person, calculate } from 'perfect-relay-race-wasm';

const run1Adam = Run.new(100, 15);
const run2Adam = Run.new(150, 25);
const run3Adam = Run.new(250, 29);
const adam = Person.new("Adam", [ run1Adam, run2Adam, run3Adam]);

const run1Jo = Run.new(100, 13);
const run2Jo = Run.new(150, 20);
const run3Jo = Run.new(250, 27);
const jo = Person.new("Jo", [ run1Jo, run2Jo, run3Jo]);

const run1Bert = Run.new(100, 17);
const run2Bert = Run.new(150, 24);
const run3Bert = Run.new(250, 27);
const bert = Person.new("Bert", [ run1Bert, run2Bert, run3Bert]);

const run1Carl = Run.new(100, 13);
const run2Carl = Run.new(150, 25);
const run3Carl = Run.new(250, 30);
const carl = Person.new("Carl", [ run1Carl, run2Carl, run3Carl]);


const result = calculate([ adam, jo, bert, carl ]);
console.log(result);
