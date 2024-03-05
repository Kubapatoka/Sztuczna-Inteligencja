# Opis rozwiązania:
# W kodzie shardkodowane są talie dla obu graczy
# losujemy 1000 rozdań
# obliczamy kto wygrał
# Obserwacja: jeśli Blotkarz uzyskał ten sam układ co Figurant to przegrał
# każdemu układowi przyporządkowuje liczbę (im wyższy układ tym większa liczba)
# Obserwacja: Poker Królewski jest nieosiągalny
# W Komentarzu na dole pozostawiam przykładowe wyniki jakie uzyskałem







# talia dla figuranta:

#Pik, Kier, Karo, Trefl
# P     K   C   T

import random

fig = ['AP', 'AK', 'AC', 'AT', 'KP', 'KK', 'KC', 'KT', 'QP', 'QK', 'QC', 'QT', 'WP', 'WK', 'WC', 'WT']
blot = []

# blot.append('2P')
# blot.append('2K')
# blot.append('2C')
# blot.append('2T')

# blot.append('3P')
# blot.append('3K')
# blot.append('3C')
# blot.append('3T')

# blot.append('4P')
# blot.append('4K')
# blot.append('4C')
# blot.append('4T')

# blot.append('5P')
# blot.append('5K')
# blot.append('5C')
# blot.append('5T')

# blot.append('6P')
# blot.append('6K')
# blot.append('6C')
# blot.append('6T')

# blot.append('7P')
# blot.append('7K')
# blot.append('7C')
# blot.append('7T')

blot.append('8P')
blot.append('8K')
blot.append('8C')
blot.append('8T')

blot.append('9P')
blot.append('9K')
blot.append('9C')
blot.append('9T')

blot.append('10P')
blot.append('10K')
blot.append('10C')
blot.append('10T')






def get_value(cart):
    if cart[0] == 'W':
        value = 11
    elif cart[0] == 'Q':
        value = 12
    elif cart[0] == 'K':
        value = 13
    elif cart[0] == 'A':
        value = 14
    elif cart[0] == '1':
        value = 10
    else:
        value = int(cart[0])
    return value


def first_bigger(fst_cart, snd_cart):
    return get_value(fst_cart) > get_value(snd_cart)


# bubble sort
def sort_hand(hand):
    for i in range(5):
        for j in range(5-i-1):
            if first_bigger(hand[j], hand[j+1]):
                hand[j], hand[j+1] = hand[j+1], hand[j]
    return hand

def draw_comp():
    b_hand = []
    f_hand = []
    lb = blot.copy()
    lf = fig.copy()
    for i in range(5):
        bx = random.randint(0, len(lb)-1)
        fx = random.randint(0, len(lf)-1)
        b_hand.append(lb[bx])
        f_hand.append(lf[fx])
        del lb[bx]
        del lf[fx]
    # print("Blotkarz: ", sort_hand(b_hand))
    # print("Figurant: ", sort_hand(f_hand))
    return compare(sort_hand(b_hand), sort_hand(f_hand))

# Wysoka Karta - 1
# Para - 2
# dwie pary - 3
# trójka - 4
# Strit - 5 (5 kart rosnąco)
# Kolor - 6 (5 kart w jednym kolorze)
# full - 7 (trójka i para)
# Kareta - 8 (4ka)
# Poker - 9 (strit i kolor)

def compare(b,f):
    wb = pkt(b)
    wf = pkt(f)
    if wb > wf: return False
    return True

def is_four(x):
    if get_value(x[0]) == get_value(x[3]) or get_value(x[1]) == get_value(x[4]): return True
    return False


def is_full(x):
    if get_value(x[0]) == get_value(x[2]) and get_value(x[3]) == get_value(x[4]) : return True
    if get_value(x[0]) == get_value(x[1]) and get_value(x[2]) == get_value(x[4]) : return True
    return False


def is_trojka(x):
    if get_value(x[0]) == get_value(x[2]) or get_value(x[1]) == get_value(x[3]) or get_value(x[2]) == get_value(x[4]): return True
    return False


def is_2_pary(x):
    if get_value(x[0]) == get_value(x[1]):
        if get_value(x[2]) == get_value(x[3]) or get_value(x[3]) == get_value(x[4]): return True
    if get_value(x[1]) == get_value(x[2]):
        if get_value(x[3]) == get_value(x[4]): return True
    return False

def is_para(x):
    if get_value(x[0]) == get_value(x[1]): return True
    if get_value(x[1]) == get_value(x[2]): return True
    if get_value(x[2]) == get_value(x[3]): return True
    if get_value(x[3]) == get_value(x[4]): return True
    return False



def pkt(x):
    is_color = True
    is_strit = True
    color = x[4][-1]
    for i in range(4):
        if get_value(x[i]) != (get_value(x[i+1]) + -1):
            is_strit = False
            break
    for i in range(4):
        if x[i][1] != color:
            is_color = False
            break

    if is_color and is_strit: return 9
    if is_four(x): return 8
    if is_full(x): return 7
    if is_color: return 6
    if is_strit: return 5
    if is_trojka(x): return 4
    if is_2_pary(x): return 3
    if is_para(x): return 2
    return 1



blotkarz_count = 0
figurant_count = 0

for _ in range(1000):
    if draw_comp():
        figurant_count+=1
    else:
        blotkarz_count+=1
print("Figurant: ", figurant_count, " Blotkarz: ", blotkarz_count, " Szansa: ", blotkarz_count/(blotkarz_count+figurant_count)*100, '%')


# Dla pełnej talii: 8 %

# Bez 2,3,4,5   : ok 30%

#Bez 2,3,4,5,6: ok 34%

# Bez 2,3,4,5 oraz same Piki i Kiery : 20%

#Piki od 6 do 10: 100%

# 8,9,10: 59%