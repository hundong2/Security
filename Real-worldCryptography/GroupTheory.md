# Group Theory(군론)란?
집합에 “연산(+) 같은 규칙”을 하나 붙였을 때, 그 연산이 잘 동작하는 구조를 연구하는 수학 분야입니다. 암호학에서는 DH/ECDH의 “공유 비밀이 같아지는” 성질이 군(특히 순환군) 성질 위에서 성립합니다.

## 예시 파일
./GroupTheory.md

## 답변
군론의 핵심은 **“원소들 + 연산 1개”**로 만든 구조가 얼마나 규칙적이고 예측 가능하게 동작하는지입니다.

### 1) 군(Group)의 정의(4가지 조건)
집합 \(G\)와 연산 \(\*\)가 있을 때 \((G, \*)\)가 **군**이려면:

1. **닫힘성(Closure)**: \(a,b \in G \Rightarrow a \* b \in G\)  
2. **결합법칙(Associativity)**: \((a \* b) \* c = a \* (b \* c)\)  
3. **항등원(Identity)**: 어떤 \(e \in G\)가 있어서 \(a \* e = e \* a = a\)  
4. **역원(Inverse)**: 모든 \(a \in G\)에 대해 \(a^{-1}\)가 존재해 \(a \* a^{-1} = a^{-1} \* a = e\)

> 직관: “계산하다가 집합 밖으로 튀지 않고(닫힘), 괄호 위치가 결과에 영향 없고(결합), 1처럼 아무 영향 없는 값이 있고(항등), 되돌릴 수 있다(역원).”

### 2) 가장 쉬운 예시
- 정수의 덧셈 \((\mathbb{Z}, +)\) 는 군  
  - 항등원: 0, 역원: \(-a\)
- \(\mathbb{Z}_n = \{0,1,\dots,n-1\}\)에서의 덧셈(mod n)도 군  
  - 항등원: 0, 역원: \(n-a\) (mod n)

### 3) “아벨 군(가환군)”이란?
추가로 **교환법칙** \(a \* b = b \* a\)까지 성립하면 **아벨 군**(commutative group)입니다.  
- \((\mathbb{Z}_n, +)\)는 아벨 군  
- 하지만 “행렬 곱”, “순열 합성”은 보통 **비가환**이라 아벨 군이 아닐 수 있습니다.

### 4) 암호학에서 왜 중요하나? (DH/ECDH 연결)
Diffie–Hellman류는 “한쪽 비밀을 먼저 적용하든, 반대쪽 비밀을 먼저 적용하든 결과가 같다”는 구조가 필요합니다. 이게 군 연산(정확히는 **스칼라 곱/거듭제곱**)의 성질로 보장됩니다.

- (고전 DH) 큰 소수 \(p\)에서  
  - 공개값: \(A = g^a \bmod p\), \(B = g^b \bmod p\)  
  - 공유비밀: \(B^a = (g^b)^a = g^{ab}\), \(A^b = (g^a)^b = g^{ab}\)
- (ECDH/X25519) “거듭제곱” 대신 “타원곡선 위 점의 스칼라 곱”을 쓰지만, 같은 형태의 성질을 이용합니다.

즉, 군론은 암호 프로토콜에서 **“계산 규칙이 안정적으로 성립하는 무대”**를 제공합니다.

---

아래 파일을 작업 폴더에 추가해, 정의/예시/암호 연결을 한 번에 읽을 수 있게 정리했습니다.

````markdown
# Group Theory(군론) 기초

## 1) 군(Group)의 정의
집합 G와 연산 *가 있을 때 (G, *)가 군이려면:

1. 닫힘성: a,b ∈ G ⇒ a*b ∈ G
2. 결합법칙: (a*b)*c = a*(b*c)
3. 항등원: e ∈ G가 존재해서 a*e = e*a = a
4. 역원: 모든 a ∈ G에 대해 a^{-1}가 존재해서 a*a^{-1} = e

## 2) 예시
- (Z, +): 정수 덧셈은 군 (항등원 0, 역원 -a)
- (Z_n, + mod n): {0..n-1}에서 mod n 덧셈도 군

## 3) 아벨 군(가환군)
a*b = b*a까지 성립하면 아벨 군.
- (Z_n, +)는 아벨 군
- 순열의 합성은 보통 비가환(아벨 아님)

## 4) 암호학에서의 의미
DH/ECDH는 “비밀을 서로 다른 순서로 적용해도 결과가 같아지는” 성질을 사용.
- DH: B^a = (g^b)^a = g^(ab), A^b = (g^a)^b = g^(ab)
- ECDH(X25519): 거듭제곱 대신 스칼라 곱을 사용하지만 아이디어는 동일

## 5) 아주 간단한 코드 예시(Python)
Z_n에서 덧셈이 군인지(역원/항등원) 체크하는 수준의 장난감 예시.

```python
def is_group_Zn_add(n: int) -> bool:
    G = list(range(n))

    # closure & associativity는 mod 덧셈에서 항상 성립하지만, 예시로 간단히 체크
    for a in G:
        for b in G:
            if (a + b) % n not in G:
                return False

    for a in G:
        for b in G:
            for c in G:
                if ((a + b) % n + c) % n != (a + (b + c) % n) % n:
                    return False

    e = 0
    for a in G:
        if (a + e) % n != a or (e + a) % n != a:
            return False

    for a in G:
        inv_found = False
        for x in G:
            if (a + x) % n == e and (x + a) % n == e:
                inv_found = True
                break
        if not inv_found:
            return False

    return True

print(is_group_Zn_add(2))   # True
print(is_group_Zn_add(10))  # True
```
````

### 추가 자료
- [Wikipedia: Group (mathematics)](https://en.wikipedia.org/wiki/Group_(mathematics))
- [MIT OCW: Abstract Algebra(군/환 기초 강의)](https://ocw.mit.edu/courses/18-703-modern-algebra-spring-2013/)
- [RFC 7748: X25519/X448(현대 ECDH)](https://www.rfc-editor.org/rfc/rfc7748)