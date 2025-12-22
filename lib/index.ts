import { add, addComplex, start } from '@bwin-ui/engine';

const result = add(5, 10);
console.log(`5 + 10 = ${result}`);

const complex1 = { real: 2, imag: 3 };
const complex2 = { real: 4, imag: 5 };
const complexResult = addComplex(complex1, complex2);
console.log(`(${complex1.real} + ${complex1.imag}i) + (${complex2.real} + ${complex2.imag}i) = (${complexResult.real} + ${complexResult.imag}i)`);

start();