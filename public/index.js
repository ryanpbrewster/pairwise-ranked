// attempt to sort `xs` using comparison function `cmp`.
// If `cmp` is undefined for any inputs, exit immediately by returning those inputs.
// If `cmp` is defined for all inputs, finish sorting the array and return nothing.
function try_sort(xs, lt) {
  for (let i = 0; i < xs.length; i++) {
    console.log(`finding the ${i}-th value`);
    let best = i;
    for (let j = i+1; j < xs.length; j++) {
      const cmp = lt(xs[best], xs[j]);
      if (cmp === undefined) {
        return [xs[best], xs[j]];
      }
      if (cmp === true) {
        best = j;
      }
    }
    const tmp = xs[i];
    xs[i] = xs[best];
    xs[best] = tmp;
  }
}

const FOO = [3, 1, 4, 5, 9, 2, 6];
const KNOWN_COMPARISONS = [
  [3, ">", 1],
  [3, "<", 4],
  [4, "<", 5],
  [5, "<", 9],
  [9, ">", 2],
  [9, ">", 6],
  [1, "<", 4],
  [5, ">", 3],
  [5, ">", 2],
  [5, "<", 6],
  [5, ">", 1],
  [4, ">", 2],
];
function BAR(a, b) {
  console.log(`comparing ${a} vs ${b}`);
  for (const elem of KNOWN_COMPARISONS) {
    if (elem[0] === a && elem[2] === b) {
      return elem[1] === "<";
    }
    if (elem[0] === b && elem[2] === a) {
      return elem[1] === ">";
    }
  }
  console.log("no known comparison...");
}

console.log(try_sort(FOO, BAR))
