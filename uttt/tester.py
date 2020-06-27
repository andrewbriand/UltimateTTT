# extremely simple and naive tester for UTI
import time
import fileinput
import sys

def eprint(*args, **kwargs):
    print(*args, file=sys.stderr, **kwargs)

inp = fileinput.input()
assert(next(inp).strip() == 'uti')
print('utiok', flush=True)

while True:
    line = next(inp)
    line = line.strip();

    toks = line.split()
    assert toks[0] == 'pos' and toks[1] == 'moves'
    x = int(toks[2])

    line = next(inp).strip()
    assert line == 'search free'
    time.sleep(3)  # "think" for a bit

    # short sequence of valid moves
    best = None
    if x == -1:
        best = 0
    elif x == 0:
        best = 1
    elif x == 1:
        best = 9
    elif x == 9:
        best = 2
    else:
        best = 18
    eprint('client: SENT info best_move={}'.format(best))
    print('info best_move={}'.format(best), flush=True)
    line = next(inp)



