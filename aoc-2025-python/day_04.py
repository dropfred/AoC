import sys

def solve(puzzle, repeat):
    w, h = len(puzzle[0]), len(puzzle)
    total = 0

    puzzle = [list(s) for s in puzzle]

    while True:
        rolls = []
        for x in range(w):
            for y in range(h):
                if puzzle[y][x] != '@': continue
                rs = 0
                for dx in (-1, 0, 1):
                    for dy  in (-1, 0, 1):
                        if dx != 0 or dy != 0:
                            nx, ny = x + dx, y + dy
                            if nx >= 0 and nx < w and ny >= 0 and ny < h and puzzle[ny][nx] == '@': rs += 1
                if rs < 4: rolls.append((x, y))
        if len(rolls) == 0: break
        total += len(rolls)
        for x, y in rolls: puzzle[y][x] = '.'
        if not repeat: break

    return total

puzzle = [s.strip() for s in sys.stdin.read().strip().splitlines()]

print('part 1:', solve(puzzle, False))
print('part 2:', solve(puzzle, True))
