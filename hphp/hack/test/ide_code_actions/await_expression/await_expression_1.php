<?hh

async function gen_int(): Awaitable<int> {
  return 1;
}


async function repro(): Awaitable<mixed> {
  return (int)gen_int() + 3;
          //     ^ at-caret should be a quickfix (not a refactoring)
}
