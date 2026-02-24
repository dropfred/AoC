import sys
from itertools import chain

class Dir:
    def __init__(self):
        self.subs = {}
        self.files = {}
    def __repr__(self):
        return f'Dir(subs={self.subs}, files={self.files})'
    def walk(self):
        return chain((ss for s in self.subs.values() for ss in s.walk()), (self,))
    def size(self):
        return sum(sum(d.files.values()) for d in self.walk())

def get_fs(puzzle):
    cmds = [s.strip().splitlines() for s in puzzle.split('$')][1:]
    root = Dir()
    cwd = [root]
    for c, *cos in cmds:
        wd = cwd[-1]
        c, *cas = c.split()
        if c == 'cd':
            assert(len(cas) == 1 and len(cos) == 0)
            n = cas[0]
            if n == '/':
                cwd = [root]
            elif n == '..':
                assert(len(cwd) > 1)
                cwd.pop()
            else:
                assert(n in wd.subs)
                cwd.append(wd.subs[n])
        elif c == 'ls':
            assert(len(cas) == 0)
            for sd_n in cos:
                sd, n = sd_n.split()
                if sd == 'dir':
                    if n not in wd.subs: wd.subs[n] = Dir()
                else:
                    wd.files[n] = int(sd)
    return root

def part_1(fs):
    return sum(s for d in fs.walk() if (s := d.size()) <= 100000)

def part_2(fs):
    free = (fs.size() + 30000000) - 70000000
    return min(s for d in fs.walk() if (s := d.size()) >= free)

puzzle = sys.stdin.read().strip()
fs = get_fs(puzzle)

print('part 1:', part_1(fs))
print('part 2:', part_2(fs))
