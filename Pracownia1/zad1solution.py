# ilość pól: 64 (8x8)
# w przybliżeniu mamy 64*64*64*2 stanów
# co daje ~524 288 stanów

letter_to_num = {'a': 1, 'b': 2, 'c': 3, 'd': 4, 'e': 5, 'f': 6, 'g': 7, 'h': 8}
num_to_letter = {1:'a', 2:'b', 3:'c', 4:'d', 5:'e', 6:'f', 7:'g', 8:'h'}
digit_to_num = {'1': 1, '2': 2, '3': 3, '4': 4, '5': 5, '6': 6, '7': 7, '8': 8}
num_to_digit = {1: '1', 2: '2', 3: '3', 4: '4', 5: '5', 6: '6', 7: '7', 8: '8'}


def pos_to_hash(x):
    l = letter_to_num[x[0]]
    c = digit_to_num[x[1]]
    res = (l-1)*8 + c
    return res

class Stan:
    def __init__(self, pl_col, wk, wt, bk):
        self.pl_col = pl_col == 'white'
        self.wk = wk
        self.wt = wt
        self.bk = bk

    def hasz(self):
        res = pos_to_hash(self.wk)
        res = res*64 + pos_to_hash(self.wt)
        res = res*64 + pos_to_hash(self.bk)
        if self.pl_col:
            res = res + 64*64*64
        return res

def possible_moves_f(p):
    res = []

    if p.pl_col:

        board = [[False for _ in range(9)] for _ in range(9)]

        bk = p.bk
        l = letter_to_num[bk[0]]
        c = digit_to_num[bk[1]]

        if l > 1:
            if c > 1 : board[l-1][c-1] = True
            board[l-1][c] = True
            if c < 8 : board[l-1][c+1] = True
        if c > 1 : board[l][c-1] = True
        board[l][c] = True
        if c < 8 : board[l][c+1] = True
        if l < 8 :
            if c > 1 : board[l+1][c-1] = True
            board[l+1][c] = True
            if c < 8 : board[l+1][c+1] = True


        wieza = p.wt
        l = letter_to_num[wieza[0]]
        c = digit_to_num[wieza[1]]
        for i in range(1,l):
            if p.wk != num_to_letter[i]+wieza[1]: res.append(Stan('black', p.wk, num_to_letter[i]+wieza[1], p.bk ))
        for i in range(l+1,9):
            if p.wk != num_to_letter[i]+wieza[1]: res.append(Stan('black', p.wk, num_to_letter[i]+wieza[1], p.bk ))
        for i in range(1,c):
            if p.wk != wieza[0]+num_to_digit[i]: res.append(Stan('black', p.wk, wieza[0]+num_to_digit[i], p.bk ))
        for i in range(c+1,9):
            if p.wk != wieza[0]+num_to_digit[i]: res.append(Stan('black', p.wk, wieza[0]+num_to_digit[i], p.bk ))

        krol = p.wk
        l = letter_to_num[krol[0]]
        c = digit_to_num[krol[1]]
        if l > 1:
            if c > 1 :
                if p.wt != num_to_letter[l-1]+num_to_digit[c-1] and board[l-1][c-1] == False:
                    res.append(Stan('black',num_to_letter[l-1]+num_to_digit[c-1] , p.wt, p.bk))
            if p.wt != num_to_letter[l-1]+num_to_digit[c] and board[l-1][c] == False: 
                res.append(Stan('black',num_to_letter[l-1]+num_to_digit[c] , p.wt, p.bk))
            if c < 8 :
                if p.wt != num_to_letter[l-1]+num_to_digit[c+1] and board[l-1][c+1] == False: 
                    res.append(Stan('black',num_to_letter[l-1]+num_to_digit[c+1] , p.wt, p.bk))

        if c > 1 :
            if p.wt != num_to_letter[l]+num_to_digit[c-1] and board[l][c-1] == False:
                res.append(Stan('black',num_to_letter[l]+num_to_digit[c-1] , p.wt, p.bk))
        if c < 8 :
            if p.wt != num_to_letter[l]+num_to_digit[c+1] and board[l][c+1] == False:
                res.append(Stan('black',num_to_letter[l]+num_to_digit[c+1] , p.wt, p.bk))

        if l < 8 :
            if c > 1 :
                if p.wt != num_to_letter[l+1]+num_to_digit[c-1] and board[l+1][c-1] == False:
                    res.append(Stan('black',num_to_letter[l+1]+num_to_digit[c-1] , p.wt, p.bk))
            if p.wt != num_to_letter[l+1]+num_to_digit[c] and board[l+1][c] == False:
                res.append(Stan('black',num_to_letter[l+1]+num_to_digit[c] , p.wt, p.bk))
            if c < 8 :
                if p.wt != num_to_letter[l+1]+num_to_digit[c+1] and board[l+1][c+1] == False:
                    res.append(Stan('black',num_to_letter[l+1]+num_to_digit[c+1] , p.wt, p.bk))
    else :
        board = [[False for _ in range(9)] for _ in range(9)]

        wt = p.wt
        l = letter_to_num[wt[0]]
        c = digit_to_num[wt[1]]

        for i in range(9):
            board[i][c] = True
            board[l][i] = True
        board[l][c] = False

        wk = p.wk
        l = letter_to_num[wk[0]]
        c = digit_to_num[wk[1]]

        if l > 1:
            if c > 1 : board[l-1][c-1] = True
            board[l-1][c] = True
            if c < 8 : board[l-1][c+1] = True
        if c > 1 : board[l][c-1] = True
        board[l][c] = True
        if c < 8 : board[l][c+1] = True
        if l < 8 :
            if c > 1 : board[l+1][c-1] = True
            board[l+1][c] = True
            if c < 8 : board[l+1][c+1] = True

        krol = p.bk
        l = letter_to_num[krol[0]]
        c = digit_to_num[krol[1]]

        if l > 1:
            if c > 1 :
                if not board[l-1][c-1]:
                    res.append(Stan('white', p.wk, p.wt, num_to_letter[l-1]+num_to_digit[c-1]))
            if not board[l-1][c]:
                res.append(Stan('white', p.wk, p.wt, num_to_letter[l-1]+num_to_digit[c]))
            if c < 8 : 
                if not board[l-1][c+1]:
                    res.append(Stan('white', p.wk, p.wt,num_to_letter[l-1]+num_to_digit[c+1]))
        
        if c > 1 : 
            if not board[l][c-1]:
                res.append(Stan('white', p.wk, p.wt,num_to_letter[l]+num_to_digit[c-1]))
        if c < 8 : 
            if not board[l][c+1]:
                res.append(Stan('white', p.wk, p.wt,num_to_letter[l]+num_to_digit[c+1]))
        
        if l < 8 :
            if c > 1 : 
                if not board[l+1][c-1]:
                    res.append(Stan('white', p.wk, p.wt,num_to_letter[l+1]+num_to_digit[c-1]))
            if not board[l+1][c]:
                res.append(Stan('white', p.wk, p.wt,num_to_letter[l+1]+num_to_digit[c]))
            if c < 8 : 
                if not board[l+1][c+1]:
                    res.append(Stan('white', p.wk, p.wt,num_to_letter[l+1]+num_to_digit[c+1]))

    return res


def possible_moves_f_rev(p):
    res = []

    if not p.pl_col:
        wieza = p.wt
        l = letter_to_num[wieza[0]]
        c = digit_to_num[wieza[1]]
        for i in range(1,l):
            if p.wk != num_to_letter[i]+wieza[1]: res.append(Stan('white', p.wk, num_to_letter[i]+wieza[1], p.bk ))
        for i in range(l+1,9):
            if p.wk != num_to_letter[i]+wieza[1]: res.append(Stan('white', p.wk, num_to_letter[i]+wieza[1], p.bk ))
        for i in range(1,c):
            if p.wk != wieza[0]+num_to_digit[i]: res.append(Stan('white', p.wk, wieza[0]+num_to_digit[i], p.bk ))
        for i in range(c+1,9):
            if p.wk != wieza[0]+num_to_digit[i]: res.append(Stan('white', p.wk, wieza[0]+num_to_digit[i], p.bk ))

        krol = p.wk
        l = letter_to_num[krol[0]]
        c = digit_to_num[krol[1]]
        if l > 1:
            if c > 1 :
                if p.wt != num_to_letter[l-1]+num_to_digit[c-1]:
                    res.append(Stan('white',num_to_letter[l-1]+num_to_digit[c-1] , p.wt, p.bk))
            if p.wt != num_to_letter[l-1]+num_to_digit[c]: 
                res.append(Stan('white',num_to_letter[l-1]+num_to_digit[c] , p.wt, p.bk))
            if c < 8 :
                if p.wt != num_to_letter[l-1]+num_to_digit[c+1]: 
                    res.append(Stan('white',num_to_letter[l-1]+num_to_digit[c+1] , p.wt, p.bk))

        if c > 1 :
            if p.wt != num_to_letter[l]+num_to_digit[c-1]:
                res.append(Stan('white',num_to_letter[l]+num_to_digit[c-1] , p.wt, p.bk))
        if c < 8 :
            if p.wt != num_to_letter[l]+num_to_digit[c+1]:
                res.append(Stan('white',num_to_letter[l]+num_to_digit[c+1] , p.wt, p.bk))

        if l < 8 :
            if c > 1 :
                if p.wt != num_to_letter[l+1]+num_to_digit[c-1]:
                    res.append(Stan('white',num_to_letter[l+1]+num_to_digit[c-1] , p.wt, p.bk))
            if p.wt != num_to_letter[l+1]+num_to_digit[c]:
                res.append(Stan('white',num_to_letter[l+1]+num_to_digit[c] , p.wt, p.bk))
            if c < 8 :
                if p.wt != num_to_letter[l+1]+num_to_digit[c+1]:
                    res.append(Stan('white',num_to_letter[l+1]+num_to_digit[c+1] , p.wt, p.bk))
    else :
        krol = p.bk
        l = letter_to_num[krol[0]]
        c = digit_to_num[krol[1]]

        if l > 1:
            if c > 1 :
                res.append(Stan('black', p.wk, p.wt, num_to_letter[l-1]+num_to_digit[c-1]))
            res.append(Stan('black', p.wk, p.wt, num_to_letter[l-1]+num_to_digit[c]))
            if c < 8 : 
                res.append(Stan('black', p.wk, p.wt,num_to_letter[l-1]+num_to_digit[c+1]))
        
        if c > 1 : 
            res.append(Stan('black', p.wk, p.wt,num_to_letter[l]+num_to_digit[c-1]))
        if c < 8 : 
            res.append(Stan('black', p.wk, p.wt,num_to_letter[l]+num_to_digit[c+1]))
        
        if l < 8 :
            if c > 1 : 
                res.append(Stan('black', p.wk, p.wt,num_to_letter[l+1]+num_to_digit[c-1]))
            res.append(Stan('black', p.wk, p.wt,num_to_letter[l+1]+num_to_digit[c]))
            if c < 8 : 
                res.append(Stan('black', p.wk, p.wt,num_to_letter[l+1]+num_to_digit[c+1]))

    new_res = []

    for re in res:
        pos = possible_moves_f(re)
        contains = False
        for r in pos:
            #print("         stąd mogę dojść z re: wt,wk: ", r.wt, r.wk, "bk: ", r.bk, "odl od mata: ", stany[r.hasz()], "color: ", r.pl_col)
            if r.pl_col == p.pl_col and r.bk == p.bk and r.wk == p.wk and p.wt == r.wt :
                contains = True
        if contains : 
            new_res.append(re)
            #print("     dodaje do res: wt,wk: ", re.wt, re.wk, "bk: ", re.bk, "odl od mata: ", stany[re.hasz()], "color: ", re.pl_col)
        # else:    
            #print("     nie nie nie dodaje do res: wt,wk: ", re.wt, re.wk, "bk: ", re.bk, "odl od mata: ", stany[re.hasz()], "color: ", re.pl_col)
                 


    return new_res

def mat(x):
    board = [[False for _ in range(9)] for _ in range(9)]

    wt = x.wt
    l = letter_to_num[wt[0]]
    c = digit_to_num[wt[1]]

    for i in range(9):
        board[i][c] = True
        board[l][i] = True
    board[l][c] = False

    wk = x.wk
    l = letter_to_num[wk[0]]
    c = digit_to_num[wk[1]]

    if l > 1:
        if c > 1 : board[l-1][c-1] = True
        board[l-1][c] = True
        if c < 8 : board[l-1][c+1] = True
    if c > 1 : board[l][c-1] = True
    # board[l][c] = True
    if c < 8 : board[l][c+1] = True
    if l < 8 :
        if c > 1 : board[l+1][c-1] = True
        board[l+1][c] = True
        if c < 8 : board[l+1][c+1] = True


    bk = x.bk
    l = letter_to_num[bk[0]]
    c = digit_to_num[bk[1]]

    is_mat = True

    if l > 1:
        if c > 1 :
            if not board[l-1][c-1]: is_mat = False
        if not board[l-1][c] : is_mat = False
        if c < 8 : 
            if not board[l-1][c+1]: is_mat = False

    if c > 1 : 
        if not board[l][c-1]: is_mat = False
    if not board[l][c]: is_mat = False
    if c < 8 : 
        if not board[l][c+1]: is_mat = False

    if l < 8 :
        if c > 1 : 
            if not board[l+1][c-1]: is_mat = False
        if not board[l+1][c]: is_mat = False
        if c < 8 : 
            if not board[l+1][c+1]: is_mat = False

    return is_mat

def pat(x):
    if x.wt == x.bk or x.bk == x.wk: return True
    return False


file = open("zad1_input.txt")
output = open("zad1_output.txt", 'w')
debug = True

stany = []
vis = []
INF = 540000

for i in range(1,530000):
    stany.append(INF)
    vis.append(False)

queue = []

for line in file:
    player, white_king, white_tower, black_king = line.split()
    if player != 'white' and player != 'black' : break 
    stan_pocz = Stan(player, white_king, white_tower, black_king)
    hasz_pocz = stan_pocz.hasz()
    vis[hasz_pocz] = True
    queue.append(stan_pocz)

    #first_mat_or_pat = stan_pocz

    while len(queue) != 0:
        #if len(queue) == 0 : break
        curr = queue.pop(0)
        #vis[curr.hasz()] = True
        possible_moves = possible_moves_f(curr)

        if mat(curr) :
            first_mat_or_pat = curr
            stany[curr.hasz()] = 0
            break

        if pat(curr) or len(possible_moves) == 0:
            first_mat_or_pat = curr

        else :
            for s in  possible_moves:
                if vis[s.hasz()] == False:
                    vis[s.hasz()] = True
                    queue.append(s)

    queue.clear()
    if stany[first_mat_or_pat.hasz()] != 0:
        print("PAT: ", first_mat_or_pat.wt, first_mat_or_pat.wk, first_mat_or_pat.bk)
        output.write("INF")
        continue

    # if first_mat_or_pat.pl_col == 'white' :
    #     new_col = 'black' 
    # else : 
    #     new_col ='white'

    # new_stan = Stan(new_col, first_mat_or_pat.wk, first_mat_or_pat.wt, first_mat_or_pat.bk)
    # stany[new_stan.hasz()] = 1
    # queue.append(new_stan)
    queue.append(first_mat_or_pat)

    while len(queue) != 0:
        curr = queue.pop(0)
        #print("DROGA: wt,wk: ", curr.wt, curr.wk, "bk: ", curr.bk, "odl od mata: ", stany[curr.hasz()], "color: ", curr.pl_col)        
        if curr.pl_col == stan_pocz.pl_col and curr.wt == stan_pocz.wt and curr.wk == stan_pocz.wk and curr.bk == stan_pocz.bk:
            break
        possible_moves = possible_moves_f_rev(curr)
        for s in  possible_moves:
            #print("     Poss: wt,wk: ", s.wt, s.wk, "bk: ", s.bk, "odl od mata: ", stany[s.hasz()])        
            if stany[s.hasz()] > stany[curr.hasz()]+1:
                stany[s.hasz()] = stany[curr.hasz()]+1
                if not queue.__contains__(s):
                    queue.append(s)

    if debug:
        # TODO
        lpoz = stany[stan_pocz.hasz()]
        output.write(str(lpoz))
        curr = stan_pocz
        for i in range(lpoz,0, -1) :
            print("STAN: wt,wk: ", curr.wt, curr.wk, "bk: ", curr.bk, "odl od mata: ", stany[curr.hasz()])
            szuk = i-1
            ruchy = possible_moves_f(curr)
            for s in ruchy:
                #print("     Poss: wt,wk: ", s.wt, s.wk, "bk: ", s.bk, "odl od mata: ", stany[s.hasz()])        
                if stany[s.hasz()] == szuk: 
                    curr = s
                    break
        print("STAN: wt,wk: ", curr.wt, curr.wk, "bk: ", curr.bk, "odl od mata: ", stany[curr.hasz()])




    else: output.write(str(stany[stan_pocz.hasz()]))




