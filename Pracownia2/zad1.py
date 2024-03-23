import copy
    # Estimated computer speed= 1.1163896966349705
    # Input is passed using zad_input.txt and contains:
    # 5 5
    # 5
    # 1 1 1
    # 3
    # 2 2
    # 5
    # 2 2
    # 1 3
    # 3 1
    # 1 3
    # 2 2

    # Output is expected in zad_output.txt with contents:
    # #####
    # #.#.#
    # .###.
    # ##.##
    # #####

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
    

def char(field):
    if field.is_black:
        return '#'
    elif field.is_white:
        return '.'
    else:
        return ' '

def possibilities(dan, pos, max_pos, descr):
    dane = copy.deepcopy(dan)
    if len(dane) == 0:
        #print("dane is empty, pos:",pos,"descr: ", ''.join(descr))
        for i in range(pos,max_pos):
            descr.append('.')
        return [''.join(descr)]
    
    mini = sum(dane) + len(dane) - 1

    ile_opcji = max_pos-pos-mini
    if ile_opcji < 0:
        return
    
    # print("ile opcji: ", ile_opcji)

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
            # print("i: ", i, "dane:", dane, "r from possibilities: ", r)
            res.append(r)
    return res



    

def dopasuj(dane, bcpy):
    board_cpy = copy.deepcopy(bcpy)
    n = len(board_cpy)

    for b in board_cpy:
        b.can_be_white = False
        b.can_be_black = False
    
    pos_opt = possibilities(copy.deepcopy(dane),0,n,[])
    
    for p in pos_opt:
        p_is_good = True
        for c in range(len(p)):
            if board_cpy[c].is_white and p[c] == '#': 
                p_is_good = False
                break
            if board_cpy[c].is_black and p[c] == '.': 
                p_is_good = False
                break
        if p_is_good:
            for c in range(len(p)):
                if not board_cpy[c].is_white and not board_cpy[c].is_black:
                    if p[c] == '#': board_cpy[c].can_be_black = True
                    else: board_cpy[c].can_be_white = True
    
    for c in range(n):
        if not board_cpy[c].is_white and not board_cpy[c].is_black:
            if board_cpy[c].can_be_black and not board_cpy[c].can_be_white: board_cpy[c].is_black = True
            elif board_cpy[c].can_be_white and not board_cpy[c].can_be_black: board_cpy[c].is_white = True

    # print('dopasuj po zmiana \n')
    # for row in board:            
    #     row_str = map(lambda f : char(f),  row)
    #     print(''.join(row_str))

    # print("\ndopasuj dane:", dane, "ret: ", ''.join(map(lambda f : char(f), board_cpy)), '\n')
    return board_cpy




input = open("zad_input.txt")
output = open("zad_output.txt", 'w')

w = -1
h = -1

for line in input:
    if h == -1 or w == -1:
        h,w = map(int,line.split())
        rows = []
        columns = []
    elif len(rows) < h:
        rows.append(list(map(int, line.split())))
    elif len(columns) < w:
        columns.append(list(map(int, line.split())))

# print("h: ", h, " w: ", w)
# print("rows: ")
# for c in rows:
#     print(c)

# print("columns: ")
# for c in columns:
#     print(c)


board = [[field for _ in range(w)] for _ in range(h)]
# loop = 0
while True:
    # loop += 1
    for i in range(h):
        # print("row: ", i, "dane: ", rows[i])

        n_board_i = []
        for j in  range(w):
            n_board_i.append(field(board[i][j]))
        n_board_i = dopasuj(copy.deepcopy(rows[i]), n_board_i)

        for j in  range(w):
            board[i][j] = field(n_board_i[j])

        # print('\n')
        # for row in board:
        #     row_str = map(lambda f : char(f),  row)
        #     print(''.join(row_str))

    for i in range(w):
        # print("col: ", i, " col[i]",columns[i])

        kol = []
        for j in range(h):
            kol.append(field(board[j][i]))
        kol = dopasuj(copy.deepcopy(columns[i]), kol)
        for j in range(h):
            board[j][i] = field(kol[j])
    
        # print('\n')
        # for row in board:
        #     row_str = map(lambda f : char(f),  row)
        #     print(''.join(row_str))

    
    good = True
    for i in range(h):
        for j in range(w):
            if (not board[i][j].is_black) and (not board[i][j].is_white): 
                good = False
                # print("i,j", i,j)
    
    # print("good", good) 
    if good:
        break

# res1 = possibilities([1,1,1,1], 0, 10, [])
# # print(res1)

# for r in res1:
#     print(r)

# print('\n')

for row in board:
    row_str = map(lambda f : char(f),  row)
    res = ''.join(row_str)
    #print(''.join(row_str))
    output.write(res)
    output.write('\n')