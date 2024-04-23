import copy

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

# for s in start:
#     print(s)

# print("RRRR")
path1 = ''

for _ in range(20):
    start = ruch(start, 'R')
    path1 += 'R'

for _ in range(20):
    start = ruch(start, 'D')
    path1 += 'D'

for _ in range(20):
    start = ruch(start, 'L')
    path1 += 'L'

for _ in range(20):
    start = ruch(start, 'U')
    path1 += 'U'


# for s in start:
#     print(s)

visited = []

queue = []
queue.append((copy.deepcopy(start), path1))

# print(len(start))

while len(queue) != 0:
    curr,path = queue.pop(0)

    hasz_curr = hasz(curr)

    # print(curr, "hasz: ", hasz_curr)

    if hasz_curr in visited: continue

    visited.append(hasz_curr)

    if win(curr):
        output.write(path)
        print(path)
        break

    queue.append((ruch(curr, 'U'), path+'U'))
    queue.append((ruch(curr, 'R'), path+'R'))
    queue.append((ruch(curr, 'D'), path+'D'))
    queue.append((ruch(curr, 'L'), path+'L'))