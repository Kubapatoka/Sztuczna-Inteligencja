def npok(n,k):
    if k > n: return 0
    licz = 1
    mian = 1
    for i in range(k):
        licz = licz*(n-i)
        mian = mian * (i+1)
    return licz/mian

# Ilość różnych talii Blotkarza: to 16 po 5
r_tal_b = npok(36,5)
r_tal_f = npok(16,5)

wszystkie_mozliwe_rozdania = r_tal_b*r_tal_f

# policzymy na ile sposobow moze wygrac blotkarz

#Poker Królewski = niemożliwy do osiągnięcia przez nikogo

#Poker - może osiągnąć tylko Blotkarz, Figurant może mieć cokolwiek
poker_b = 5*4 #Możemy zacząć od 2,3,4,5,6 oraz 4 kolory do wyboru
poker = poker_b*r_tal_f

#Kareta - Blotkarz musi miec karete, Figurant nie moze jej miec
kar_b = 9 * (8*4) # Na 9 sposobow Kareta, na 32 pozostala jedna karta 
kar_f = 4 * (3*4) 

kareta = kar_b * (r_tal_f - kar_f)

#Full (3+2) - Blotkarz musi miec fulla, Figurant nie ma miec karety i fula
full_b = 9*npok(4,3)*8*npok(4,2)
full_f = 4*npok(4,3)*3*npok(4,2)

full = full_b*(r_tal_f-kar_f-full_f)

#Kolor - Blotkarz ma kolor, Figurant nie ma miec karety i fula
kol_b = npok(9,5)*4
kolor = kol_b * (r_tal_f - kar_f - full_f)

#Strit - Blotkarz ma strita (ale nie pokera), Figurant nie ma miec karety i fula
strit_b = 5*pow(4,5)-poker_b
strit = strit_b* (r_tal_f - kar_f - full_f)

#trójka
tr_b = 9*npok(4,3) * npok(8,2)*4*4
tr_f = 4*npok(4,3) * npok(3,2)*4*4
trojka = tr_b* (r_tal_f - kar_f - full_f - tr_f)

#dwie pary
dp_b = npok(9,2)*npok(4,2)*npok(4,2) *npok(7,1)*4
dp_f = npok(4,2)*npok(4,2)*npok(4,2)*npok(2,1)*4

dwie_pary = dp_b * (r_tal_f - kar_f - full_f - tr_f - dp_f)

#para - Figurant zawsze ma pare! - Dirichlet
#Wysoka karta zawsze przegrana

WygraneBlotkarza = poker + kareta + full + kolor + strit + trojka + dwie_pary
WygraneFiguranta = wszystkie_mozliwe_rozdania - WygraneBlotkarza

pstwo_b = WygraneBlotkarza/wszystkie_mozliwe_rozdania
pstwo_f = WygraneFiguranta/wszystkie_mozliwe_rozdania


print("Pstwo Wygranej Blotkarza: ", pstwo_b)

print("Pstwo Wygranej Figuranta: ", pstwo_f)

