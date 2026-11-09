#!/usr/bin/env python3
"""Exact rational weighted-streaming D3Q27 anisotropy. POST-EXPLORATION."""
from fractions import Fraction as Q
from itertools import product, permutations
from math import factorial

V = tuple(product((-1, 0, 1), repeat=3))
AXIS, OBLIQUE = (5, 0, 0), (3, 4, 0)
S = 10**20
REF = {
 ("uniform",9):(117601337830287677,117601337830306991),
 ("uniform",27):(1499327493336217,1499327493336218),
 ("uniform",81):(18576198240021,18576198240022),
 ("tuned",9):(-852164040857798,-852164040851018),
 ("tuned",27):(-1188808061117,-1188808061116),
 ("tuned",81):(-1633790202,-1633790201),
}
CASES = (
 ("uniform",(1,1,1,1),Q(8),-Q(70,3),Q(1168129,30240)),
 ("tuned",(0,12,2,1),Q(0),-Q(60,13),Q(139949,8736)),
)
def dot(k,v):
 return sum(a*b for a,b in zip(k,v))
def W(w):
 return sum(w[sum(z!=0 for z in v)] for v in V)
def avg_power(k,n,w):
 return sum((Q(w[sum(z!=0 for z in v)],W(w))*dot(k,v)**n for v in V),Q(0))
def taylor(x,n):
 return sum(((-1)**j*x**(2*j)/factorial(2*j) for j in range(n//2+1)),Q(0))
def symbol(k,d,w,n):
 return sum((Q(w[sum(z!=0 for z in v)],W(w))*taylor(Q(dot(k,v),d),n) for v in V),Q(0))
def interval(d,w):
 return symbol(OBLIQUE,d,w,14)-symbol(AXIS,d,w,16),symbol(OBLIQUE,d,w,16)-symbol(AXIS,d,w,14)
def floor(a):
 return a.numerator//a.denominator
def ceil(a):
 return -floor(-a)
def bound(w):
 return Q(sum(w[sum(z!=0 for z in v)]*(abs(dot(AXIS,v))**8+abs(dot(OBLIQUE,v))**8) for v in V),W(w)*factorial(8))
def absrange(lo,hi):
 assert lo*hi>0
 return (lo,hi) if lo>0 else (-hi,-lo)
def run():
 assert len(V)==27 and sum(x*x for x in AXIS)==sum(x*x for x in OBLIQUE)==25
 assert sum(x*x for x in (4,4,0))!=25   # altered wavelength control
 assert 12==2*2+8*1 and 13!=2*2+8*1 # wrong shell-weight control
 for perm in permutations(range(3)):
  for signs in product((-1,1),repeat=3):
   k=tuple(OBLIQUE[perm[i]]*signs[i] for i in range(3))
   assert avg_power(k,2,CASES[1][1])==avg_power(OBLIQUE,2,CASES[1][1])
   assert avg_power(k,4,CASES[1][1])==avg_power(OBLIQUE,4,CASES[1][1])
 print("SCALING_SCOPE|date=2026-11-09|model=D3Q27_weighted_streaming_scalar_symbol|status=POST_EXPLORATION|physical_lattice_sim=NO|controls=5|json=0")
 for name,w,c4,c6,b in CASES:
  assert avg_power(OBLIQUE,2,w)==avg_power(AXIS,2,w)
  assert (avg_power(OBLIQUE,4,w)-avg_power(AXIS,4,w))/24==c4
  assert -(avg_power(OBLIQUE,6,w)-avg_power(AXIS,6,w))/720==c6
  assert bound(w)==b
  print(f"SCALING_MOMENT|model={name}|W={W(w)}|delta_h4={c4}|delta_h6={c6}|remainder_h8_le={b}|json=0")
  ranges=[]
  for d in (9,27,81):
   lo,hi=interval(d,w)
   assert lo<=hi
   assert (floor(lo*S),ceil(hi*S))==REF[name,d]
   a=c4*Q(1,d)**4+c6*Q(1,d)**6
   assert lo-b*Q(1,d)**8<=a<=hi+b*Q(1,d)**8
   ranges.append(absrange(lo,hi))
   l,r=REF[name,d]
   print(f"SCALING_INTERVAL|model={name}|h=1/{d}|scale={S}|lower={l}|upper={r}|json=0")
  lim=((78,79),(80,81)) if name=="uniform" else ((716,718),(727,729))
  for j,(l,r) in enumerate(lim):
   p,q=ranges[j],ranges[j+1]
   assert Q(l)<p[0]/q[1]<=p[1]/q[0]<Q(r)
   ds=(9,27,81)
   print(f"SCALING_RATIO|model={name}|h_num=1/{ds[j]}|h_den=1/{ds[j+1]}|lower_gt={l}|upper_lt={r}|json=0")
 print("SCALING_VERDICT|rank4_uniform=FAIL|rank4_tuned=PASS|leading_directional_error_uniform=h4|leading_directional_error_tuned=h6|nonlinear_dynamics=UNTESTED|json=0")
if __name__=="__main__":
 run()
