import sys
import random
import time 

fields = []
for i in range(10):
    line = input()
    fields.append(list(line))
#print(fields, file=sys.stderr, flush=True)
robot_count = int(input())
robots = []
for i in range(robot_count):
    inputs = input().split()
    x = int(inputs[0])
    y = int(inputs[1])
    dir = inputs[2]
    robots.append([x, y, dir])

start_time = time.time()

empty_fields = []
for i in range(19):
    for j in range(10):
        if fields[j][i] == ".":
            empty_fields.append((i, j))
empty_fields = set(empty_fields)

def evaluate(base_fields, arrows, initial_robots):
    fields = [row[:] for row in base_fields] # hardcopy
    for arrow in arrows:
        if fields[arrow[1]][arrow[0]] == '.':
            fields[arrow[1]][arrow[0]] = arrow[2]

    robots = [[r[0], r[1], r[2], True, set()] for r in initial_robots] # x, y, direction, is_alive, history
    
    score = 0
    active_count = len(robots)
    steps = 0
    max_steps = 300
    
    while active_count > 0 and steps < max_steps:
        score += active_count
        steps += 1
        
        for r in robots:
            if not r[3]:
                continue
                
            x, y, dir, alive, history = r
            
            cell = fields[y][x]
            if cell in ['U', 'D', 'L', 'R']:
                dir = cell
                r[2] = dir
                
            state = (x, y, dir)
            if state in history:
                r[3] = False
                active_count -= 1
                continue
            history.add(state)
            
            if dir == "U":
                y = (y - 1) % 10
            elif dir == "D":
                y = (y + 1) % 10
            elif dir == "L":
                x = (x - 1) % 19
            elif dir == "R":
                x = (x + 1) % 19
                
            r[0] = x
            r[1] = y
            
            if fields[y][x] == '#':
                r[3] = False
                active_count -= 1
                
    return score

max_res = []
max_score = 0
iter_count = 0
while time.time() - start_time < 0.8:
    res = []
    for field in empty_fields:
        dir = random.choice(['L', 'R', 'U', 'D', '.'])
        res.append([field[0], field[1], dir])

    score = evaluate(fields, res, robots)
    if score > max_score:
        max_score = score
        max_res = res
    iter_count += 1
    

print(f"Max score {max_score}", file=sys.stderr, flush=True)
res = ""
for arrow in max_res:
    if arrow[2] != ".":
        res += f'{arrow[0]} {arrow[1]} {arrow[2]} '
print(res)
print(time.time() - start_time, file=sys.stderr, flush=True)
print(iter_count, file=sys.stderr, flush=True)
