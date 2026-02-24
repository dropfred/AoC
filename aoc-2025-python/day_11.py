import sys

def paths(net, start, end):
    memo = {end: 1}
    def paths(n):
        if n not in memo:
            memo[n] = sum(paths(n) for n in net[n]) if n in net else 0
        return memo[n]

    return paths(start)

def part_1(puzzle):
    return paths(puzzle, 'you', 'out')

def part_2(puzzle):
    return paths(puzzle, 'svr', 'fft') * fft_dac * paths(puzzle, 'dac', 'out') \
           if (fft_dac := paths(puzzle, 'fft', 'dac')) != 0 else \
           paths(puzzle, 'svr', 'dac') * paths(puzzle, 'dac', 'fft') * paths(puzzle, 'fft', 'out')

puzzle = {n: ns.split() for n, ns in (r.strip().split(': ') for r in sys.stdin.read().strip().splitlines())}

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))
