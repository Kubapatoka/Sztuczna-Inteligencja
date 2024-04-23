
def B(i,j):
    return 'B_%d_%d' % (i,j)

def storms(rows, cols, triples):
    writeln(':- use_module(library(clpfd)).')

    R = len(rows)
    C = len(cols)

    bs = [ B(i,j) for i in range(R) for j in range(C)]

    writeln('solve([' + ', '.join(bs) + ']) :- ')

    # ustalic zakresy
    writeln('[' + ', '.join(bs) + '] ins 0..1,')

    # sprawdzic sumy w wierszach
    for c,v in enumerate(rows):
        rs = [B(c,j) for j in range(C)]
        writeln('sum([' + ', '.join(rs) +  '], #= , ' + str(v)+ '), ')

    # i w kolumnach
    for c,v in enumerate(cols):
        rs = [B(j,c) for j in range(R)]
        writeln('sum([' + ', '.join(rs) +  '], #= , ' + str(v)+ '), ')

    #warunek na 4

    # AB
    # CD

    # A+D = 2 <=> B+C = 2
    for i in range(R-1):
        for j in range(C-1):
            a = B(i,j)
            b = B(i,j+1)
            c = B(i+1,j)
            d = B(i+1,j+1)
            writeln(a + ' + ' + d + ' #= 2 #<==> ' +  b + ' + ' + c + ' #= 2, ')


    # warunek na 3 dla wierszy
    tuples= []
    for i in range(R):
        for j in range(C-2):
            a = B(i,j)
            b = B(i,j+1)
            c = B(i,j+2)
            tuples.append('['+a+','+b+','+c+']')

    #pionowo
    for i in range(R-2):
        for j in range(C):
            a = B(i,j)
            b = B(i+1,j)
            c = B(i+2,j)
            tuples.append('['+a+','+b+','+c+']')


    writeln('tuples_in( [' + ', '.join(tuples)  + '], [ [0,0,0], [1,1,0], [1,0,0],[0,1,1], [0,0,1], [1,1,1], [1,0,1]]), ')

    #TODO: add some constraints
    for triple in triples:
        writeln('%s #= %d, ' % (B(triple[0], triple[1]), triple[2]))
    # writeln('    [%s] = [1,1,0,1,1,0,1,1,0,1,1,0,0,0,0,0,0,0,1,1,1,1,1,0,1,1,1,1,1,0,1,1,1,1,1,0],' % (', '.join(bs),)) #only for test 1

    #writeln('')
    writeln('   labeling([ff], [' +  ', '.join(bs) + ']).' )
    writeln('')
    writeln(":- tell('prolog_result.txt'), solve(X), write(X), nl, told.")
    # writeln(":- solve(X), write(X), nl.")

def writeln(s):
    output.write(s + '\n')

txt = open('zad_input.txt').readlines()
output = open('zad_output.txt', 'w')

rows = list(map(int, txt[0].split()))
cols = list(map(int, txt[1].split()))
triples = []

for i in range(2, len(txt)):
    if txt[i].strip():
        triples.append(list(map(int, txt[i].split())))

storms(rows, cols, triples)


