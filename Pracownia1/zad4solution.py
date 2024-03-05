# Opis:
# Łatwo rozwiązywalne za pomocą sum prefiksowych
# IMHO kod jest na tyle krótki i czytelny że nie będę się produkować

file = open("zad4_input.txt")
output = open("zad4_output.txt", 'w')


for line in file:
    if len(line) == 0 : break

    board = []
    board_str, n = line.split()
    for b in board_str:
        if b == '0' : board.append(0)
        else : board.append(1)

    n = int(n)
    pref = []
    pocz = 0
    for b in board:
        if b == 1: pocz = pocz+1
        pref.append(pocz)

    if n == 0:
        res = pref[length-1]
        output.write(str(res)+'\n')
        print (res)
        continue

    length = len(board)
    res = (n-pref[n-1]) + pref[length-1]-pref[n-1]
    for i in range(1,len(board)-n+1):
        m = pref[i-1]                       # 1 do usuniecia
        m+= n-(pref[i+n-1] - pref[i-1])     # 0 do usuniecia
        m+= pref[length-1] - pref[i+n-1]    # 1 do usuniecia
        res = min(res, m)

    output.write(str(res)+'\n')
    print (res)

