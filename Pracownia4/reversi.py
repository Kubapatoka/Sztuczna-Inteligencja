import random
import sys
from collections import defaultdict as dd
#from turtle import *
from copy import deepcopy
#import cProfile
#from profilestats import profile

M = 8

#@profile
def initial_board():
    B = [ [None] * M for i in range(M)]
    B[3][3] = 1
    B[4][4] = 1
    B[3][4] = 0
    B[4][3] = 0
    return B

#@profile
def vbp(b, p):
    if b == p: return 1
    if b == 1-p: return -1
    return 0
    
class Board:
    dirs  = [ (0,1), (1,0), (-1,0), (0,-1), (1,1), (-1,-1), (1,-1), (-1,1) ]

    def __init__(self):
        self.board = initial_board()
        self.fields = set()
        self.move_list = []
        #self.history = []
        for i in range(M):
            for j in range(M):
                if self.board[i][j] == None:
                    self.fields.add( (j,i) )

    def draw(self):
        for i in range(M):
            res = []
            for j in range(M):
                b = self.board[i][j]
                if b == None:
                    res.append('.')
                elif b == 1:
                    res.append('#')
                else:
                    res.append('o')
            print (''.join(res))
        print()


    def moves(self, player):
        res = []
        for (x,y) in self.fields:
            if any( self.can_beat(x,y, direction, player) for direction in Board.dirs):
                res.append( (x,y) )
        if not res:
            return [None]
        return res

    def can_beat(self, x,y, d, player):
        dx,dy = d
        x += dx
        y += dy
        cnt = 0
        while self.get(x,y) == 1-player:
            x += dx
            y += dy
            cnt += 1
        return cnt > 0 and self.get(x,y) == player

    def get(self, x,y):
        if 0 <= x < M and 0 <=y < M:
            return self.board[y][x]
        return None

    def do_move(self, move, player):
        #self.history.append([x[:] for x in self.board])
        self.move_list.append(move)

        if move == None:
            return
        x,y = move
        x0,y0 = move
        self.board[y][x] = player
        self.fields -= set([move])
        for dx,dy in self.dirs:
            x,y = x0,y0
            to_beat = []
            x += dx
            y += dy
            while self.get(x,y) == 1-player:
              to_beat.append( (x,y) )
              x += dx
              y += dy
            if self.get(x,y) == player:
                for (nx,ny) in to_beat:
                    self.board[ny][nx] = player

    def result(self):
        res = 0
        for y in range(M):
            for x in range(M):
                b = self.board[y][x]
                if b == 0:
                    res -= 1
                elif b == 1:
                    res += 1
        return res

    def terminal(self):
        if not self.fields:
            return True
        if len(self.move_list) < 2:
            return False
        return self.move_list[-1] == self.move_list[-2] == None 

    def random_move(self):
        ms = self.moves(1)
        if ms:
            return random.choice(ms)
        return [None]

    def heuristic(self, player):
        res = 0

        a = 60
        res += a*vbp(self.board[0][0], player)
        res += a*vbp(self.board[0][7], player)
        res += a*vbp(self.board[7][0], player)
        res += a*vbp(self.board[7][7], player)

        b = -30
        res += b*vbp(self.board[0][1], player)
        res += b*vbp(self.board[0][6], player)
        res += b*vbp(self.board[1][0], player)
        res += b*vbp(self.board[1][7], player)
        res += b*vbp(self.board[6][0], player)
        res += b*vbp(self.board[6][7], player)
        res += b*vbp(self.board[7][1], player)
        res += b*vbp(self.board[7][6], player)

        c = -40
        res += c*vbp(self.board[1][1], player)
        res += c*vbp(self.board[1][6], player)
        res += c*vbp(self.board[6][1], player)
        res += c*vbp(self.board[6][6], player)

        d = 25
        for i in range(2, 6):
            res += d*vbp(self.board[0][i], player)
            res += d*vbp(self.board[7][i], player)
            res += d*vbp(self.board[i][0], player)
            res += d*vbp(self.board[i][7], player)

        e = -25
        for i in range(2, 6):
            res += e*vbp(self.board[1][i], player)
            res += e*vbp(self.board[6][i], player)
            res += e*vbp(self.board[i][1], player)
            res += e*vbp(self.board[i][6], player)

        f = 1
        for i in range(2,6): 
            for j in range(2,6):
                res += f*vbp(self.board[i][j], player)

        c = len(self.moves(player))
        v = 5
        res += c*v

        return res

    def my_agent(self):
        return ABalg(self,0)


def ABalg(board, player):
    ms = board.moves(player)

    if not ms: return [None]

    best_m = ms[0]

    depth = 1

    bcpy = deepcopy(board)
    bcpy.do_move(best_m, player)
    inf = 10e8
    a = -inf
    b = inf
    best_m_v = MMAB(False, depth, 1-player, bcpy, a, b)
    for m in ms:
        bcpy = deepcopy(board)
        bcpy.do_move(m, player)
        cv = MMAB(False, depth, 1-player, bcpy, a, b)
        if cv > best_m_v:
            best_m_v = cv
            best_m = m

    return best_m

    
def MMAB(is_max, depth, player, board, a,b):
    if depth == 0: return board.heuristic(player=0)
    if len(board.moves(player)) == 0 and len(board.moves(1-player)) == 0: return board.heuristic(player=0) #my player is player 0

    if is_max:
        ms = board.moves(player)
        for m in ms:
            bcpy = deepcopy(board)
            bcpy.do_move(m, player)
            v = MMAB(~is_max, depth-1, 1-player, bcpy, a,b)
            #print('depth: ', depth, ' a: ', a, ' v: ', v)
            a = max(a,v)
            if a>=b : break
        return a

    if not is_max:
        ms = board.moves(player)
        for m in ms:
            bcpy = deepcopy(board)
            bcpy.do_move(m, player)
            v = MMAB(~is_max, depth-1, 1-player, bcpy, a,b)
            #print('depth: ', depth, ' b: ', b, ' v: ', v)
            b = min(b,v)
            if b <= a : break
        return b

def gra():
    player = random.randint(0,1)
    B = Board()

    while True:
        #B.draw()
        if player == 1:
            m = B.random_move()
        else:
            m = B.my_agent()
        B.do_move(m, player)
        player = 1-player
        if B.terminal():
            break

    if B.result() < 0:
        return True
    else:
        return False

res = 0
n = 1000
for i in range(n):
    if gra():
        res +=1
        print(i, ': you win, actual result: ', res)
    else:
        print(i, ':you fail, actual result: ', res)

print('you win:', res, '/',n)
sys.exit(0)
