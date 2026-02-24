import sys
from collections import deque

class Grid:
    def __init__(self,  grid):
        self.__grid = grid
    def size(self):
        return (len(self.__grid[0]), len(self.__grid))
    def neighbors(self, p):
        w, h = self.size()
        return (
            (nr, nc) for (dr, dc) in ((-1, 0), (1, 0), (0, -1), (0, 1))
            if  0 <= (nr := p[0] + dr) < h and 0 <= (nc := p[1] + dc) < w
        )
    def __call__(self, p):
        return self.__grid[p[0]][p[1]]

class Puzzle:
    def __init__(self, grid, enter, exit):
        self.grid = grid
        self.enter = enter
        self.exit = exit
    def parse(txt):
        g = [[*r] for r in txt.strip().splitlines()]
        w, h = len(g[0]), len(g)
        r, c = s = next((r, c) for r in range(h) for c in range(w) if g[r][c] == 'S'); g[r][c] = 'a'
        r, c = e = next((r, c) for r in range(h) for c in range(w) if g[r][c] == 'E'); g[r][c] = 'z'
        g = Grid(tuple(tuple(map(lambda c: ord(c) - ord('a'), r)) for r in g))
        return Puzzle(g, s, e)

def solve(grid, start, goal, valid):
    q = deque([(start, 0)])
    vs =  {start}
    while q:
        p, d = q.popleft()
        if goal(p, grid(p)) if callable(goal) else p == goal: return d
        for n in (n for n in grid.neighbors(p) if n not in vs and valid(grid(p), grid(n))):
            vs.add(n)
            q.append((n, d + 1))
    return None

def part_1(puzzle):
    return solve(puzzle.grid, puzzle.enter, puzzle.exit, lambda ph, nh: nh - ph < 2)

def part_2(puzzle):
    return solve(puzzle.grid, puzzle.exit, lambda _, h: h == 0, lambda ph, nh: ph - nh < 2)

puzzle = Puzzle.parse(sys.stdin.read())

print('part 1:', part_1(puzzle))
print('part 2:', part_2(puzzle))
