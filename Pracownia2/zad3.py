import copy

from queue import PriorityQueue

def hasz(stan):
    h = len(stan)*97
    for y,x in stan:
        h += (x*47+y)*47
        h = h%100000007
    return h

def win(c):
    for s in c:
        if s not in goals: return False
    return True

def ruch(stan, direction):
    res = []
    for sy,sx in stan:
        nx = sx
        ny = sy

        if direction == 'U':
            ny-=1
        elif direction == 'D':
            ny+=1
        elif direction == 'R':
            nx+=1
        elif direction == 'L':
            nx-=1
        
        if board[ny][nx] == '#':
            # print(ny,nx, "is a wall")
            ny,nx = sy,sx
        
        if (ny, nx) not in res:
            res.append((ny,nx))
    return res

def heura(stan):
    res = 0 #len(stan)
    for s in stan:
        m = 10e9
        for g in goals:
            m = min(m, abs(s[0] - g[0]) + abs(s[1] - g[1]))
        res += m*m
    return res

input = open("zad_input.txt")
output = open("zad_output.txt", 'w')

board_temp = []

for line in input:
    board_temp.append(line)

h = len(board_temp)
w = len(board_temp[0])

#list of goals
goals = []

#list of start points
start = []

board = [['#' for _ in range(w)] for _ in range(h)]

for i in range(h):
    for j in  range(w):
        if(board_temp[i][j] == 'G' or board_temp[i][j] == 'B'):
            goals.append((i,j))
        if board_temp[i][j] == 'S' or board_temp[i][j] == 'B':
            start.append((i,j))
        if board_temp[i][j] != '#':
            board[i][j] = ' '

path1 = ''

# for s in start:
#     print(s)

visited = []

queue = PriorityQueue()
queue.put((heura(start), copy.deepcopy(start), path1))

# print(len(start))

while not queue.empty():
    _,curr,path = queue.get()

    # print(len(curr), path,'\n')

    hasz_curr = hasz(curr)

    if hasz_curr in visited: continue

    visited.append(hasz_curr)

    if win(curr):
        output.write(path)
        print(path)
        break

    lpp1 = 100*(len(path)+1)

    su = ruch(curr, 'U')
    queue.put((lpp1+heura(su),su, path+'U'))

    sr = ruch(curr, 'R')
    queue.put((lpp1+heura(sr),sr, path+'R'))

    sd = ruch(curr, 'D')
    queue.put((lpp1+heura(sd),sd, path+'D'))
    
    sl = ruch(curr, 'L')
    queue.put((lpp1+heura(sl),sl, path+'L'))