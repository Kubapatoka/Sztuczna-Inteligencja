file = open("zad2_input.txt")
output = open("zad2_output.txt", 'w')
dict = open("words_for_ai1.txt")

d = []
for w in dict:
    w = w.strip()
    #print('#',w, '#')
    d.append(w)

def DFS(r1, s1):
    # print("DFS( ", r1, "\n", s1, ")\n")

    if len(s1) == 0: 
        q = r1.split()[-1]
        # print ("q: ", q)
        if q in d:
            # print("stan dobry: ", r1)
            return r1
        else :
            return ''

    r2 = r1+s1[0]
    d1 = DFS(r2, s1[1:])

    # print ('start')
    # #for r in r1.split():
    # print('r1.split()[-1]: ', r1.split()[-1], " is in  d? ", r1.split()[-1] in d)
    # print ('finish')


    if r1.split()[-1] in d:
        # print('druga opcja')
        r3 = r1+' '+s1[0]
        d2 = DFS(r3, s1[1:])
    else : d2 = ''


    if len(d1) == 0 and len(d2) == 0 :
        # print("ERROR")
        return ''

    if len(d1) == 0 and len(d2) != 0 : return d2

    if len(d1) != 0 and len(d2) == 0 : return d1

    e1 = sum(map(lambda x: pow(len(x),2), d1.split()))
    e2 = sum(map(lambda x: pow(len(x),2), d2.split()))

    if e1 >= e2 : return d1
    return d2

# for di in d:
#     print('#',di, '#')

# print('ksiega in d: ', 'księga' in d)
# print('len(d)', len(d))

for line in file:
    ll = len(line)
    if ll == 0 : break
    print ("I am analyzing: \n", line)
    print("result: \n", DFS(line[0], line[1:]))


