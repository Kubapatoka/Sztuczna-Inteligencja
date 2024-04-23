import copy
import random
from typing import Optional

# Opis wejścia: wysokość - h, szerokość - w
# wiadomo

class field:
    is_white = False
    is_black = False
    can_be_white = False
    can_be_black = False

    def __init__(self, orig=None):
        if orig is None:
            self.non_copy_constructor()
        else:
            self.copy_constructor(orig)
    def __eq__(self, other):
        return self.is_black == other.is_black and self.is_white == other.is_white
    def non_copy_constructor(self):
        self.is_white = False
        self.is_black = False
        self.can_be_white = False
        self.can_be_black = False
    def copy_constructor(self, orig):
        self.is_white = orig.is_white
        self.is_black = orig.is_black
        self.can_be_white = orig.can_be_white
        self.can_be_black = orig.can_be_black
    def is_initialized(self):
        return self.is_black or self.is_white


def char(field):
    if field.is_black:
        return '#'
    elif field.is_white:
        return '.'
    else:
        return 'x'


def possibilities(dan, pos, max_pos, descr):
    dane = copy.deepcopy(dan)
    if len(dane) == 0:
        for i in range(pos,max_pos):
            descr.append('.')
        return [''.join(descr)]
    mini = sum(dane) + len(dane) - 1

    ile_opcji = max_pos-pos-mini
    if ile_opcji < 0:
        return

    d = dane.pop(0)
    res = []
    for i in range(0, ile_opcji+1):
        descr_cpy = descr.copy()
        for j in range(0,i): descr_cpy.append('.')
        for j in range(0,d): descr_cpy.append('#')
        new_pos = pos+i+d
        if len(dane) > 0:
            descr_cpy.append('.')
            new_pos+=1
        r1 = possibilities(dane.copy(),new_pos, max_pos, descr_cpy)
        for r in r1:
            res.append(r)
    return res


def dopasuj(dane, bcpy):
    board_cpy = copy.deepcopy(bcpy)
    n = len(board_cpy)

    for b in board_cpy:
        b.can_be_white = False
        b.can_be_black = False

    is_failure = True

    for p in dane:
        p_is_good = True
        for c in range(len(p)):
            if board_cpy[c].is_white and p[c] == '#':
                p_is_good = False
                break
            if board_cpy[c].is_black and p[c] == '.':
                p_is_good = False
                break
        if p_is_good:
            is_failure = False
            for c in range(len(p)):
                if not board_cpy[c].is_white and not board_cpy[c].is_black:
                    if p[c] == '#': board_cpy[c].can_be_black = True
                    else: board_cpy[c].can_be_white = True
    
    if is_failure: return None

    for c in range(n):
        if not board_cpy[c].is_white and not board_cpy[c].is_black:
            if board_cpy[c].can_be_black and not board_cpy[c].can_be_white: board_cpy[c].is_black = True
            elif board_cpy[c].can_be_white and not board_cpy[c].can_be_black: board_cpy[c].is_white = True

    return board_cpy


def dopasuj_ogl(board_cpy):

    for i in range(h):
        n_board_i = []
        for j in  range(w):
            n_board_i.append(field(board_cpy[i][j]))
        n_board_i = dopasuj(copy.deepcopy(rows_poss[i]), n_board_i)
        if n_board_i == None: return None

        for j in  range(w):
            board_cpy[i][j] = field(n_board_i[j])

    for i in range(w):

        kol = []
        for j in range(h):
            kol.append(field(board_cpy[j][i]))
        kol = dopasuj(copy.deepcopy(columns_poss[i]), kol)
        if kol == None: return None

        for j in range(h):
            board_cpy[j][i] = field(kol[j])

    return board_cpy


def BT(board_input):
    oldold_board = copy.deepcopy(board_input)
    while True:
        h1 = random.randrange(0,h)
        w1 = random.randrange(0,w)
        board_cpy = copy.deepcopy(oldold_board)
        if not board_cpy[h1][w1].is_initialized():
            board_cpy[h1][w1].is_black = True


        print("\nTry:\n")
        for row in board_cpy:
            row_str = map(lambda f : char(f),  row)
            print(''.join(row_str))

        while True:
            old_board = copy.deepcopy(board_cpy)
            board_cpy = dopasuj_ogl(board_cpy)
            if board_cpy == None: return None
            good = True
            for i in range(h):
                for j in range(w):
                    if not board_cpy[i][j].is_initialized():
                        good = False

            if compare_boards(old_board, board_cpy) or good: break
        if good: return Optional(board_cpy)
        new_bo = BT(copy.deepcopy(board_cpy))
        if new_bo == None:
            board_cpy = copy.deepcopy(oldold_board)
            board_cpy[h1][w1].is_black = False
            board_cpy[h1][w1].is_white = True
            while True:
                old_board = copy.deepcopy(board_cpy)
                board_cpy = dopasuj_ogl(board_cpy)
                if board_cpy == None: return None
                good = True
                for i in range(h):
                    for j in range(w):
                        if not board_cpy[i][j].is_initialized():
                            good = False

                if compare_boards(old_board, board_cpy) or good: break
            if good: return Optional(board_cpy)
            new_bo = BT(copy.deepcopy(board_cpy))
            if new_bo == None: return None

        good1 = True
        for i in range(h):
            for j in range(w):
                if not new_bo[i][j].is_initialized():
                    good1 = False
        if good1: return Optional(new_bo)


def compare_boards(b1,b2):
    for i in range(h):
        for j in range(w):
            if not b1[i][j] == b2[i][j]: return False
    return True

input = open("zad_input.txt")
output = open("zad_output.txt", 'w')

w = -1
h = -1

for line in input:
    if h == -1 or w == -1:
        h,w = map(int,line.split())
        rows = []
        columns = []
        rows_poss = []
        columns_poss = []
    elif len(rows) < h:
        rows.append(list(map(int, line.split())))
        rows_poss.append(possibilities(rows[-1], 0,w, []))
    elif len(columns) < w:
        columns.append(list(map(int, line.split())))
        columns_poss.append(possibilities(columns[-1], 0,h,[]))

board = [[field for _ in range(w)] for _ in range(h)]

# print("before first loop")
while True:
    old_board = copy.deepcopy(board)
    # print("\n Old and new:\n")
    for row in old_board:
        row_str = map(lambda f : char(f),  row)
        # print(''.join(row_str))
    
    board = dopasuj_ogl(board)
    # print("\n")
    for row in board:
        row_str = map(lambda f : char(f),  row)
        # print(''.join(row_str))

    is_the_same = compare_boards(old_board, board)
    # print("is the same", is_the_same)
    if is_the_same: break

# for row in board:
#     row_str = map(lambda f : char(f),  row)
#     print(''.join(row_str))

res1 = BT(board)


for row in res1:
    row_str = map(lambda f : char(f),  row)
    res = ''.join(row_str)
    #print(res)
    output.write(res)
    output.write('\n')