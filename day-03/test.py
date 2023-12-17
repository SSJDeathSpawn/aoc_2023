import re
from collections import defaultdict
from math import prod
from sys import stdin

if '__main__' == __name__:
    lines = list(stdin)
    lineLength = len(lines[0])

    if any(len(line) != lineLength for line in lines):
        print('Error: Input lines are not all the same length.')
        exit(-1)

    data = ''.join(lines)
    vectors = tuple(lineLength * row + col
                    for row in range(-1, 2) for col in range(-1, 2)
                    if row or col)

    total = 0
    gears = defaultdict(list)
    for match in re.finditer(r'\d+', data):
        for i in {ci for mi in range(*match.span()) for v in vectors
                  if 0 <= (ci := mi + v) < len(data)}:
            if (checkChar := data[i]) not in '\n.0123456789':
                total += (partNumber := int(match.group()))
                if checkChar == '*':
                    gears[i].append(partNumber)
                break
    
    for l in sorted(list(filter(lambda x: len(x) == 2, map(sorted, gears.values()))), key= lambda x: x[0]):
        print(l)
