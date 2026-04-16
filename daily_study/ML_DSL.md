# ML-DSA (Module-Lattice-Based Digital Signature Algorithm)

> 기준: NIST FIPS 204 최종본 (CRYSTALS-Dilithium 기반)  
> 구성: 정의 → 수학적 바탕 → 알고리즘 → 정확성 증명 → 보안 증명 스케치 → 구현 포인트

---

## 1. ML-DSA가 무엇인가

ML-DSA는 **포스트양자 전자서명 표준**입니다.  
고전적 전자서명인 RSA/ECDSA가 정수분해나 이산로그의 어려움에 기대는 반면, ML-DSA는 **모듈 격자(module lattice)** 위의 MLWE(Module-LWE) 및 MSIS(Module-SIS) 류의 문제의 어려움에 기대어 서명과 검증을 수행합니다.

표준은 세 파라미터 집합을 제공합니다:

| 파라미터 집합 | NIST 카테고리 | 공개키 크기 | 서명 크기 | 개인키 크기 |
|---|---|---|---|---|
| ML-DSA-44 | 2 | 1312 B | 2420 B | 2560 B |
| ML-DSA-65 | 3 | 1952 B | 3309 B | 4032 B |
| ML-DSA-87 | 5 | 2592 B | 4627 B | 4896 B |

설계 관점에서 보면 ML-DSA는 **Fiat–Shamir with Aborts** 계열 서명입니다. 핵심 흐름은 다음과 같습니다:

1. 서명자는 짧은 비밀 `s₁, s₂`로부터 공개값 `t = A s₁ + s₂`를 만든다.
2. 서명 시 임시 마스크 `y`를 뽑아 `w = Ay`를 계산한다.
3. `w`의 "상위 비트"만 해시해 챌린지 `c`를 만든다.
4. 응답 `z = y + c s₁`와 힌트 `h`를 내보낸다.
5. 검증자는 공개키에 압축된 `t₁`과 힌트 `h`를 이용해 서명자가 봤던 `w₁`을 복원하고, 같은 챌린지 해시가 나오는지 확인한다.

---

## 2. 기초 수학: 격자, 환, 모듈

ML-DSA를 이해하려면 **"격자 문제를 환 위에서 벡터/행렬 형태로 다룬다"**는 그림을 잡아야 합니다.

기본 환:

$$R_q = \mathbb{Z}_q[X]/(X^{256}+1)$$

여기서 `q = 8380417`입니다.

"module lattice"란 **스칼라가 정수가 아니라 $R_q$ 원소인 선형대수**를 한다는 뜻입니다. 즉 $A \in R_q^{k \times \ell}$, $s_1 \in R_q^\ell$, $s_2 \in R_q^k$ 처럼 다항식 벡터/행렬을 다룹니다.

각 다항식은 길이 256의 계수 벡터로 펼칠 수 있으므로, 다항식 `a`에 대한 "곱하기 `a`"는 256×256 negacyclic circulant matrix $\mathrm{rot}(a)$로 표현됩니다:

$$t = A s_1 + s_2 \quad \Leftrightarrow \quad \mathrm{vec}(t) = \mathrm{rot}(A)\,\mathrm{vec}(s_1) + \mathrm{vec}(s_2)$$

---

## 3. 왜 이 환에서 NTT가 되나: 핵심 대수 구조

ML-DSA의 효율은 **NTT(Number Theoretic Transform)**에 크게 의존합니다.

표준 파라미터에서 `q = 8380417`이고, $X^{256}+1$은 $R_q$ 위에서 256개의 일차 인수로 완전히 분해됩니다:

$$R_q \cong \prod_{i=0}^{255} \mathbb{Z}_q$$

NTT의 핵심:

$$\mathrm{NTT}(ab)_i = (ab)(\zeta_i) = a(\zeta_i) \cdot b(\zeta_i) = \mathrm{NTT}(a)_i \cdot \mathrm{NTT}(b)_i$$

따라서:

$$\mathrm{NTT}(ab) = \mathrm{NTT}(a) \circ \mathrm{NTT}(b)$$

이를 통해 다항식 곱셈을 $O(n^2)$ 대신 $O(n \log n)$에 수행할 수 있습니다.

---

## 4. MLWE, MSIS, SelfTargetMSIS

### MLWE (Module-LWE)

"무작위처럼 보이는 선형식 뒤에 짧은 비밀과 짧은 잡음이 숨어 있느냐"를 구별하는 문제입니다.

- $(A,\; As_1+s_2)$ 와 $(A,\; t_{\text{uniform}})$ 의 구별 이득으로 MLWE를 정의
- ML-DSA 공개키의 핵심인 `t = A s₁ + s₂`가 정확히 이 형태
- 공격자가 공개키로부터 비밀을 회수하려면 이 문제를 깨야 함

### MSIS (Module-SIS)

"주어진 모듈 행렬에 대해 짧은 비자명 영벡터 관계를 찾는 문제"입니다.

- $A z = 0 \pmod{q}$를 만족하는 짧은 $z \neq 0$를 찾는 문제
- coefficient embedding을 쓰면 표준 SIS가 됨
- 결국 "특정 $q$-ary lattice에서 짧은 벡터를 찾는 문제"

### SelfTargetMSIS

Fiat–Shamir화된 서명의 비상호작용 구조를 캡처한 문제입니다.

- 해시 $H$까지 포함해서, 어떤 $\mu$와 짧은 $y = [r;\,c]$를 찾아 $H(\mu \| [I \mid A]y) = c$를 만족시키는 문제

---

## 5. 파라미터들의 의미

세 파라미터 집합 모두 `n = 256`, `q = 8380417`, `d = 13`을 공유합니다.

| 파라미터 | ML-DSA-44 | ML-DSA-65 | ML-DSA-87 |
|---|---|---|---|
| `k` (공개키 행 수) | 4 | 6 | 8 |
| `ℓ` (비밀키 열 수) | 4 | 5 | 7 |
| `η` (비밀 계수 범위) | 2 | 4 | 2 |
| `τ` (챌린지 해밍중량) | 39 | 49 | 60 |
| `γ₁` (마스크 범위) | 2¹⁷ | 2¹⁹ | 2¹⁹ |
| `γ₂` (Low bits 경계) | (q-1)/88 | (q-1)/32 | (q-1)/32 |
| `ω` (힌트 최대 밀도) | 80 | 55 | 75 |

**τ (챌린지 희소도)의 역할:**

챌린지 `c`는 정확히 `τ`개의 비영 계수를 가지며 각 비영 계수는 `±1`입니다.  
`c s₁`의 임의 계수는 "최대 `τ`개의 `±η` 계수의 합"이므로 절댓값이 최대 `τ·η`입니다.  
이 간단한 상계가 정확성 증명과 거부 샘플링 조건을 떠받칩니다.

---

## 6. 알고리즘

### 6.1 KeyGen

```
입력: 32바이트 시드 ξ
출력: (pk, sk)

1. (ρ, ρ', K) = H(ξ)
2. A = ExpandA(ρ)          # 공개 행렬 생성
3. (s₁, s₂) = ExpandS(ρ') # 짧은 비밀 벡터 생성
4. t = A s₁ + s₂
5. (t₁, t₀) = Power2Round(t)
6. pk = (ρ, t₁)
7. tr = H(pk, 64)
8. sk = (ρ, K, tr, s₁, s₂, t₀)
```

### 6.2 Sign

```
입력: sk, 메시지 M, context ctx
출력: 서명 σ = (c̃, z, h)

외부:
1. M' = BytesToBits(len(ctx)) ‖ ctx ‖ M
2. rnd = Random(32 bytes)   # hedged variant

내부 (반복):
3. μ = H(BytesToBits(tr) ‖ M', 64)
4. ρ'' = H(K ‖ rnd ‖ μ, 64)
5. y = ExpandMask(ρ'')      # 마스크 샘플링
6. w = Ay
7. w₁ = HighBits(w)
8. c̃ = H(μ ‖ w1Encode(w₁), λ/4)
9. c = SampleInBall(c̃)
10. z = y + c s₁
11. r₀ = LowBits(w - c s₂)
12. if ‖z‖∞ ≥ γ₁ - β or ‖r₀‖∞ ≥ γ₂ - β: 재시도
13. h = MakeHint(-c t₀, w - c s₂ + c t₀)
14. if ‖h‖₁ > ω: 재시도
15. return σ = (c̃, z, h)
```

### 6.3 Verify

```
입력: pk = (ρ, t₁), 메시지 M, 서명 σ = (c̃, z, h)
출력: Accept / Reject

1. A = ExpandA(ρ)
2. tr = H(pk, 64)
3. μ = H(BytesToBits(tr) ‖ M', 64)
4. c = SampleInBall(c̃)
5. w' = A z - c t₁ 2^d
6. w₁' = UseHint(h, w')
7. c̃' = H(μ ‖ w1Encode(w₁'), λ/4)
8. Accept iff:
   - ‖z‖∞ < γ₁ - β
   - ‖h‖₁ ≤ ω
   - c̃' = c̃
```

---

## 7. Power2Round, Decompose, Hint가 왜 필요한가

공개키가 `t` 전체를 담으면 크기가 커집니다. 그래서 ML-DSA는 `t`를:

$$t = t_1 \cdot 2^d + t_0 \quad (d = 13)$$

으로 분해해 `t₁`만 공개키에 담고 `t₀`는 개인키에 남깁니다.

**Power2Round vs Decompose의 차이:**

- **Power2Round**: 말 그대로 비트 수준의 상·하위 분해. 공개키 압축에 사용.
- **Decompose**: `α = 2γ₂` 단위로 분해하되, `q/2` 근처와 0 근처의 랩어라운드 때문에 특별한 예외 처리를 포함. 힌트 구조에 사용.

Decompose의 특수 처리는 "아주 작은 오차가 상위 비트를 크게 바꾸는" 병목을 피하는 장치입니다.

---

## 8. 힌트 보조정리의 핵심 아이디어와 간단한 증명

**명제:** `‖z‖∞ ≤ γ₂`이면

$$\mathrm{UseHint}(\mathrm{MakeHint}(z, r),\; r) = \mathrm{HighBits}(r + z)$$

**증명 스케치:**

Decompose(r)가 다음을 만족한다고 하자:
$$r \equiv r_1 \alpha + r_0 \pmod{q}, \qquad r_0 \in \left(-\frac{\alpha}{2}, \frac{\alpha}{2}\right]$$

`r`에서 `r + z`로 이동할 때 분해 구간의 경계를 최대 한 칸만 넘을 수 있습니다.

- **경계를 넘지 않으면:** high bits는 안 바뀌므로 힌트는 0이고 UseHint는 `r₁`을 그대로 반환.
- **경계를 넘으면:** high bits는 `±1`만큼만 바뀜. MakeHint는 "바뀌었다"는 사실을 기록하고, UseHint는 `z`가 양수/음수인지로 어느 방향으로 넘어갔는지를 판정해 `r₁ + 1` 또는 `r₁ - 1`을 반환.

Decompose의 특수 처리 (`r₀ = q/2` 근처)는 랩어라운드 근처에서 "한 칸 이동"이라는 해석이 깨지지 않게 만드는 장치입니다. 이것이 **힌트가 1비트씩만으로도 충분한 이유**입니다.

---

## 9. 정확성(Correctness) 증명

검증이 왜 항상 통과하는지 보겠습니다.

키생성에서:
$$t = A s_1 + s_2 = t_1 \cdot 2^d + t_0$$

검증자가 계산하는 `w'`:
$$w' = A z - c t_1 2^d$$

이를 전개하면:
$$\begin{aligned}
w' &= A(y + cs_1) - c(t - t_0) \\
   &= Ay + cAs_1 - cAs_1 - cs_2 + ct_0 \\
   &= w - cs_2 + ct_0
\end{aligned}$$

서명자는:
$$h = \mathrm{MakeHint}(-ct_0,\; w - cs_2 + ct_0)$$

을 만족하도록 했으므로:
$$\mathrm{UseHint}(h,\; w - cs_2 + ct_0) = \mathrm{HighBits}(w - cs_2)$$

거부 조건 `‖r₀‖∞ < γ₂ - β`를 만족하는 서명만 출력하므로, `w - cs₂`의 low bits는 반올림 경계에서 충분히 떨어져 있습니다. 따라서:

$$\mathrm{HighBits}(w - cs_2) = \mathrm{HighBits}(w) = w_1$$

결론:
$$w_1' = \mathrm{UseHint}(h, w') = w_1$$
$$H(\mu \| \mathrm{w1Encode}(w_1')) = H(\mu \| \mathrm{w1Encode}(w_1)) = \tilde{c} \quad \checkmark$$

---

## 10. 왜 거부 샘플링(Rejection Sampling)이 필요한가

Fiat–Shamir with Aborts의 본질은 **"응답 `z`가 비밀 `s₁`을 드러내지 않게 만들기"**입니다.

`y`를 그냥 균등하게 뽑고 항상 `z = y + cs₁`을 내보내면, `z`의 분포는 `cs₁`만큼 이동된 분포라서 비밀에 대한 정보가 남습니다.

그래서 ML-DSA는 `‖z‖∞ < γ₁ - β` 같은 조건을 통과한 경우만 출력합니다. 그러면 허용된 범위 안에서는 `(z, c)`가 사실상 비밀과 무관하게 균등하게 나옵니다.

**요약:** Rejection sampling은 "서명 응답이 비밀에 의해 살짝 이동된 분포"를 "검증 가능한 범위에서 거의 비밀과 무관한 분포"로 교정하는 장치입니다. 이것이 lattice signature에서 aborts가 들어가는 이유입니다.

---

## 11. 보안 증명 스케치

보안 목표는 **SUF-CMA** (Strong Unforgeability under Chosen Message Attack)입니다.

QROM(Quantum Random Oracle Model)에서의 구체적 경계:

$$\mathrm{Adv}^{\mathrm{SUF\text{-}CMA}}_{\mathrm{Dilithium}}(A)
\le
\mathrm{Adv}_{\mathrm{MLWE}}(B) + \mathrm{Adv}_{\mathrm{SelfTargetMSIS}}(C) + \mathrm{Adv}_{\mathrm{MSIS}}(D) + 2^{-254}$$

### 11.1 새 메시지 위조 → SelfTargetMSIS

유효한 위조 서명 $(\tilde{c}, z, h)$가 있으면 검증 조건에 의해:

$$2\gamma_2 \cdot \mathrm{UseHint}(h, Az - ct_1 2^d) = Az - ct_1 2^d + u$$

이를 정리하면:

$$Az - ct + u' = H'(\mu \| [A \mid t \mid I][z; c; u']) = c$$

여기서 $\|u'\|_\infty \le 2\gamma_2 + 1 + \tau 2^{d-1}$. 이것이 바로 SelfTargetMSIS 인스턴스입니다.

### 11.2 같은 메시지의 다른 서명 위조 → MSIS

강한 위조성: 공격자가 같은 메시지에 대해 다른 $(z', h', c)$를 내놓을 때, 두 유효한 서명이 같은 복원값을 주면 힌트의 유일성과 bound를 이용해 결국 짧은 영벡터 관계를 도출:

$$Az + u = 0 \pmod{q}$$

이것이 MSIS 인스턴스입니다.

### 11.3 왜 QROM 얘기가 나오나

ML-DSA는 Fiat–Shamir 변환을 사용합니다. 양자 공격자는 해시 오라클을 **중첩 상태(superposition)**로 질의할 수 있으므로 고전 ROM의 forking lemma가 곧장 통하지 않습니다.

Dilithium의 분석은 QROM을 고려하고, Kiltz–Lyubashevsky–Schaffner 계열 결과를 배경으로 설명됩니다. 특히 **deterministic 서명**은 UF-NMA와 UF-CMA 사이의 더 타이트한 연결을 제공한다는 점이 강조됩니다.

---

## 12. 표준 사용 관점에서 알아둘 점

### 순수형 vs HashML-DSA

| 구분 | 설명 |
|---|---|
| **ML-DSA** (순수형) | 표준이 일반적으로 권장 |
| **HashML-DSA** (pre-hash) | 큰 메시지에서 해시 성능 제약이 있을 때 사용 |

### Context String

- 최대 255바이트
- 실제 서명 입력 `M'` 구성에 들어감
- 검증 시에도 동일한 context를 사용해야 함

### Hedged Variant

기본 서명에는 hedged variant가 쓰입니다:

```
rnd = Random(32 bytes)
ρ'' = H(K ‖ rnd ‖ μ, 64)
```

이를 통해 nonce 재사용 취약점을 방지합니다.

### 구현 시 주의사항

1. **부동소수점 사용 금지**: NIST는 ML-DSA 구현에서 부동소수점 사용을 금지. 반올림 오차가 correctness를 깨뜨릴 수 있음.
2. **이산 가우시안 샘플링 없음**: Dilithium/ML-DSA는 애초에 이산 가우시안 샘플링을 피해 상수시간 구현 친화적으로 설계됨.
3. **성능의 핵심**: SHAKE 확장과 NTT가 전체 실행 시간의 대부분을 차지.

---

## 13. 핵심 요약

| 구성 요소 | 역할 |
|---|---|
| `t = A s₁ + s₂` | 공개키의 MLWE 구조 |
| `Power2Round(t)` | 공개키 압축 (`t₁` 공개, `t₀` 비밀) |
| `y` (마스크) | 비밀 은폐용 임시 랜덤 벡터 |
| `c` (챌린지) | 해밍중량 `τ`인 희소 다항식 |
| `z = y + cs₁` | 서명 응답 |
| `h` (힌트) | 검증자가 `w₁`을 복원하기 위한 보조 정보 |
| Rejection Sampling | 비밀 누출 방지 |
| SUF-CMA | MLWE + SelfTargetMSIS + MSIS로 환원 |

정확성의 핵심 등식:

$$Az - ct_1 2^d = w - cs_2 + ct_0$$

이로부터 $w_1' = w_1$이 성립하고 검증이 통과합니다.

---

## 다음 단계

Algorithm 6, 7, 8 (KeyGen, Sign, Verify)을 줄 단위로 주석 달아 완전히 해부하는 방식으로 이어서 공부하는 것이 가장 도움이 됩니다.

---

## 참고 자료

- [NIST FIPS 204](https://csrc.nist.gov/pubs/fips/204/final) - ML-DSA 최종 표준
- [CRYSTALS-Dilithium](https://pq-crystals.org/dilithium/) - 원본 알고리즘 문서
