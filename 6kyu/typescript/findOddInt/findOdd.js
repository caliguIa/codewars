const findIntThatAppearsOnce = (xs) => {
  if (xs.length === 1) return xs[0];

  // return xs.reduce((acc, num) => acc ^ num, 0);

  const occurrenceMap = new Map();

  xs.forEach((n) => {
    if (occurrenceMap.has(n)) {
      occurrenceMap.set(n, occurrenceMap.get(n) + 1);
    } else {
      occurrenceMap.set(n, 1);
    }
  });

  for (const [key, value] of occurrenceMap.entries()) {
    if (value % 2 !== 0) return key;
  }
};

console.log(findIntThatAppearsOnce([1, 1, 2, -2, 5, 2, 4, 4, -1, -2, 5])); // -1
