import sys
from copy import deepcopy
from math import prod

def part_1(ms, rs) :
    ms = deepcopy(ms)
    for _ in range(rs) :
        for m in ms :
            for i in m['items'] :
                m['total'] += 1
                w = m['op'](i) // 3
                t, mt, mf = m['test']
                n = ms[mt if w % t == 0 else mf]
                n['items'].append(w)
            m['items'] = []
    t2 = sorted(m['total'] for m in ms)[-2:]
    return t2[0] * t2[1]

def part_2(ms, rs) :
    ps = prod(m['test'][0] for m in ms)
    ms = deepcopy(ms)
    for _ in range(rs) :
        for m in ms :
            for i in m['items'] :
                m['total'] += 1
                w = m['op'](i)
                t, mt, mf = m['test']
                n = ms[mt if w % t == 0 else mf]
                n['items'].append(w % ps)
            m['items'] = []
    t2 = sorted(m['total'] for m in ms)[-2:]
    return t2[0] * t2[1]

puzzle = '\n'.join(sys.stdin.read().strip().splitlines())

monkeys = []
for m in [b.splitlines() for b in puzzle.split('\n\n')] :
    m = iter(m)
    next(m)
    monkeys.append({
        'items' : [int(i) for i in next(m).split(': ')[1].split(', ')],
        'op' : eval('lambda old : ' + next(m).split(' = ')[1]),
        'test' : (
            int(next(m).split('by ')[1]),
            int(next(m).split('monkey ')[1]),
            int(next(m).split('monkey ')[1])
        ),
        'total' : 0
    })

print('part 1:', part_1(monkeys, 20))
print('part 2:', part_2(monkeys, 10000))
