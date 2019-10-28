// attempt to sort `input` using comparison function `lt`.
// If `lt` is undefined for any inputs, exit immediately by returning those inputs.
// If `lt` is defined for all inputs, finish sorting the array and return nothing.
function try_sort(input, lt) {
  const xs = [...input];
  let v = undefined;
  if (v = try_heapify(xs, lt)) {
    return v;
  }
  for (let i = 0; i < xs.length; i++) {
    const end = xs.length - i - 1;
    swap(xs, 0, end); // move the max value to the correct spot
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
