import sys
from itertools import islice
from copy import deepcopy

def solve(puzzle, reverse):
    stacks = deepcopy(puzzle[0])
    for n, f, t in puzzle[1]:
        cs = stacks[f][-n:]
        if reverse: cs.reverse()
        stacks[t] += cs
        del stacks[f][-n:]
    return ''.join(s[-1] for s in stacks)

puzzle = '\n'.join(sys.stdin.read().splitlines()).split('\n\n')
moves = [
    tuple(int(n) - i for n, i in zip(islice(m.split(), 1, None, 2), (0, 1, 1)))
    for m in puzzle[1].splitlines()
]
stacks = [
    list(filter(lambda c: c != ' ', reversed(s[:-1])))
    for s in islice(zip(*(list(r) for r in puzzle[0].splitlines())), 1, None, 4)
]
puzzle = (stacks, moves)

print('part 1:', solve(puzzle, True))
print('part 2:', solve(puzzle, False))
