// attempt to sort `xs` using comparison function `cmp`.
// If `cmp` is undefined for any inputs, exit immediately by returning those inputs.
// If `cmp` is defined for all inputs, finish sorting the array and return nothing.
function try_sort(xs, lt) {
  let v = undefined;
  if (v = try_heapify(xs, lt)) {
    return v;
  }
  for (let i = 0; i < xs.length; i++) {
    const end = xs.length - i - 1;
    swap(xs, 0, end); // move the max value to the correct spot
    console.log(`the ${i}-th value is ${xs[end]}`);
    if (v = try_sift_down(xs, 0, end, lt)) {
      return v;
    }
  }
}

function swap(xs, i, j) {
  const t = xs[i];
  xs[i] = xs[j];
  xs[j] = t;
}

function try_sift_down(xs, i, length, lt) {
  const left_idx = 2 * i + 1;
  const right_idx = 2 * i + 2;

  let best_idx = i;
  if (left_idx < length) {
    const cmp = lt(xs[best_idx], xs[left_idx]);
    if (cmp === undefined) {
      return [xs[best_idx], xs[left_idx]];
    }
    if (cmp === true) {
      best_idx = left_idx;
    }
  }
  if (right_idx < length) {
    const cmp = lt(xs[best_idx], xs[right_idx]);
    if (cmp === undefined) {
      return [xs[best_idx], xs[right_idx]];
    }
    if (cmp === true) {
      best_idx = right_idx;
    }
  }
  if (best_idx !== i) {
    swap(xs, i, best_idx);
    return try_sift_down(xs, best_idx, length, lt);
  }
}

function try_heapify(xs, lt) {
  let v = undefined;
  for (let i = 0; i < xs.length; i++) {
    const end = xs.length - i - 1;
    if (v = try_sift_down(xs, end, xs.length, lt)) {
      return v;
    }
  }
}

const FOO = [3, 1, 4, 5, 9, 2, 6];
const KNOWN_COMPARISONS = [
  [4, ">", 2],
  [4, "<", 6],
  [1, "<", 5],
  [5, "<", 9],
  [3, "<", 9],
  [9, ">", 6],
  [3, "<", 5],
  [4, "<", 5],
  [5, "<", 6],
  [2, "<", 3],
  [2, "<", 5],
  [3, ">", 1],
  [3, ">", 4],
  [1, "<", 2],
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
